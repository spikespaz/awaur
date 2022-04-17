use async_trait::async_trait;
use awaur::paginator::{PaginatedStream, PaginationDelegate};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const GITHUB_API_BASE: &str = "https://api.github.com/";

pub struct Client {
    inner: surf::Client,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to make a request: {0:?}")]
    Request(surf::Error),
    #[error("failed to deserialize response data from '{url}': {error:?}")]
    Deserialize {
        error: serde_json::Error,
        url: surf::Url,
        bytes: Vec<u8>,
    },
}

// It would seem that the `http-types` crate is very silly and doesn't implement
// `Error` for their error types. Who needs standards anyway?
impl From<surf::Error> for Error {
    fn from(other: surf::Error) -> Self {
        Self::Request(other)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl Client {
    pub fn new<U>(base: U, token: Option<String>) -> surf::Result<Self>
    where
        U: AsRef<str>,
    {
        let mut config = surf::Config::new();

        config = config.set_base_url(surf::Url::parse(base.as_ref())?);
        config = config.add_header("Accept", "application/vnd.github.v3+json")?;

        if let Some(token) = token {
            config = config.add_header("Authorization", &format!("token {}", token))?;
        }

        Ok(Self {
            inner: config.try_into()?,
        })
    }

    pub async fn search_issues(&self, params: &IssueSearchParams) -> Result<IssueSearchResponse> {
        let mut request = self.inner.get("search/issues");
        request = request.query(params)?;
        let request = request.build();
        let url = request.url().to_owned();
        let mut response = self.inner.send(request).await?;
        let bytes = response.body_bytes().await?;
        let value = serde_json::from_slice(bytes.as_slice())
            .map_err(|error| Error::Deserialize { error, url, bytes })?;

        Ok(value)
    }

    pub fn search_issues_iter(
        &self,
        params: IssueSearchParams,
    ) -> PaginatedStream<'_, IssueSearchDelegate<'_>> {
        IssueSearchDelegate::new(self, params).into()
    }
}

#[derive(Debug, Serialize)]
pub struct IssueSearchParams {
    #[serde(rename = "q")]
    pub query: String,
    pub per_page: usize,
    pub page: usize,
}

impl IssueSearchParams {
    /// Create new parameters with the given `query` and a default of 10 items
    /// per page, starting at page 0.
    pub fn new<Q>(query: Q) -> Self
    where
        Q: AsRef<str>,
    {
        Self {
            query: query.as_ref().to_owned(),
            per_page: 10,
            page: 0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct IssueSearchItem {
    // We only care about these fields for the example.
    pub repository_url: String,
    pub html_url: String,
    pub id: usize,
    pub title: String,
    // Put the rest of the data into this field.
    #[serde(flatten)]
    pub unknown_keys: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct IssueSearchResponse {
    pub total_count: usize,
    pub items: Vec<IssueSearchItem>,
}

pub struct IssueSearchDelegate<'c> {
    client: &'c Client,
    params: IssueSearchParams,
    total_count: Option<usize>,
}

impl<'c> IssueSearchDelegate<'c> {
    pub fn new(client: &'c Client, params: IssueSearchParams) -> Self {
        Self {
            client,
            params,
            total_count: None,
        }
    }
}

#[async_trait]
impl PaginationDelegate for IssueSearchDelegate<'_> {
    type Item = IssueSearchItem;
    type Error = Error;

    async fn next_page(&mut self) -> Result<Vec<Self::Item>> {
        // Make a new search with the client that has been borrowed.
        let value = self.client.search_issues(&self.params).await?;

        // Update the total count with the new API results. Whenever `Self::total_items`
        // is called, the value provided will be accurate according to the last request
        // that was made.
        self.total_count = Some(value.total_count);

        Ok(value.items)
    }

    fn offset(&self) -> usize {
        // The `PaginatedStream` is asking for the offset of the current page in items.
        self.params.per_page * self.params.page
    }

    fn set_offset(&mut self, value: usize) {
        // The `PaginatedStream` sent us the offset value in terms of the number of
        // items. The value has to be divided for this to work correctly.
        if value % self.params.per_page == 0 {
            self.params.page = value / self.params.per_page;
        } else {
            // If the API last responded with fewer items than `per_page` we must be on the
            // last page already. Go ahead and set it to the highest possible number to
            // force the stream to close.
            self.params.page = usize::MAX / self.params.per_page;
        }
    }

    fn total_items(&self) -> Option<usize> {
        self.total_count
    }
}
