use std::{fs, process};

use crate::cli::Cli;
use clap::Parser;

mod cli;
mod clipboard;
mod crypt;

fn main() {
    let args = Cli::parse();
    let additional_data = match args.seed_file_path {
        Some(file_path) => fs::read_to_string(file_path).unwrap_or_else(|_| {
            println!("Should have been able to read the seed file.");
            process::exit(1);
        }),
        None => "".to_owned(),
    };
    let secret = args.secret.unwrap_or("".to_owned());
    let password = crypt::derive_password(
        &args.master_password,
        &args.resource_identifier,
        &secret,
        &additional_data,
    );
    if args.clip {
        clipboard::put_to_clipboard(&password);
        println!("Password was put onto clipboard.")
    } else {
        println!("{}", &password);
    }
}
