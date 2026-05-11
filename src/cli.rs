use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(
    name = "dotenv",
    about = "Read and update environment variables from a .env file",
    disable_version_flag = true
)]
pub struct Cli {
    #[arg(short = 'v', long = "version", action = ArgAction::SetTrue, help = "Output the version number")]
    pub version: bool,

    #[arg(value_name = "key")]
    pub key: Vec<String>,

    #[arg(short, long, help = "Specify the .env file (default: .env)")]
    pub file: Option<String>,

    #[arg(short, long, action = ArgAction::SetTrue, help = "Output as JSON")]
    pub json: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Output as plain text")]
    pub no_json: bool,

    #[arg(short, long, action = ArgAction::SetTrue, help = "Allow multiline values")]
    pub multiline: bool,

    #[arg(short, long, help = "Update the environment variable in the .env file")]
    pub set: Option<String>,

    #[arg(short = 'D', long, action = ArgAction::SetTrue, help = "Delete the environment variable from the .env file")]
    pub delete: bool,

    #[arg(short, long, action = ArgAction::SetTrue, help = "Output extra debugging")]
    pub debug: bool,
}
