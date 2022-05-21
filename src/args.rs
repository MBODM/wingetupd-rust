use std::env;

const NO_CONFIRM_ARG: &str = "--no-confirm";
const NO_LOG_ARG: &str = "--no-log";
const HELP_ARG: &str = "--help";

pub fn valid() -> bool {
    let supported_args = vec![NO_CONFIRM_ARG, NO_LOG_ARG];
    get_args()
        .iter()
        .all(|arg| supported_args.contains(&arg.as_str()))
}

pub fn has_no_confirm() -> bool {
    get_args().iter().any(|arg| arg == NO_CONFIRM_ARG)
}

pub fn has_help() -> bool {
    get_args().contains(&HELP_ARG.to_string())
}

fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let args_without_bin = &args[1..args.len()];
    args_without_bin.to_vec()
}
