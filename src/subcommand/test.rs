use super::utils;
use core::panic;
use std::fs;
use std::io::{Read, Write};
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wait_timeout::ChildExt;

#[derive(PartialEq)]
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
        // new line
        println!("");
    }
}

pub fn test() -> Result<(), ()> {
    let test_dir = "test";
    // let test_files = fs::read_dir(test_dir).unwrap();
    let test_files = [("1.in", "1.out"), ("2.in", "2.out")]; // sample

    let result_list_tmp: Vec<Mutex<ProblemResult>> = (0..test_files.len() as i32)
        .map(|c| {
            Mutex::new(ProblemResult {
                input: String::from(""),
                problem_path: String::from(""),
                user_output: String::from(""),
                expected_output: String::from(""),
                result_type: ProblemResultType::AC,
            })
        })
        .collect();

    let result_list = Arc::new(result_list_tmp);

    let handles = Vec::new();
    for test_file_path in test_files.iter() {
        let stdin_path = test_file_path.0;
        let stdout_path = test_file_path.1;
        let handle = std::thread::spawn(move || {
            // add an argument what &mut ProblemResult in result_list
            code_test("./a.out", stdin_path, stdout_path);
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

fn code_test(
    execute_file_path: &str,
    std_input_path: &str,
    std_output_path: &str,
) -> ProblemResult {
    // execute_file_path == "./a.out";  maybe
    let std_input = fs::read(std_input_path)
        .unwrap()
        .iter()
        .map(|&s| s as char)
        .collect::<String>();
    let mut subprocess = std::process::Command::new(execute_file_path)
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
    let expected_ans = fs::read(std_output_path)
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
    return result;
}
