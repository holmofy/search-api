use std::future::Future;
use thiserror::Error;

pub use baidu::Baidu;
pub use bing::Bing;
pub use sogou::Sogou;

mod baidu;
mod bing;
mod sogou;

pub trait SearchEngine {
    fn search(&self, keyword: &str) -> impl Future<Output = Result<Vec<SearchItem>>> + Send;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchItem {
    pub url: String,
    pub title: String,
    pub desc: String,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ScraperError(#[from] reqwest_scraper::error::ScraperError),
}
