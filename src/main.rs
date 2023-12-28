use anyhow::Result;
use html2text;
use std::io;
use ureq;
use url;

fn make_request(url: &url::Url) -> Result<String> {
    let agent = ureq::get(url.as_str()).call().expect("");
    let page = agent.into_string()?;
    Ok(page)
}
fn build_url(search_term: &str) -> Result<url::Url> {
    let base = url::Url::parse("https://www.google.com/")?;
    let search = base.join("search?")?;
    let link = url::Url::parse_with_params(search.as_str(), &[("q", search_term)])?;
    Ok(link)
}
fn html_to_text(res: Result<String>) -> Result<()> {
    let res_string = res?;
    let content = res_string.as_bytes();
    let to_print = html2text::from_read(content, 120);
    println!("{to_print}");
    Ok(())
}
fn input() -> String {
    println!("What to search: ");
    let mut userinput = String::with_capacity(100);
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut userinput).expect("");
    userinput
}
fn main() {
    let userinput = input();
    let url = build_url(userinput.as_str());
    let res = match url {
        Ok(val) => make_request(&val),
        Err(_) => Ok("None".to_string()),
    };
    let _ = html_to_text(res);
}
