use std::env;
use std::time::Instant;

mod subcommand;
use subcommand::utils::std_output;

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
            std_output::print_info(
                std_output::PrintColor::BLUE,
                "INFO",
                "start creating new directorys",
            );
            match subcommand::new::new(args) {
                Ok(()) => {
                    std_output::print_info(
                        std_output::PrintColor::BLUE,
                        "INFO",
                        "create new contest directory completed",
                    );
                }
                Err(_) => {
                    std_output::print_info(
                        std_output::PrintColor::RED,
                        "RED",
                        "failed to execute the new command",
                    );
                }
            }
        }
        "t" => {
            std_output::print_info(std_output::PrintColor::BLUE, "INFO", "start testing code");
            match subcommand::test::test_code() {
                Ok(()) => {
                    std_output::print_info(
                        std_output::PrintColor::BLUE,
                        "INFO",
                        "code-test is completed.",
                    );
                }
                Err(_) => {
                    std_output::print_info(
                        std_output::PrintColor::RED,
                        "RED",
                        "failed to code test",
                    );
                }
            }
        }
        "s" => {
            std_output::print_info(
                std_output::PrintColor::BLUE,
                "INFO",
                "start submitting code.",
            );
            match subcommand::submit::submit_code() {
                Ok(()) => {
                    std_output::print_info(
                        std_output::PrintColor::BLUE,
                        "INFO",
                        "submit code is completed.",
                    );
                }
                Err(_) => {
                    std_output::print_info(
                        std_output::PrintColor::RED,
                        "RED",
                        "failed to submit code.",
                    );
                }
            }
        }
        "login" => {
            std_output::print_info(std_output::PrintColor::BLUE, "INFO", "start login");
            match subcommand::login::login() {
                Ok(()) => {
                    std_output::print_info(
                        std_output::PrintColor::BLUE,
                        "INFO",
                        "login is completed.",
                    );
                }
                Err(_) => {
                    std_output::print_info(std_output::PrintColor::RED, "RED", "failed to login");
                }
            }
        }
        _ => {
            std_output::print_info(std_output::PrintColor::RED, "RED", "command not found");
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
