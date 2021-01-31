use reqwest;
use reqwest::Response;

use super::utils;

pub fn login() -> Result<(), ()> {
    utils::std_output::print_info(utils::std_output::PrintColor::BLUE, "INFO", "login start");

    let login_url = "https://atcoder.jp/login";

    // username,password,csrf_tokenが必要

    // csrf_tokenは非ログイン状態でログインページにゲットリクエストを送ることで取得できる．
    let parse_str = "input[name=\"csrf_token\"]";
    let selector = scraper::Selector::parse(parse_str).unwrap();
    let body = reqwest::blocking::get(&login_url.to_string())?.text()?;
    let document = scraper::Html::parse_fragment(&body);
    let csrf_token = document.select(&selector);

    // post reqwest
    let client = reqwest::Client::new();
    let mut res = client.post(logini_url).body(/*reqwest body */).send();

    Ok(())
}

async fn call_post_request(
    url: &str,
    params: &std::collections::HashMap<&str, String>,
) -> Result<reqwest::Response, reqwest::Error> {
    Ok(())
}
