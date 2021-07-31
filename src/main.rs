use rust_weekly::{GithubContentFetcher, ContentFetcher};

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let content_fetcher = GithubContentFetcher::new();
    let links = content_fetcher.fetch_content().await?;

    for link in links {
        println!("{:?}", link);
    }

    Ok(())
}
