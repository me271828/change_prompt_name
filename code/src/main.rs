use clap::{Command, Arg};
use regex::Regex;
use std::io::{Write, BufReader, BufRead, stdin};
use std::fs::OpenOptions;
use dirs::home_dir; // 添加 dirs crate 的引用
use std::process;

/// 代表一个命令行参数错误
#[derive(Debug)]
struct CliError(String);

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CliError {}

/// 更新 Bash 的提示符
fn update_bash_prompt(new_prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = home_dir().ok_or_else(|| CliError("Failed to get home directory".to_string()))?;
    let config_path = home.join(".bashrc");
    let mut file = OpenOptions::new().append(true).open(config_path)?;
    writeln!(file, "PS1='{}'", new_prompt)?;
    Ok(())
}

/// 更新 Fish 的提示符
fn update_fish_prompt(new_prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = home_dir().ok_or_else(|| CliError("Failed to get home directory".to_string()))?;
    let config_path = home.join(".config/fish/config.fish");
    let mut file = OpenOptions::new().append(true).open(config_path)?;
    writeln!(file, "set -gx fish_prompt '{}'", new_prompt)?;
    Ok(())
}

/// 读取用户输入的新提示符
fn read_new_prompt() -> Result<String, Box<dyn std::error::Error>> {
    print!("Enter the new prompt format: ");
    std::io::stdout().flush()?;
    let mut reader = BufReader::new(stdin());
    let mut input = String::new();
    reader.read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// 验证提示符格式
fn validate_prompt(prompt: &str) -> Result<(), CliError> {
    let re = Regex::new(r"^[^\x00-\x1F\x7F]*$").map_err(|_| CliError("Invalid regex".to_string()))?;
    if !re.is_match(prompt) {
        Err(CliError("The prompt contains invalid characters.".to_string()))
    } else {
        Ok(())
    }
}

/// 主函数
fn main() {
    let matches = Command::new("prompt-changer")
        .about("Change the command prompt in Bash or Fish.")
        .arg(
            Arg::new("shell")
                .short('s')
                .long("shell")
                .value_name("SHELL")
                .help("Choose the shell to change the prompt for (bash or fish)")
                .required(true)
                .possible_values(["bash", "fish"]),
        )
        .get_matches();

    let shell = matches.value_of("shell").unwrap();
    let new_prompt = read_new_prompt().unwrap_or_else(|err| {
        eprintln!("Error reading prompt: {}", err);
        process::exit(1);
    });

    if let Err(err) = validate_prompt(&new_prompt) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }

    match shell {
        "bash" => {
            if let Err(err) = update_bash_prompt(&new_prompt) {
                eprintln!("Error updating Bash prompt: {}", err);
                process::exit(1);
            }
            println!("Bash prompt updated successfully.");
        },
        "fish" => {
            if let Err(err) = update_fish_prompt(&new_prompt) {
                eprintln!("Error updating Fish prompt: {}", err);
                process::exit(1);
            }
            println!("Fish prompt updated successfully.");
        },
        _ => unreachable!(),
    }
}