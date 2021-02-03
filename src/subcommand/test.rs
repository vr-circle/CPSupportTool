#![allow(unused_imports)]
use super::utils;
use core::panic;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{fmt::format, io::prelude::*};
use std::{fs, result};
use subprocess::Redirection;
use subprocess::{Exec, Popen, PopenConfig};
use wait_timeout::ChildExt;

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum ProblemResultType {
    AC,  // accepted
    CE,  // compile error
    WA,  // wrong answer
    TLE, // time limit exceeded
    RE,  // runtime error
    MLE, // memory limit exceeded
}

pub struct ProblemResult {
    problem_path: String,
    result_type: ProblemResultType,
    input: String,
    user_output: String,
    expected_output: String,
}

impl ProblemResult {
    pub fn print(&self) {
        utils::std_output::print_info(
            utils::std_output::PrintColor::BLUE,
            "INFO",
            format!("case - {}:", self.problem_path).as_str(),
        );
        match self.result_type {
            ProblemResultType::AC => {
                let message =
                    utils::std_output::color_print(utils::std_output::PrintColor::GREEN, "AC");
                utils::std_output::print_info(
                    utils::std_output::PrintColor::GREEN,
                    "SUCCESS",
                    message.as_str(),
                );
            }
            ProblemResultType::WA => {
                let message =
                    utils::std_output::color_print(utils::std_output::PrintColor::RED, "WA");
                utils::std_output::print_info(
                    utils::std_output::PrintColor::RED,
                    "FAILURE",
                    message.as_str(),
                );
                println!("input:\n{}", self.input);
                println!("output: \n{}", self.user_output);
                println!("expected:\n{}", self.expected_output);
            }
            ProblemResultType::TLE => {
                let message =
                    utils::std_output::color_print(utils::std_output::PrintColor::YELLOW, "TLE");
                utils::std_output::print_info(
                    utils::std_output::PrintColor::RED,
                    "FAILURE",
                    message.as_str(),
                );
                println!("input:\n{}", self.input);
                println!("output: \n{}", self.user_output);
                println!("expected:\n{}", self.expected_output);
            }
            _ => {
                println!("error");
            }
        }
        println!("");
    }
}

pub fn code_test() -> Result<(), ()> {
    let test_dir = "test";
    let test_files = fs::read_dir(test_dir).unwrap();
    let mut result_list: Vec<ProblemResult> = Vec::new();
    for test_file in test_files {
        let path_name = format!("{}", test_file.unwrap().path().display());
        let split_path_name: Vec<&str> = path_name.split('.').collect();
        let file_name_without_extension = split_path_name[0];
        if split_path_name[1] == "out" {
            continue;
        }
        // テストディレクトリから標準入力用ファイルを取得する
        let std_input = fs::read(format!("{}.in", file_name_without_extension))
            .unwrap()
            .iter()
            .map(|&s| s as char)
            .collect::<String>();

        let mut subprocess = std::process::Command::new("./a.out")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        {
            let stdin = subprocess.stdin.as_mut().expect("failed to get stdin");
            stdin
                .write_all(std_input.as_bytes())
                .expect("failed to write to stdin");
        }
        let is_timeout: bool;
        let wait_time = Duration::new(2, 0);
        match subprocess.wait_timeout(wait_time).unwrap() {
            Some(_) => {
                is_timeout = false;
            }
            None => {
                is_timeout = true;
                subprocess.kill().unwrap();
            }
        };
        let mut user_ans = String::new();
        subprocess
            .stdout
            .unwrap()
            .read_to_string(&mut user_ans)
            .unwrap();
        let expected_ans = fs::read(format!("{}.out", file_name_without_extension))
            .unwrap()
            .iter()
            .map(|&s| s as char)
            .collect::<String>();
        let mut result: ProblemResult = ProblemResult {
            problem_path: path_name,
            result_type: ProblemResultType::AC,
            input: std_input,
            user_output: user_ans.clone(),
            expected_output: expected_ans.clone(),
        };
        if is_timeout {
            result.result_type = ProblemResultType::TLE;
        } else if user_ans == expected_ans {
            result.result_type = ProblemResultType::AC;
        } else {
            result.result_type = ProblemResultType::WA;
        }
        result_list.push(result);
    }
    let mut ac_num = 0;
    for problem_result in &result_list {
        problem_result.print();
        if problem_result.result_type == ProblemResultType::AC {
            ac_num += 1;
        }
    }
    if ac_num == result_list.len() {
        utils::std_output::print_info(
            utils::std_output::PrintColor::GREEN,
            "SUCCESS",
            format!("{} AC / {} cases", ac_num, result_list.len()).as_str(),
        );
    } else {
        utils::std_output::print_info(
            utils::std_output::PrintColor::RED,
            "FAILURE",
            format!("{} AC / {} cases", ac_num, result_list.len()).as_str(),
        );
    }
    return Ok(());
}
