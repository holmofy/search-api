use crate::Result;
use crate::SearchEngine;
use async_trait::async_trait;
use reqwest_scraper::FromXPath;
use reqwest_scraper::ScraperResponse;

pub struct Bing;

#[async_trait]
impl SearchEngine for Bing {
    async fn search(keyword: &str) -> Result<Vec<crate::SearchItem>> {
        let html = reqwest::get(format!("https://www.bing.com/search?q={keyword}"))
            .await?
            .xpath()
            .await?;

        Ok(SearchResult::from_xhtml(html).map(|rs| rs.into_iter().map(|r| r.into()).collect())?)
    }
}

#[derive(Debug, FromXPath)]
#[xpath(path = "//ol[@id='b_results']/li[contains(@class,'b_algo')]")]
struct SearchResult {
    #[xpath(
        path = ".//a[contains(@class,'tilk')]//div[contains(@class,'tpmeta')]//cite",
        default = ""
    )]
    url: String,
    #[xpath(path = "./h2/a/text()", default = "")]
    title: String,
    #[xpath(path = "./div[contains(@role,'contentinfo')]/p/text()", default = "")]
    desc: String,
}

impl From<SearchResult> for crate::SearchItem {
    fn from(value: SearchResult) -> Self {
        Self {
            url: value.url,
            title: value.title,
            desc: value.desc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bing() {
        let r = Bing::search("搜索引擎").await;
        assert_eq!(r.is_ok(), true);
        let r = r.unwrap();
        assert!(r.len() > 0);
        println!("{:?}", r);
    }
}
