//! This module helps you implement pagination over a web API endpoint.
//!
//! The only thing that needs to happen for your functions to return
//! asynchronous paginators (via the [`Stream`] trait) is the implementation of
//! the [`PaginationDelegate`] trait. See the documentation of the methods on
//! that trait to see what they should do.

use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures_core::{Future, Stream};

/// This is the trait that needs to be implemented in order to tell the
/// [`PaginatedStream`] how to keep track of the current page and make requests
/// to the API.
///
/// The indices of the pages are handled automatically, simply
/// implement `offset` and `set_offset` correctly to ensure that any internal
/// fields are being updated.
///
/// After creating implementing this on a type, use `PaginatedStream::from` to
/// get an iterable stream from the delegate.
#[async_trait]
pub trait PaginationDelegate {
    /// This is the type of the item that calls to `poll_next` are expected to
    /// yield.
    type Item;
    /// This is the type error that will occur when a future from
    /// `PaginationDelegate::next_page` resolves to an error.
    type Error;

    /// Performs an asynchronous request for the next page and returns either
    /// a vector of the result items or an error. Implementing this may require
    /// the [`macro@async_trait`] macro from the [mod@async_trait] crate.
    async fn next_page(&mut self) -> Result<Vec<Self::Item>, Self::Error>;

    /// Gets the current offset, which will be the index at the end of the
    /// current/previous page. The value returned from this will be changed by
    /// [`PaginatedStream`] immediately following a successful call to
    /// [`Self::next_page`], increasing by the number of items returned.
    fn offset(&self) -> usize;

    /// Sets the offset for the next page. The offset is required to be the
    /// index of the last item from the previous page.
    fn set_offset(&mut self, value: usize);

    /// Gets the total count of items that are currently expected from the API.
    /// This may change if the API returns a different number of results on
    /// subsequent pages, and may be less than what the API claims in its
    /// response data if the API has a maximum limit and stops providing results
    /// after a certain amount.
    fn total_items(&self) -> Option<usize>;
}

/// Resolution type of the future from [`PaginatedStream::Pending`] and the
/// inner value of [`PaginatedStream::Ready`].
pub struct ReadyStateValue<D>
where
    D: PaginationDelegate,
{
    delegate: D,
    items: VecDeque<D::Item>,
}

/// The future will be the result returned from the
/// [`PaginationDelegate::next_page`], and will either resolve to an `Err` with
/// `<D as PaginationDelegate>::Error` or a [`PendingFutureOutput`] with the
/// delegate and response items.
pub type PendingStateFuture<'f, D> =
    dyn Future<Output = Result<ReadyStateValue<D>, <D as PaginationDelegate>::Error>> + 'f;

/// This enumerable holds the current state of the paginated stream and also
/// implements the [`Stream`] trait itself. It is highly recommended to read the
/// source code of the `Stream` implementation for more documentation about how
/// the state is changed as the stream is polled, there is a liberal amount of
/// commentary.
pub enum PaginatedStream<'f, D: PaginationDelegate> {
    /// This is the entry-point, or rather where the state machine begins.
    /// This is also used to indicate that the state machine is ready for the
    /// next page from the API. This will be set when the state was previously
    /// `Ready` but had no more items to yield.
    Request(D),
    /// At some point in the past, the delegate was requested to fetch the next
    /// page and has returned a future. This will be polled whenever `poll_next`
    /// is called, eventually resulting in the state changing to `Ready` if
    /// successful, or `Closed` if an error was yielded.
    Pending(Pin<Box<PendingStateFuture<'f, D>>>),
    /// The next page is ready and its current items have been taken and are
    /// currently being yielded to whatever is polling the stream. This state
    /// will remain the same until it runs out of items, and on the very next
    /// poll, the state will change back to `Request` if there is another page,
    /// or `Closed` if the expected number of results has already been yielded.
    Ready(ReadyStateValue<D>),
    /// Either an error has occurred or the API has been exhausted of the items
    /// that it is willing to provide. Polling the stream when this is the state
    /// will always yield `Poll::Ready(None)`, and will never change once this
    /// has been set.
    Closed,
    /// This state is used internally when the result of `poll_next` is being
    /// resolved. If you are matching variants directly, always resolve this
    /// to [`unimplemented!()`].
    Indeterminate,
}

