use reqwest;
use scraper;
use std::fs;
use std::io::Write;

/*
json example
{
    "A":{
        name:"SampleProblem",
        test_dir:"./test/A/",
        submit_file:"./sample.cpp",
    },
    "B":{
        ...
    },
    ...
}
*/

pub fn new(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // create a config file what is written problems informations.

    if args.len() == 2 {
        println!("cli new <target_contest_name>");
        return Ok(());
    }
    fs::create_dir(format!("{}", args[2]))?;
    let atcoder_url = "https://atcoder.jp";
    let tasks_url = format!("{}/contests/{}/tasks", atcoder_url, args[2]);
    let parse_str = "tbody > tr > td.text-center.no-break > a";
    let selector = scraper::Selector::parse(parse_str).unwrap();
    let body = reqwest::blocking::get(&tasks_url)?.text()?;
    let document = scraper::Html::parse_fragment(&body);
    let elements = document.select(&selector);

    for e in elements {
        fs::create_dir_all(format!("./{}/{}/test", args[2], e.text().next().unwrap()))?;
        println!(
            "create dir: ./{}/{}/test",
            args[2],
            e.text().next().unwrap()
        );

        // duplicate the specified file.
        let target_file_name = "./sample";
        let target_file_extension = ".cpp";
        let target_file = format!("{}{}", target_file_name, target_file_extension);
        match fs::copy(
            target_file,
            format!(
                "./{}/{}/main{}",
                args[2],
                e.text().next().unwrap(),
                target_file_extension
            ),
        ) {
            Err(why) => println!("Err: {}", why),
            Ok(_) => {}
        }

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
        println!("problem url: {}{}", atcoder_url, target_task_url);

        let parse_str = r#"div[id="task-statement"] > .lang > .lang-ja > .part > section > pre"#;
        let selector = scraper::Selector::parse(parse_str).unwrap();
        let body = reqwest::blocking::get(format!("{}{}", atcoder_url, target_task_url).as_str())?
            .text()?;
        let document = scraper::Html::parse_document(&body);
        let elements = document.select(&selector);
        for (index, element) in elements.enumerate() {
            if index % 2 == 0 {
                println!(
                    "create file: ./{}/{}/{}.in",
                    args[2],
                    e.text().next().unwrap(),
                    index / 2 + 1
                );
                let mut file = fs::File::create(format!(
                    "./{}/{}/test/{}.in",
                    args[2],
                    e.text().next().unwrap(),
                    index / 2 + 1
                ))?;
                let context = element.text().next().unwrap();
                file.write_all(context.as_bytes()).unwrap();
                println!("{}", context);
            } else {
                println!(
                    "create file: ./{}/{}/{}.out",
                    args[2],
                    e.text().next().unwrap(),
                    index / 2 + 1
                );
                let mut file = fs::File::create(format!(
                    "./{}/{}/test/{}.out",
                    args[2],
                    e.text().next().unwrap(),
                    index / 2 + 1
                ))?;
                let context = element.text().next().unwrap();
                file.write_all(context.as_bytes()).unwrap();
                println!("{}", context);
            }
        }
    }
    Ok(())
}
