use url::Url;

use super::request::*;

pub struct Client {}

pub struct Config {
    pub base_url: Option<Url>,
    pub headers: HashMap<String, Vec<String>>,
}

pub enum ConfigError {
    UrlCannotBeABase,
}

impl Config {
    fn validate(&self) -> Result<(), ConfigError> {
        if let Some(base_url) = self.base_url {
            if base_url.cannot_be_a_base() {
                Err(ConfigError::UrlCannotBeABase)
            }
        }
    }
}

impl Client {
    fn new() -> Self {
        todo!()
    }

    fn head<P>(&self, path: P) -> Request<'_, ()>
    where
        P: AsRef<str>,
    {
        todo!()
    }

    fn get<P>(&self, path: P) -> Request<'_, ()>
    where
        P: AsRef<str>,
    {
        todo!()
    }

    fn put<P, B>(&self, path: P, body: B) -> Request<'_, B>
    where
        P: AsRef<str>,
        B: RequestBody,
    {
        todo!()
    }

    fn post<P, B>(&self, path: P, body: B) -> Request<'_, B>
    where
        P: AsRef<str>,
        B: RequestBody,
    {
        todo!()
    }

    fn delete<P>(&self, path: P) -> Request<'_, ()>
    where
        P: AsRef<str>,
    {
        todo!()
    }
}

pub struct Request<'c> {
    pub client: &'c Client,
    pub method: RequestMethod,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

pub enum RequestMethod {
    Head,
    Get,
    Put,
    Post,
    Delete,
}

// pub trait RequestBody<I>: From<I> + Into<Vec<u8>>
// where
//     I: IntoIterator<Item = u8>,
// {
// }
//
// impl<T, I> RequestBody<I> for T
// where
//     T: From<I> + Into<Vec<u8>>,
//     I: IntoIterator<Item = u8>,
// {
// }
