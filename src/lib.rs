use async_trait::async_trait;
use reqwest::Result;
use serde::Deserialize;

const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_GET_REPOSITORY_CONTENT_PATH: &str =
    "/repos/rust-lang/this-week-in-rust/contents/content";

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug)]
pub struct ArticleLink {
    pub url: String,
    pub name: String,
    pub download_url: String,
}

pub struct Article {
    pub text: String,
}

impl Article {
    fn new(text: String) -> Self {
        Self { text }
    }
}

#[derive(Deserialize, PartialEq)]
enum ContentType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "dir")]
    Directory,
}

#[derive(Deserialize)]
struct GithubContentResponse {
    r#type: ContentType,
    name: String,
    #[serde(rename = "html_url")]
    url: String,
    download_url: Option<String>,
}

#[async_trait]
pub trait ContentFetcher {
    async fn fetch_content(&self) -> Result<Vec<ArticleLink>>;
    async fn fetch_article(&self, link: &ArticleLink) -> Result<Article>;
}

pub struct GithubContentFetcher {}

impl GithubContentFetcher {
    pub fn new() -> GithubContentFetcher {
        GithubContentFetcher {}
    }
}

#[async_trait]
impl ContentFetcher for GithubContentFetcher {
    async fn fetch_content(&self) -> Result<Vec<ArticleLink>> {
        let url = format!("{}{}", GITHUB_API_URL, GITHUB_GET_REPOSITORY_CONTENT_PATH);

        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;
        let response: Vec<GithubContentResponse> = client.get(url).send().await?.json().await?;
        let mut links: Vec<ArticleLink> = response
            .into_iter()
            .filter(|item| item.r#type == ContentType::File)
            .filter(|item| item.name.ends_with("md") || item.name.ends_with("markdown"))
            .map(|item| ArticleLink {
                url: item.url,
                name: item.name,
                download_url: item.download_url.expect("File should have download url"),
            })
            .collect();
        links.sort_by(|a, b| a.name.cmp(&b.name).reverse());

        Ok(links)
    }

    async fn fetch_article(&self, link: &ArticleLink) -> Result<Article> {
        println!("{}", link.download_url.to_string());
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;
        let response = client.get(&link.download_url).send().await?.text().await?;

        Ok(Article::new(response))
    }
}
