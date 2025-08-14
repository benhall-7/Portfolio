use clap::{Command, CommandFactory};

use crate::args::Cli;

fn completions_for_command(cmd: &Command) -> Vec<String> {
    let mut completions = Vec::new();

    // Subcommands
    completions.extend(cmd.get_subcommands().map(|sc| sc.get_name().to_string()));

    // Flags / options
    completions.extend(cmd.get_arguments().filter_map(|a| {
        // Only include named flags (skip positional args)
        a.get_long().map(|name| format!("--{}", name))
    }));

    completions
}

pub fn get_autocomplete(input: String) -> Vec<String> {
    let cli = Cli::command();

    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut current_cmd = &cli;

    for token in &tokens {
        if let Some(sub) = current_cmd.find_subcommand(token) {
            current_cmd = sub;
        }
    }

    let completions = completions_for_command(current_cmd);

    if input.trim().is_empty() || input.ends_with(' ') {
        completions
    } else {
        let last_token = tokens.last().unwrap_or(&"");
        completions
            .into_iter()
            .filter(|c| c.starts_with(last_token))
            .collect()
    }
}
