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
        "n" | "new" => {
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
                        "FAILURE",
                        "failed to execute the new command",
                    );
                }
            }
        }
        "t" | "test" => {
            std_output::print_info(std_output::PrintColor::BLUE, "INFO", "start testing code");
            match subcommand::test::test() {
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
                        "FAILURE",
                        "failed to code test",
                    );
                }
            }
        }
        "s" | "submit" => {
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
                        "FAILURE",
                        "failed to submit code.",
                    );
                }
            }
        }
        "l" | "login" => {
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
                    std_output::print_info(
                        std_output::PrintColor::RED,
                        "FAILURE",
                        "failed to login",
                    );
                }
            }
        }
        _ => {
            std_output::print_info(
                std_output::PrintColor::RED,
                "ERROR",
                format!("{}: command not found", command).as_str(),
            );
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
