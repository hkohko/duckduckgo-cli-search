use anyhow::Result;
use select::document::Document;
use select::predicate::Name;
use std::env;
use ureq;
use url;

fn make_request(url: &url::Url) -> Result<String> {
    let agent = ureq::get(url.as_str()).call()?;
    let page = agent.into_string()?;
    Ok(page)
}
fn build_url(search_term: &str) -> Result<url::Url> {
    let base = url::Url::parse("https://html.duckduckgo.com/")?;
    let search = base.join("html/?")?;
    let link = url::Url::parse_with_params(search.as_str(), &[("q", search_term)])?;
    Ok(link)
}
fn html_to_text(res: &str) -> Result<()> {
    let doc = Document::from(res);
    let divs = doc.find(Name("div"));

    let mut div_vec = Vec::with_capacity(50);
    for div in divs {
        if div.attr("class").unwrap_or("None")
            == "result results_links results_links_deep web-result "
        {
            div_vec.push(div);
        }
    }

    let mut a_vec = Vec::with_capacity(50);
    for div in div_vec.iter() {
        for a in div.find(Name("a")) {
            if a.attr("class").unwrap_or("None") == "result__snippet" {
                a_vec.push(a.text());
            }
        }
    }
    println!("\n{}\n", a_vec.join("\n\n"));
    Ok(())
}
fn input() -> String {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
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
