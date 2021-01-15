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

pub enum ProblemResultType {
    AC,  // accepted
    CE,  // compile error
    WA,  // wrong answer
    TLE, // time limit exceeded
    RE,  // runtime error
    MLE, // memory limit exceeded
}

pub struct ProblemResult {
    result_type: ProblemResultType,
    input: String,
    user_output: String,
    expected_output: String,
}

impl ProblemResult {
    pub fn print(&self) {
        match self.result_type {
            ProblemResultType::AC => {
                println!("AC");
            }
            ProblemResultType::WA => {
                println!("WA");
            }
            ProblemResultType::TLE => {
                println!("TLE");
            }
            _ => {
                println!("error");
            }
        }
        println!("input:\n{}", self.input);
        println!("output: none");
        println!("expected:\n{}", self.expected_output);
    }
}

pub struct ProblemsResult {
    ac_num: i8,
    problems_result: Vec<ProblemResult>,
}

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
            utils::std_output::PrintColor::BLUE,
            "INFO",
            format!("case - {}:", file_name_without_extension).as_str(),
        );
        // テストディレクトリから標準入力用ファイルを取得する
        let input_string = fs::read(format!("{}.in", file_name_without_extension))
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
                .write_all(input_string.as_bytes())
                .expect("failed to write to stdin");
        }

        let is_timeout: bool;

        match subprocess.wait_timeout(Duration::new(2, 0)).unwrap() {
            Some(status) => {
                is_timeout = false;
                status.code()
            }
            None => {
                is_timeout = true;
                subprocess.kill().unwrap();
                subprocess.wait().unwrap().code()
            }
        };

        let mut user_ans = String::new();
        subprocess
            .stdout
            .unwrap()
            .read_to_string(&mut user_ans)
            .unwrap();

        let correct_ans = fs::read(format!("{}.out", file_name_without_extension))
            .unwrap()
            .iter()
            .map(|&s| s as char)
            .collect::<String>();

        if user_ans == correct_ans {
            ac_num += 1;
            let message =
                utils::std_output::color_print(utils::std_output::PrintColor::GREEN, "AC");
            utils::std_output::print_info(
                utils::std_output::PrintColor::GREEN,
                "SUCCESS",
                message.as_str(),
            );
        } else if is_timeout {
            let message =
                utils::std_output::color_print(utils::std_output::PrintColor::YELLOW, "TLE");
            utils::std_output::print_info(
                utils::std_output::PrintColor::RED,
                "FAILURE",
                message.as_str(),
            );
            println!("input:\n{}", input_string);
            println!("output: none");
            println!("expected:\n{}", correct_ans);
        } else {
            let message = utils::std_output::color_print(utils::std_output::PrintColor::RED, "WA");
            utils::std_output::print_info(
                utils::std_output::PrintColor::RED,
                "FAILURE",
                message.as_str(),
            );
            println!("input:\n{}", input_string);
            println!("output:\n{}", user_ans);
            println!("expected:\n{}", correct_ans);
        }
    }
    if ac_num == problem_num {
        utils::std_output::print_info(
            utils::std_output::PrintColor::GREEN,
            "SUCCESS",
            format!("{} AC / {} cases", ac_num, problem_num).as_str(),
        );
    } else {
        utils::std_output::print_info(
            utils::std_output::PrintColor::RED,
            "FAILURE",
            format!("{} AC / {} cases", ac_num, problem_num).as_str(),
        );
    }
    return Ok(());
}
