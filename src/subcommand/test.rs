use super::utils;
use core::panic;
use fs::read_dir;
use std::io::{Read, Write};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, result};
use wait_timeout::ChildExt;

pub struct ProblemCase {
    number: i32,
    std_input: String,
    expected_output: String,
}

#[derive(PartialEq)]
pub enum ExecutionResultType {
    AC, // accepted
    // CE,  // compile error
    WA, // wrong answer
    TLE, // time limit exceeded
        // RE,  // runtime error
        // MLE, // memory limit exceeded
}

pub struct ExecutionResult {
    problem_case: ProblemCase,
    result_type: ExecutionResultType,
    user_output: String,
}

impl ExecutionResult {
    pub fn print(&self) {
        utils::std_output::print_info(
            utils::std_output::PrintColor::BLUE,
            "INFO",
            format!("case - {}:", self.problem_case.number).as_str(),
        );
        match self.result_type {
            ExecutionResultType::AC => {
                let message =
                    utils::std_output::color_print(utils::std_output::PrintColor::GREEN, "AC");
                utils::std_output::print_info(
                    utils::std_output::PrintColor::GREEN,
                    "SUCCESS",
                    message.as_str(),
                );
            }
            ExecutionResultType::WA => {
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
            ExecutionResultType::TLE => {
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
            } // _ => {
              //     println!("error");
              // }
        }
        // new line
        println!("");
    }
}

pub fn test() -> Result<(), ()> {
    let test_dir = "test";
    let mut files_in_test_dir = std::fs::read_dir(test_dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    files_in_test_dir.sort();

    let test_file_hashset: std::collections::HashSet<String> = std::collections::HashSet::new();
    let problem_case_list_tmp: Vec<Mutex<ProblemCase>> = Vec::new();
    for (index, test_file) in files_in_test_dir.iter().enumerate() {
        let file_name_without_extension = test_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .to_string();
        if test_file_hashset.contains(&file_name_without_extension) {
            continue;
        }
        test_file_hashset.insert(file_name_without_extension);
        let std_input_path = file_name_without_extension + ".in";
        let std_output_path = file_name_without_extension + ".out";
        let std_input = fs::read(std_input_path)
            .unwrap()
            .iter()
            .map(|&s| s as char)
            .collect::<String>();
        let expected_output = fs::read(std_output_path)
            .unwrap()
            .iter()
            .map(|&s| s as char)
            .collect::<String>();
        problem_case_list_tmp.push(Mutex::new(ProblemCase {
            number: index as i32,
            std_input: std_input,
            expected_output: expected_output,
        }));
    }

    let problem_case_list = Arc::new(problem_case_list_tmp);
    let mut handles = Vec::new();
    for (index, case) in problem_case_list.into_iter().enumerate() {
        let handle = std::thread::spawn(move || {
            problem_case_list.lock().unwrap()[index] = code_test(case.lock().unwrap(), "./a.out")
        });
        handles.push(handle);
    }
    // wait for each a.out
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }

    // pickup vec from result_list
    let result_list_vec = match Arc::try_unwrap(result_list) {
        Ok(v) => v,
        Err(_) => panic!("error: failed to execute in Arc::try_unwrap."),
    };

    // print information
    for x in result_list_vec.iter() {
        x.lock().unwrap().print();
    }

    Ok(())
}

fn code_test(case: ProblemCase, execution_file_path: &str) -> ExecutionResult {
    let mut subprocess = std::process::Command::new(execution_file_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let stdin = subprocess.stdin.as_mut().expect("failed to get stdin");
        stdin
            .write_all(case.std_input.as_bytes())
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
    let mut result: ExecutionResult = ExecutionResult {
        problem_case: case,
        result_type: ExecutionResultType::AC,
        user_output: user_ans,
    };
    if is_timeout {
        result.result_type = ExecutionResultType::TLE;
    } else if user_ans == case.expected_output {
        result.result_type = ExecutionResultType::AC;
    } else {
        result.result_type = ExecutionResultType::WA;
    }
    return result;
}
