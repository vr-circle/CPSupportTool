use std::env;
use std::time::Instant;
mod login;
mod new;
mod submit;
mod test;
mod utils;

fn main() {
    // measure the execution time
    let start = Instant::now();

    let cli_name = "cli";
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("{} new", cli_name);
        return;
    }
    let command = args[1].as_str();
    match command {
        "n" => {
            utils::print_info(
                utils::PrintColor::INFO,
                "INFO",
                "start creating new directorys",
            );
            match new::new(args) {
                Ok(()) => {
                    utils::print_info(
                        utils::PrintColor::SUCCESS,
                        "SUCCESS",
                        "create new contest directory completed",
                    );
                }
                Err(_) => {
                    utils::print_info(
                        utils::PrintColor::ERROR,
                        "ERROR",
                        "failed to execute the new command",
                    );
                }
            }
        }
        "t" => {
            utils::print_info(utils::PrintColor::INFO, "INFO", "start testing code");
            match test::test_code() {
                Ok(()) => {
                    utils::print_info(
                        utils::PrintColor::SUCCESS,
                        "SUCCESS",
                        "code-test is completed.",
                    );
                }
                Err(_) => {
                    utils::print_info(utils::PrintColor::ERROR, "ERROR", "failed to code test");
                }
            }
        }
        "s" => {
            utils::print_info(utils::PrintColor::INFO, "INFO", "start submitting code.");
            match submit::submit_code() {
                Ok(()) => {
                    utils::print_info(
                        utils::PrintColor::SUCCESS,
                        "SUCCESS",
                        "submit code is completed.",
                    );
                }
                Err(_) => {
                    utils::print_info(utils::PrintColor::ERROR, "ERROR", "failed to submit code.");
                }
            }
        }
        "login" => {
            utils::print_info(utils::PrintColor::INFO, "INFO", "start login");
            match login::login() {
                Ok(()) => {
                    utils::print_info(utils::PrintColor::SUCCESS, "SUCCESS", "login is completed.");
                }
                Err(_) => {
                    utils::print_info(utils::PrintColor::ERROR, "ERROR", "failed to login");
                }
            }
        }
        _ => {
            println!("[ERROR] command not found");
        }
    }

    let end = start.elapsed();
    println!(
        "{}.{:03}seconds",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
    return;
}
