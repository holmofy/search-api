use crate::Result;
use reqwest::header;
use reqwest_scraper::FromXPath;
use reqwest_scraper::ScraperResponse;

pub struct Baidu;

impl Baidu {
    pub async fn search(keyword: &str) -> Result<Vec<crate::SearchItem>> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0"));
        let html = reqwest::Client::builder()
            .default_headers(headers)
            .build()?
            .get(format!("https://www.baidu.com/s?wd={keyword}"))
            .send()
            .await?
            .xpath()
            .await?;

        Ok(SearchResult::from_xhtml(html).map(|rs| rs.into_iter().map(|r| r.into()).collect())?)
    }
}

#[derive(Debug, FromXPath)]
#[xpath(path = "//div[@id='content_left']/div[contains(@class,'result c-container')]")]
struct SearchResult {
    #[xpath(path = "./@mu", default = "")]
    url: String,
    #[xpath(
        path = "./div[contains(@class,'c-container')]//h3[contains(@class,'c-title')]/a//text()",
        default = ""
    )]
    title: String,
    #[xpath(
        path = "./div[contains(@class,'c-container')]//*[contains(@class,'content-right')]//text()",
        default = ""
    )]
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
    async fn test_baidu() {
        let r = Baidu::search("搜索引擎").await;
        assert_eq!(r.is_ok(), true);
        let r = r.unwrap();
        assert!(r.len() > 0);
        println!("{:?}", r);
    }
}
