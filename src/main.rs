use std::process::Command;

use clap::{Parser, Subcommand, command};
use colored::Colorize;
use inquire::{
    Select,
    ui::{RenderConfig, StyleSheet},
};
use strip_ansi_escapes::strip_str;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Alias for list")]
    Ls,
    #[command(about = "List all namespaces")]
    List,
    #[command(about = "Set the current namespace")]
    Set,
}

fn main() {
    let cli = Cli::parse();

    let selected_namespace = String::from_utf8(
        Command::new("kubectl")
            .args(vec![
                "config",
                "view",
                "--minify",
                "--output",
                "jsonpath={..namespace}",
            ])
            .output()
            .expect("Failed to execute command")
            .stdout,
    )
    .expect("Error getting current namespace. Is `kubectl` installed?");

    let selected_namespace = if selected_namespace.is_empty() {
        "default".to_string()
    } else {
        selected_namespace
    };

    let namespaces = Command::new("kubectl")
        .args(vec![
            "get",
            "namespaces",
            "-o",
            "jsonpath={.items[*].metadata.name}",
        ])
        .output()
        .expect("Error getting namespaces")
        .stdout;

    let namespaces = String::from_utf8(namespaces).expect("Failed to convert output to string");
    let namespaces = namespaces
        .split_whitespace()
        .map(|ns| ns.trim())
        .map(|ns| {
            if ns == selected_namespace.as_str() {
                ns.green().to_string()
            } else {
                ns.to_string()
            }
        })
        .collect::<Vec<String>>();

    let selected_namespace = (
        selected_namespace.clone(),
        namespaces
            .iter()
            .position(|ns| *ns == selected_namespace.green().to_string())
            .unwrap_or(0),
    );

    match cli.command {
        Commands::List | Commands::Ls => {
            println!("{}", namespaces.join("\n"));
        }
        Commands::Set => {
            let ans = Select::new("Choose namespace", namespaces)
                .with_starting_cursor(selected_namespace.1)
                .with_render_config(RenderConfig::default().with_selected_option(Some(
                    StyleSheet::default().with_fg(inquire::ui::Color::LightBlue),
                )))
                .prompt();

            match ans {
                Ok(choice) => {
                    if strip_str(choice.clone()) == strip_str(selected_namespace.0.clone()) {
                        println!("Namespace is already set to: {}", choice);
                        return;
                    }
                    Command::new("kubectl")
                        .args(vec![
                            "config",
                            "set-context",
                            "--current",
                            format!("--namespace={}", choice).as_str(),
                        ])
                        .output()
                        .expect("Failed to set namespace");
                    println!("Namespace set to: {}", choice.green());
                }
                Err(
                    inquire::InquireError::OperationInterrupted
                    | inquire::InquireError::OperationCanceled,
                ) => println!(),
                Err(_) => println!("An error occured"),
            }
        }
    }
}
