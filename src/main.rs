use reqwest;
use scraper;
use std::env;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let cli_name = "cli";
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("{} new", cli_name);
        return Ok(());
    }
    let command = args[1].as_str();
    match command {
        "new" => {
            if args.len() == 2 {
                println!("cli new <target_contest_name>");
                return Ok(());
            }

            let tasks_url = format!("https://atcoder.jp/contests/{}/tasks", args[2]);
            println!("{}", tasks_url);

            let parse_str = "tbody > tr > td.text-center.no-break > a";
            let selector = scraper::Selector::parse(parse_str).unwrap();

            let body = reqwest::blocking::get(&tasks_url)?.text()?;
            let document = scraper::Html::parse_fragment(&body);
            let elements = document.select(&selector);

            let atcoder_url = "https://atcoder.jp";

            for e in elements {
                println!("problem-{}", e.text().next().unwrap());
                let mut index_start = 0;
                let mut index_end = 0;
                for (i, c) in e.html().chars().enumerate() {
                    if c == '"' {
                        if index_start == 0 {
                            index_start = i + 1;
                        } else {
                            index_end = i;
                        }
                    }
                }
                let target_task_url = e
                    .html()
                    .chars()
                    .enumerate()
                    .filter(|&(i, _)| i >= index_start && i < index_end)
                    .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));

                let parse_str = ".lang-ja > .part pre";
                let selector = scraper::Selector::parse(parse_str).unwrap();
                let body =
                    reqwest::blocking::get(format!("{}{}", atcoder_url, target_task_url).as_str())?
                        .text()?;
                let document = scraper::Html::parse_document(&body);
                let elements = document.select(&selector);
                for (index, element) in elements.enumerate() {
                    if index % 2 == 0 {
                        println!("input: {}", index / 2 + 1);
                    } else {
                        println!("output: {}", index / 2 + 1);
                    }
                    println!("{}", element.text().next().unwrap());
                }
            }
        }
        "s" => {
            println!("submit");
        }
        _ => {
            println!("command not found");
        }
    }

    let end = start.elapsed();
    println!(
        "{}.{:03}seconds",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
    return Ok(());
}
