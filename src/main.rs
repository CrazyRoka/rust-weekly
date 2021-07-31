use clap::{App, Arg, SubCommand};
use rust_weekly::{ContentFetcher, GithubContentFetcher};
use termimad::MadSkin;

const DEFAULT_ARTICLES_COUNT: usize = 5;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let matches = App::new("Rust Weekly CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("articles")
                .about("Prints recent articles")
                .arg(
                    Arg::with_name("count")
                        .short("c")
                        .takes_value(true)
                        .multiple(false)
                        .help(&format!(
                            "print 'c' recent articles. Default to {}",
                            DEFAULT_ARTICLES_COUNT
                        )),
                ),
        )
        .subcommand(SubCommand::with_name("latest").about("Prints latest article"))
        .get_matches();

    match matches.subcommand() {
        ("articles", Some(args)) => {
            let articles_count = match args.value_of("count") {
                Some(count) => match count.parse() {
                    Ok(value) if value > 0 => value,
                    _ => {
                        eprintln!(
                            "Invalid count of articles: \"{}\". Expected a positive number",
                            count
                        );
                        return Ok(());
                    }
                },
                _ => DEFAULT_ARTICLES_COUNT,
            };

            print_articles_list(articles_count).await?;
        }
        ("latest", _) | _ => {
            print_latest_article().await?
        }
    }

    Ok(())
}

async fn print_articles_list(articles_count: usize) -> reqwest::Result<()> {
    let content_fetcher = GithubContentFetcher::new();
    let links = content_fetcher.fetch_content().await?;

    println!("{}", links.len());
    for link in links.into_iter().take(articles_count) {
        println!("[{}] {}", link.name, link.url);
    }

    Ok(())
}

async fn print_latest_article() -> reqwest::Result<()> {
    let content_fetcher = GithubContentFetcher::new();
    let links = content_fetcher.fetch_content().await?;

    let latest_link = links.get(0).expect("There should be at least one article");
    let article = content_fetcher.fetch_article(latest_link).await?;

    let skin = MadSkin::default();
    skin.print_text(&article.text);

    Ok(())
}