impl<'f, D> From<D> for PaginatedStream<'f, D>
where
    D: PaginationDelegate,
{
    fn from(other: D) -> PaginatedStream<'f, D> {
        PaginatedStream::Request(other)
    }
}

impl<'f, D> Stream for PaginatedStream<'f, D>
where
    D: 'f + PaginationDelegate + Unpin,
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
            Request(mut delegate) => {
                self.set(Pending(Box::pin(async {
                    // Request the next page from the delegate and await the result.
                    let result = delegate.next_page().await;
                    // Map the `Ok` value of the result to a tuple that includes the delegate
                    // that was moved into this block.
                    result.map(|items| ReadyStateValue {
                        delegate,
                        items: items.into_iter().collect(),
                    })
                })));

                // Reawaken the context so that the executor doesn't ignore the future.
                ctx.waker().wake_by_ref();

                // Return the distilled version of the new state to the callee, indicating that
                // a new request has been made, and we are waiting for new data.
                Poll::Pending
            }
            // At some point in the past this stream was polled and asked the delegate to make a new
            // request. Now it is time to poll the future returned from that request, and if results
            // are available, unpack them to the `Ready` state and move the delegate. If the future
            // still doesn't have results, set the state back to `Pending` and move the fields back
            // into position.
            Pending(mut future) => match future.as_mut().poll(ctx) {
                // The future from the last request returned successfully with new items,
                // and gave the delegate back.
                Poll::Ready(Ok(ReadyStateValue {
                    mut delegate,
                    mut items,
                })) => {
                    // Tell the delegate the offset for the next page, which is the sum of the
                    // old offset and the number of items that the API sent back.
                    delegate.set_offset(delegate.offset() + items.len());
                    // Get the first item out so that it can be yielded. The event that there are no
                    // more items should have been handled by the `Ready` branch, so it should be
                    // safe to unwrap.
                    let popped = items.pop_front().unwrap();

                    // Set the new state to `Ready` with the delegate and the items.
                    self.set(Ready(ReadyStateValue { delegate, items }));

                    // Note that this could have been `self.poll_next(ctx)` rather than popping the
                    // item in this branch, but doing everything here is better than moving the
                    // fields twice and doing unnecessary checks.
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
                    self.set(Pending(future));

                    // Tell the callee that we are still waiting for a response.
                    Poll::Pending
                }
            },
            // The request has resolved with data in the past, and there are items ready for us to
            // provide the callee. In the event that there are no more items in the `VecDeque`, we
            // will make the next request and construct the state for `Pending` again.
            Ready(ReadyStateValue {
                delegate,
                mut items,
            }) => match items.pop_front() {
                // There is at least one item in the buffer, so yield it.
                Some(item) => {
                    // Set the state back to `Ready`, even if the items buffer is empty. This allows
                    // the next page request to be made lazily, only after the current page is
                    // exhausted, and then the stream is polled again.
                    self.set(Ready(ReadyStateValue { delegate, items }));
                    Poll::Ready(Some(Ok(item)))
                }
                // There was no item to yield.
                None => {
                    // Check if we have met or exceeded the number of items expected to be yielded.
                    // Unwrapping `delegate.total_items()` should be safe because it would be
                    // impossible to be in the `Ready` state if we have not received data from the
                    // API yet, which is the only situation in which the value here would be `None`.
                    if delegate.offset() >= delegate.total_items().unwrap_or(usize::MAX) {
                        // All the items that API is willing to send have been yielded, so set
                        // the stream to `Closed` so that any further polls will yield
                        // `Poll::Ready(None)`.
                        self.set(Closed);
                        Poll::Ready(None)
                    } else {
                        // Set the state back to `Request` so that the next poll will make a request
                        // for the next page. The offset should have already been updated at a
                        // previous state.
                        self.set(Request(delegate));
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

    /// Currently, it is only possible to get the upper bound from the `Request`
    /// and `Ready` state. If no request has been made yet, the delegate
    /// can't know the expected number of items and will therefore return
    /// `None`. It should be possible to get a value when the state is
    /// `Pending`, but unfortunately the delegate is locked behind the stack
    /// frame of the pinned `Future`.
    fn size_hint(&self) -> (usize, Option<usize>) {
        use PaginatedStream::*;

        match self {
            Request(delegate) | Ready(ReadyStateValue { delegate, .. }) => {
                (0, delegate.total_items())
            }
            _ => (0, None),
        }
    }
}
