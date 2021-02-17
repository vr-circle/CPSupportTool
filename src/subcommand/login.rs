use reqwest::{self};
use std::collections::HashMap;

use super::utils;

pub fn login() -> Result<(), std::io::Error> {
    utils::std_output::print_info(utils::std_output::PrintColor::BLUE, "INFO", "login start");

    let login_url = "https://atcoder.jp/login";

    // username,password,csrf_tokenが必要
    // csrf_tokenは非ログイン状態でログインページにゲットリクエストを送ることで取得できる．
    let parse_str = r#"input[name="csrf_token"]"#;
    let selector = scraper::Selector::parse(parse_str).unwrap();
    let body = reqwest::blocking::get(&login_url.to_string())
        .unwrap()
        .text()
        .unwrap();
    let document = scraper::Html::parse_fragment(&body);
    let mut csrf_token = "";
    if let Some(element) = document.select(&selector).next() {
        if let Some(token) = element.value().attr("value") {
            csrf_token = token;
        }
    }

    // How to hide input
    print!("username: ");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let username = buf.trim();
    print!("password: ");
    std::io::stdin().read_line(&mut buf).ok();
    let password = buf.trim();

    let param = {
        let mut params = std::collections::HashMap::new();
        params.insert("username", username);
        params.insert("password", password);
        params.insert("csrf_token", csrf_token);
        params
    };

    // post reqwest
    let res = send_post(login_url, param);

    // let client = reqwest::Client::new();
    // let mut res = client.post(logini_url).body(/*reqwest body */).send();

    return Ok(());
}

// send post request function. todo: fix reqwests compile error.
pub async fn send_post(
    url: &str,
    params: HashMap<&str, &str>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.post(url).json(&params).send().await
}
