use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures_core::{Future, Stream};

#[async_trait]
pub trait PaginationDelegate {
    type Item;
    type Error;

    /// Performs an asynchronous request for the next page and returns either
    /// a vector of the result items or an error.
    async fn next_page(&mut self) -> Result<Vec<Self::Item>, Self::Error>;

    /// Gets the current offset, which will be the index at the end of the
    /// current/previous page. The value returned from this will be changed by
    /// [`PaginatedStream`] immediately following a successful call to
    /// [`next_page()`], increasing by the number of items returned.
    fn offset(&self) -> usize;

    /// Sets the offset for the next page. The offset is required to be the
    /// index of the last item from the previous page.
    fn set_offset(&mut self, value: usize);

    /// Gets the total count of items that are currently expected from the API.
    /// This may change if the API returns a different number of results on
    /// subsequent pages, and may be less than what the API claims in its
    /// response data if the API has a maximum limit.
    fn total_items(&self) -> Option<usize>;
}

pub enum PaginatedStream<'f, D: PaginationDelegate> {
    Request {
        delegate: D,
    },
    Pending {
        #[allow(clippy::type_complexity)]
        future: Pin<Box<dyn Future<Output = Result<(D, Vec<D::Item>), D::Error>> + 'f>>,
    },
    Ready {
        delegate: D,
        items: VecDeque<D::Item>,
    },
    Closed,
    Indeterminate,
}

impl<'f, D> From<D> for PaginatedStream<'f, D>
    where
        D: PaginationDelegate,
{
    fn from(other: D) -> PaginatedStream<'f, D> {
        PaginatedStream::Request { delegate: other }
    }
}

impl<'f, D> Stream for PaginatedStream<'f, D>
    where
        D: PaginationDelegate + Unpin + 'f,
        D::Item: Unpin,
{
    // If the state is `Pending` and the future resolves to an `Err`, that error is
    // forwarded only once and the state set to `Closed`. If there is at least one
    // result to return, the `Ok` variant is, of course, used instead.
    type Item = Result<D::Item, D::Error>;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Avoid using the full namespace to match all variants.
        use PaginatedStream::*;

        // Take ownership of the current state (`self`) and replace it with the
        // `Indeterminate` state until the new state is in fact determined.
        let this = std::mem::replace(&mut *self, Indeterminate);

        match this {
            // This state occurs at the entry of the state machine and when there was a poll when
            // the state was `Ready` but had no items to yield. It only holds the
            // `PaginationDelegate` that will be used to update the offset and make new requests.
            Request { mut delegate } => {
                self.set(Pending {
                    /// Construct a new future and pin it on the heap.
                    future: Box::pin(async {
                        // Request the next page from the delegate and await the result.
                        let result = delegate.next_page().await;
                        // Map the `Ok` value of the result to a tuple that includes the delegate
                        // that was moved into this block.
                        result.map(|items| (delegate, items))
                    }),
                });

                // Return the distilled verson of the new state to the callee, indicating that a
                // new request has been made and we are waiting or new data.
                Poll::Pending
            }
            // At some point in the past this stream was polled and asked the delegate to make a new
            // request. Now it is time to poll the future returned from that request, and if results
            // are available, unpack them to the `Ready` state and move the delegate. If the future
            // still doesn't have results, set the state back to `Pending` and move the fields back
            // into position.
            Pending { mut future } => match future.as_mut().poll(ctx) {
                // The future from the last request returned successfully with new items,
                // and gave the delegate back.
                Poll::Ready(Ok((mut delegate, items))) => {
                    // Tell the delegate the offset for the next page, which is the sum of the
                    // old offset and the number of items that the API sent back.
                    delegate.set_offset(delegate.offset() + items.len());
                    // Construct a new `VecDeque` so that the items can be popped from the front.
                    // This should be more efficient than reversing the `Vec`, and less confusing.
                    let mut items = VecDeque::from(items);
                    // Get the first item out so that it can be yielded. The event that there are no
                    // more items should have been handled by the `Ready` branch, so it should be
                    // safe to unwrap.
                    let popped = items.pop_front().unwrap();

                    // Set the new state to `Ready` with the delegate and the items.
                    self.set(Ready { delegate, items });

                    Poll::Ready(Some(Ok(popped)))
                }
                // The future from the last request returned with an error.
                Poll::Ready(Err(error)) => {
                    // Set the state to `Closed` so that any future polls will return
                    // `Poll::Ready(None)`. The callee can even match against this if needed.
                    self.set(Closed);

                    // Forward the error to whoever polled. This will only happen once because the
                    // error is moved, and the state set to `Closed`.
                    Poll::Ready(Some(Err(error)))
                }
                // The future from the last request is still pending.
                Poll::Pending => {
                    // Because the state is currently `Indeterminate` it must be set back to what it
                    // was. This will move the future back into the state.
                    self.set(Pending { future });

                    // Tell the callee that we are still waiting for a response.
                    Poll::Pending
                }
            },
            // The request has resolved with data in the past, and there are items ready for us to
            // provide the callee. In the event that there are no more items in the `VecDeque`, we
            // will make the next request and construct the state for `Pending` again.
            Ready {
                delegate,
                mut items,
            } => match items.pop_front() {
                // There is at leats one item in the buffer, so yield it.
                Some(item) => Poll::Ready(Some(Ok(item))),
                // There was no item to yield.
                None => {
                    // Check if we have met or exceeded the number of items expected to be yielded.
                    // Unwrapping `delegate.total_items()` should be safe because it would be
                    // impossible to be in the `Ready` state if we have not recieved data from the
                    // API yet, which is the only situation in which the value here would be `None`.
                    if delegate.offset() >= delegate.total_items().unwrap_or(usize::MAX) {
                        // All of the items that API is willing to send have been yielded, so set
                        // the stream to `Closed` so that any further polls will yield
                        // `Poll::Ready(None)`.
                        self.set(Closed);
                        Poll::Ready(None)
                    } else {
                        // Set the state back to `Request` so that the next poll will make a request
                        // for the next page. The offset should have already been updated at a
                        // previous state.
                        self.set(Request { delegate });
                        // Poll again to make the request and forward the `Poll::Pending`.
                        self.poll_next(ctx)
                    }
                }
            },
            // Either an error has occurred, or the last item has been yielded already. Nobody
            // should be polling anymore, but to be nice, just tell them that there are no more
            // results with `Poll::Ready(None)`.
            Closed => Poll::Ready(None),
            // The `Indeterminate` state should have only been used internally and reset back to a
            // valid state before yielding the `Poll` to the callee. This branch should never be
            // reached, if it is, that is a panic.
            Indeterminate => unreachable!(),
        }
    }
}
