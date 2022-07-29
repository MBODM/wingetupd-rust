use std::env;

const NO_CONFIRM_ARG: &str = "--no-confirm";
const HELP_ARG: &str = "--help";

pub fn valid() -> bool {
    let supported_args = [NO_CONFIRM_ARG, HELP_ARG];
    env::args()
        .skip(1)
        .all(|arg| supported_args.contains(&arg.as_str()))
}

pub fn has_no_confirm() -> bool {
    env::args().skip(1).any(|arg| arg == NO_CONFIRM_ARG)
}

pub fn has_help() -> bool {
    env::args().skip(1).any(|arg| arg == HELP_ARG)
}
