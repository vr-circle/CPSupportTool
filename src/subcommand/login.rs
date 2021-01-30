use reqwest;
use reqwest::Response;

use super::utils;

pub fn login() -> Result<(), ()> {
    utils::std_output::print_info(utils::std_output::PrintColor::BLUE, "INFO", "login start");

    let url = "https://atcoder.jp/";
    let client = reqwest::Client::builder().build().unwrap();

    Ok(())
}

// async fn call_post_request(
//     url: &str,
//     params: &std::collections::HashMap<&str, String>,
// ) -> Result<reqwest::Response, reqwest::Error> {
//     Ok(())
// }
