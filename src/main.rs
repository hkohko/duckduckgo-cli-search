use anyhow::Result;
use html2text;
use ureq;
use url;
use std::env;

fn make_request(url: &url::Url) -> Result<String> {
    let agent = ureq::get(url.as_str()).call()?;
    let page = agent.into_string()?;
    Ok(page)
}
fn build_url(search_term: &str) -> Result<url::Url> {
    let base = url::Url::parse("https://www.google.com/")?;
    let search = base.join("search?")?;
    let link = url::Url::parse_with_params(search.as_str(), &[("q", search_term)])?;
    Ok(link)
}
fn html_to_text(res: &str) -> Result<()> {
    let content = res.as_bytes();
    let to_print = html2text::from_read(content, 120);
    println!("{to_print}");
    Ok(())
}
fn input() -> String {
    let args = env::args().collect::<Vec<String>>();
    args.join(" ")
}
fn main() {
    let userinput = input();
    let url = build_url(userinput.as_str());
    let res = match url {
        Ok(val) => make_request(&val),
        Err(e) => panic!("{e}"),
    };
    let _ = html_to_text(&res.expect("").as_str());
}
