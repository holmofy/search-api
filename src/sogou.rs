use crate::Result;
use crate::SearchEngine;
use async_trait::async_trait;
use reqwest::header;
use reqwest_scraper::FromXPath;
use reqwest_scraper::ScraperResponse;

pub struct Sogou;

#[async_trait]
impl SearchEngine for Sogou {
    async fn search(&self, keyword: &str) -> Result<Vec<crate::SearchItem>> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0"));

        let html = reqwest::Client::builder()
            .default_headers(headers)
            .build()?
            .get(format!("https://sogou.com/web?query={keyword}"))
            .send()
            .await?
            .xpath()
            .await?;

        Ok(SearchResult::from_xhtml(html).map(|rs| rs.into_iter().map(|r| r.into()).collect())?)
    }
}

#[derive(Debug, FromXPath)]
#[xpath(path = "//div[@id='main']//div[contains(@class,'results')]/div[@class='vrwrap']")]
struct SearchResult {
    #[xpath(
        path = ".//div[contains(@class,'result_list')]/@data-url",
        default = ""
    )]
    url: String,
    #[xpath(path = ".//h3[contains(@class,'vr-title')]/a//text()", default = "")]
    title: String,
    #[xpath(path = ".//div[contains(@class,'space-txt')]//text()", default = "")]
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
    async fn test_sogou() {
        let sogou = Sogou;
        let r = sogou.search("搜索引擎").await;
        assert_eq!(r.is_ok(), true);
        let r = r.unwrap();
        assert!(r.len() > 0);
        println!("{:?}", r);
    }
}
