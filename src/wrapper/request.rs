use url::Url;

use super::client::Client;

pub enum BuildError {
    UrlCannotBeABase,
}

pub struct Request {
    method: Method,
    url: String,
    body: String,
    headers: Vec<(String, String)>,
}

pub enum Method {
    Get,
    Put,
    Post,
    Patch,
    Delete,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RequestBuilder {
    method: Option<Method>,
    url: Option<Url>,
    query: Option<QueryParams>,
    body: Option<Body>,
    headers: Option<Headers>,
    chain: Vec<FnOnce(&mut Self) -> Result<(), BuildError>>,
}

impl RequestBuilder {
    pub fn new(method: Method) -> Self {
        Self {
            method: Some(method),
            ..Default::default()
        }
    }

    pub fn from_client(client: &Client, method: Method) -> Self {
        Self {
            method: Some(method),
            url: client.base_url.clone(),
            headers: client.headers.clone(),
            ..Default::default()
        }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn url<T>(mut self, url: T) -> Self
    where
        T: TryInto<Url>,
        <T as TryInto>::Error: Into<BuildError>,
    {
        self.chain.push(|this: &mut Self| {
            this.url = Some(match this.url {
                Some(url) => url.join(url)?,
                None => {
                    let url = Url::try_from(url)?;
                    if url.cannot_be_a_base() {
                        Err(BuildError::UrlCannotBeABase)?;
                    }
                    url
                }
            });
        });

        self
    }

    pub fn query<T>(self, query: T) -> Self
    where
        T: TryInto<QueryString>,
        <T as TryInto>::Error: Into<BuildError>,
    {
        self.query = Some(query.try_into()?);
        self
    }

    pub fn body<T>(self, body: T) -> Self
    where
        T: TryInto<Body>,
        <T as TryInto>::Error: Into<BuildError>,
    {
        self.chain.push(|this: &mut Self| {
            this.body = Some(body.try_into()?);
        });

        self
    }

    pub fn headers<T>(self, headers: T) -> Self
    where
        T: TryInto<Headers>,
        <T as TryInto>::Error: Into<BuildError>,
    {
        this.chain
        self.headers = Some(match self.headers {
            Some(current) => current.extend(headers.try_into()?),
            None => headers,
        });
        Ok(self)
    }

    pub fn done(self) -> Result<Self, BuildError> {
        if let Some(errors) = self.errors {
            if errors.len() == 1 {
                Err(errors.pop().unwrap())?
            } else {
                Err(BuildError::Multiple(errors))?
            }
        }
    }
}

macro_rules! wrapper_type {
    ($($vis:vis)? struct $name:ident($inner:ty)) => {
        $($vis)? struct $name($inner);

        impl Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            type Target = $inner;

            fn deref(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

wrapper_type!(pub struct QueryParams(String));
wrapper_type!(pub struct Body(Vec<u8>));
wrapper_type!(pub struct Headers(Vec<String, String>));
