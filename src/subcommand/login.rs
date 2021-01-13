use super::utils;

pub fn login() -> Result<(), ()> {
    utils::std_output::print_info(utils::std_output::PrintColor::INFO, "INFO", "login start");
    Ok(())
}
