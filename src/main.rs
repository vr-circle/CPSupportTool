use reqwest;
use scraper;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::Write;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let init_file_path = "./sample.cpp";

    let cli_name = "cli";
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("{} new", cli_name);
        return Ok(());
    }
    let command = args[1].as_str();
    match command {
        "n" => {
            println!("start creating new contest files");
            if args.len() == 2 {
                println!("cli new <target_contest_name>");
                return Ok(());
            }
            fs::create_dir(format!("{}", args[2]))?;
            let tasks_url = format!("https://atcoder.jp/contests/{}/tasks", args[2]);
            // println!("target tasks url: {}", tasks_url);
            let parse_str = "tbody > tr > td.text-center.no-break > a";
            let selector = scraper::Selector::parse(parse_str).unwrap();

            let body = reqwest::blocking::get(&tasks_url)?.text()?;
            let document = scraper::Html::parse_fragment(&body);
            let elements = document.select(&selector);

            let atcoder_url = "https://atcoder.jp";

            for e in elements {
                fs::create_dir_all(format!("./{}/{}/test", args[2], e.text().next().unwrap()))?;
                // println!(
                //     "create dir: ./{}/{}/test",
                //     args[2],
                //     e.text().next().unwrap()
                // );

                match fs::copy(
                    init_file_path,
                    format!("./{}/{}/main.cpp", args[2], e.text().next().unwrap()),
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

                // println!("problem url: {}{}", atcoder_url, target_task_url);

                let parse_str =
                    r#"div[id="task-statement"] > .lang > .lang-ja > .part > section > pre"#;
                let selector = scraper::Selector::parse(parse_str).unwrap();
                let body =
                    reqwest::blocking::get(format!("{}{}", atcoder_url, target_task_url).as_str())?
                        .text()?;
                let document = scraper::Html::parse_document(&body);
                let elements = document.select(&selector);
                for (index, element) in elements.enumerate() {
                    if index % 2 == 0 {
                        // println!(
                        //     "create file: ./{}/{}/{}.in",
                        //     args[2],
                        //     e.text().next().unwrap(),
                        //     index / 2 + 1
                        // );
                        let mut file = fs::File::create(format!(
                            "./{}/{}/test/{}.in",
                            args[2],
                            e.text().next().unwrap(),
                            index / 2 + 1
                        ))?;
                        let context = element.text().next().unwrap();
                        file.write_all(context.as_bytes()).unwrap();
                    // println!("{}", context);
                    } else {
                        // println!(
                        //     "create file: ./{}/{}/{}.out",
                        //     args[2],
                        //     e.text().next().unwrap(),
                        //     index / 2 + 1
                        // );
                        let mut file = fs::File::create(format!(
                            "./{}/{}/test/{}.out",
                            args[2],
                            e.text().next().unwrap(),
                            index / 2 + 1
                        ))?;
                        let context = element.text().next().unwrap();
                        file.write_all(context.as_bytes()).unwrap();
                        // println!("{}", context);
                    }
                }
            }
        }
        "t" => {
            let mut stdout_color: i8 = 4;
            print!("[\x1b[{}mINFO\x1b[m] ", 30 + stdout_color);
            println!("start testing a code.");
            let test_dir = "test";
            let test_files = fs::read_dir(test_dir).unwrap();
            let mut problem_num = 0;
            let mut ac_num = 0;
            for test_file in test_files {
                let path_name = format!("{}", test_file.unwrap().path().display());
                let split_path_name: Vec<&str> = path_name.split('.').collect();
                let file_name_without_extension = split_path_name[0];
                if split_path_name[1] == "out" {
                    continue;
                }
                problem_num += 1;
                stdout_color = 6;
                print!("[\x1b[{}mFILE\x1b[m] ", 30 + stdout_color);
                println!("case - {}:", file_name_without_extension);
                // テストディレクトリから標準入力用ファイルを取得する
                let input_string = fs::read(format!("{}.in", file_name_without_extension))?
                    .iter()
                    .map(|&s| s as char)
                    .collect::<String>();
                let process = match std::process::Command::new("./a.out")
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .spawn()
                {
                    Err(why) => panic!("couldn't spawn: {}", why),
                    Ok(process) => process,
                };
                match process.stdin.unwrap().write_all(input_string.as_bytes()) {
                    Err(why) => panic!("couldn't write to ./a.out stdin: {}", why),
                    Ok(_) => {}
                }
                let mut user_ans = String::new();
                match process.stdout.unwrap().read_to_string(&mut user_ans) {
                    Err(why) => panic!("couldn't read ./a.out stdout: {}", why),
                    Ok(_) => {}
                }
                let correct_ans = fs::read(format!("{}.out", file_name_without_extension))?
                    .iter()
                    .map(|&s| s as char)
                    .collect::<String>();
                if user_ans == correct_ans {
                    stdout_color = 2;
                    ac_num += 1;
                    print!("[\x1b[{}mSUCCESS\x1b[m] ", 30 + stdout_color);
                    println!("\x1b[{}mAC\x1b[m\n", 30 + stdout_color);
                } else {
                    stdout_color = 1;
                    print!("[\x1b[{}mFAILURE\x1b[m] ", 30 + stdout_color);
                    println!("\x1b[{}mWA\x1b[m", 30 + stdout_color);
                    println!("input:\n{}", input_string);
                    println!("output:\n{}", user_ans);
                    println!("expected:\n{}", correct_ans);
                }
            }
            if ac_num == problem_num {
                stdout_color = 2;
                print!("[\x1b[{}mSUCCESS\x1b[m] ", 30 + stdout_color);
                println!("{} AC / {} cases", ac_num, problem_num);
            } else {
                stdout_color = 1;
                print!("[\x1b[{}mFAILURE\x1b[m] ", 30 + stdout_color);
                println!("{} AC / {} cases", ac_num, problem_num);
            }
        }
        "s" => {
            println!("start submitting a code");
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
