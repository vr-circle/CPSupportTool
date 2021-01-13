use super::utils;
use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

pub fn test_code() -> Result<(), Box<dyn std::error::Error>> {
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
        utils::std_output::print_info(
            utils::std_output::PrintColor::INFO,
            "INFO",
            format!("case - {}:", file_name_without_extension).as_str(),
        );
        // テストディレクトリから標準入力用ファイルを取得する
        let input_string = fs::read(format!("{}.in", file_name_without_extension))
            .unwrap()
            .iter()
            .map(|&s| s as char)
            .collect::<String>();

        let process_status: i8;
        let mut process = std::process::Command::new("./a.out")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        println!("created process");

        process
            .stdin
            .unwrap()
            .write_all(input_string.as_bytes())
            .unwrap();
        let mut user_ans = String::new();
        process
            .stdout
            .unwrap()
            .read_to_string(&mut user_ans)
            .unwrap();
        let status_code = match process.wait_timeout(Duration::from_secs(2)).unwrap() {
            Some(status) => {
                println!("Some");
                println!("status: {}", status.code().unwrap());
                Ok(())
            }
            None => {
                println!("None");
                Err(())
            }
        };
        println!("status code: {}", status_code == Ok(()));
        // match process.stdin.unwrap().write_all(input_string.as_bytes()) {
        // match process.stdout.unwrap().read_to_string(&mut user_ans) {

        // let wait_secs = Duration::from_secs(2);
        // let _status_code = match process.wait_timeout(wait_secs).unwrap() {
        //     Some(status) => {
        //         println!("Some");
        //         process_status = 1;
        //         status.code().unwrap()
        //     }
        //     None => {
        //         println!("None");
        //         process.kill().unwrap();
        //         process_status = 0;
        //         process.wait().unwrap().code().unwrap()
        //     }
        // };
        // println!("status code: {}", _status_code);
        // println!("process status: {}", process_status);
        // if _status_code == 0 {
        //     continue;
        // }
        // match process.stdin.unwrap().write_all(input_string.as_bytes()) {
        //     Err(why) => panic!("couldn't write to ./a.out stdin: {}", why),
        //     Ok(_) => {}
        // }
        // let mut user_ans = String::new();
        // match process.stdout.unwrap().read_to_string(&mut user_ans) {
        //     Err(why) => panic!("couldn't read ./a.out stdout: {}", why),
        //     Ok(_) => {}
        // }

        // if process_status == 0 {
        //     println!("TLE");
        //     continue;
        // }
        // let correct_ans = fs::read(format!("{}.out", file_name_without_extension))
        //     .unwrap()
        //     .iter()
        //     .map(|&s| s as char)
        //     .collect::<String>();
        // if user_ans == correct_ans {
        //     ac_num += 1;
        //     utils::std_output::print_info(utils::std_output::PrintColor::SUCCESS, "SUCCESS", "AC");
        // } else {
        //     utils::std_output::print_info(utils::std_output::PrintColor::ERROR, "FAILURE", "WA");
        //     println!("input:\n{}", input_string);
        //     println!("output:\n{}", user_ans);
        //     println!("expected:\n{}", correct_ans);
        // }
    }
    if ac_num == problem_num {
        utils::std_output::print_info(
            utils::std_output::PrintColor::SUCCESS,
            "SUCCESS",
            format!("{} AC / {} cases", ac_num, problem_num).as_str(),
        );
    } else {
        utils::std_output::print_info(
            utils::std_output::PrintColor::ERROR,
            "FAILURE",
            format!("{} AC / {} cases", ac_num, problem_num).as_str(),
        );
    }
    return Ok(());
}
