use std::ops::Not;

use clap::{Command, CommandFactory};

use crate::args::Cli;

fn completions_for_command(cmd: &Command) -> Vec<String> {
    let mut completions = Vec::new();

    // Subcommands
    completions.extend(cmd.get_subcommands().map(|sc| sc.get_name().to_string()));

    // Flags
    completions.extend(cmd.get_arguments().filter_map(|a| {
        // Only include named flags (skip positional args)
        a.get_long().map(|name| format!("--{}", name))
    }));

    completions
}

pub fn get_autocomplete(input: String) -> Vec<String> {
    let cli = Cli::command();

    let mut tokens: Vec<&str> = input.split_whitespace().collect();
    // the last token is popped from the list unless there's a space after it
    let last_token = input
        .ends_with(' ')
        .not()
        .then(|| tokens.pop())
        .flatten()
        .unwrap_or("");

    let mut current_cmd = &cli;

    // go through each token and follow the chain of subcommands
    for token in &tokens {
        if let Some(sub) = current_cmd.find_subcommand(token) {
            current_cmd = sub;
        } else {
            // if the token is a flag
            let is_valid_flag = current_cmd
                .get_arguments()
                .any(|a| a.get_long().map(|l| format!("--{}", l)) == Some(token.to_string()));
            if !is_valid_flag {
                return vec![];
            }
        }
    }

    let completions = completions_for_command(current_cmd);

    // use the last token only to filter the list of options from the previous tokens
    completions
        .into_iter()
        .filter(|c| c.starts_with(last_token))
        .collect()
}
