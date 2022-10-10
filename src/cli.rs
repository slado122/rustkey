use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    /// Master password is the key to derive your other passwords.
    /// {n}Be sure to remember your master password or you'll LOSE your passwords
    pub master_password: String,
    /// Identify a resource to which you create a password.
    /// {n}Example: google.com, youtube
    /// {n}Be sure to remember your resource identifier or
    /// you'll LOSE your password
    pub resource_identifier: String,
    /// Provide supplementary secret value.
    /// {n}It can be used for password versioning e.g. "1", "2" and so on
    #[arg(short, long)]
    pub secret: Option<String>,
    /// Put the output password on the clipboard
    #[arg(long, default_value_t = false)]
    pub clip: bool,
    /// Path to seed file. It's recommended for use, because it adds additional
    /// entropy
    #[arg(long = "seed")]
    pub seed_file_path: Option<PathBuf>,
}
