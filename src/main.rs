/*
    fix-cursor: CLI that makes Flatpak XWayland
    Copyright (C) 2026 Xleepree
    xleepree@proton.me

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
*/

use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use serde_json;
use std::process::Command;

fn list_location() -> PathBuf {
    let home = PathBuf::from(env::var_os("HOME").expect("HOME is not set"));
    return home.join(".config/fix-cursor/list.json");
}

fn create_list() -> Result<(), std::io::Error> {
    let list_path = list_location();
    if !Path::exists(list_location().as_path()) {
        println!("List does not exist, creating list");
        if let Some(parent) = list_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(list_path)?;
        println!("Created list at {}", list_location().display());
    }
    return Ok(());
}

fn list_as_plaintext() -> Result<String, std::io::Error> {
    create_list()?;
    return fs::read_to_string(list_location());
}

fn list_as_jsonvalue() -> Result<serde_json::Value, std::io::Error> {
    let list_plaintext: String = list_as_plaintext()?;
    let list_json: serde_json::Value = serde_json::from_str(&list_plaintext)?;
    return Ok(list_json);
}

fn add_application(application: &String) {
    // a tribute to ali
    // let graphics: &str = "good";
    let list_path: PathBuf = list_location();
    let mut list_jsonvalue: serde_json::Value = list_as_jsonvalue()
        .expect("Failed to get list as JSON value");
    let list_jsonvalue_vec: &mut Vec<serde_json::Value> = list_jsonvalue
        .as_array_mut()
        .expect("Failed to get list as Vec");
    let new_list_jsonvalue_vec: &mut Vec<serde_json::Value> = list_jsonvalue_vec;
    new_list_jsonvalue_vec.push(serde_json::json!(application));
    let new_list_jsonvalue: serde_json::Value =
        serde_json::Value::from_iter(new_list_jsonvalue_vec.iter().cloned());
    let new_list: String = serde_json::Value::to_string(&new_list_jsonvalue);
    fs::write(list_path, new_list)
        .expect("Failed to write application to list");
    println!("Added application {}", application);
}

fn remove_application(application: &String) {
    let list_path: PathBuf = list_location();
    let mut list_jsonvalue: serde_json::Value = list_as_jsonvalue()
        .expect("Failed to get list as JSON value");
    let list_jsonvalue_vec: &mut Vec<serde_json::Value> = list_jsonvalue
        .as_array_mut()
        .expect("Failed to get list as Vec");
    let new_list_jsonvalue_vec: &mut Vec<serde_json::Value> = list_jsonvalue_vec;
    new_list_jsonvalue_vec.retain(|item| item != &serde_json::json!(application));
    let new_list_jsonvalue: serde_json::Value =
        serde_json::Value::from_iter(new_list_jsonvalue_vec.iter().cloned());
    let new_list: String = serde_json::Value::to_string(&new_list_jsonvalue);
    fs::write(list_path, new_list)
        .expect("Failed to remove application from list");
    println!("Removed application {}", application);
}

fn display_list() {
    let list_plaintext: String = list_as_plaintext()
        .expect("Could not get list as plaintext");
    println!("{}", list_plaintext);
}

fn enforce_list() {
    let mut list_jsonvalue: serde_json::Value = list_as_jsonvalue()
        .expect("Failed to get list as JSON value");
    let list_jsonvalue_vec: &mut Vec<serde_json::Value> = list_jsonvalue
        .as_array_mut()
        .expect("Failed to get list as Vec");
    for application_jsonvalue in list_jsonvalue_vec {
        let application_plaintext: &str = application_jsonvalue
            .as_str()
            .expect("Failed to get application as string")
            .trim_matches('"');
        let enforcement: std::process::ExitStatus = Command::new("flatpak")
            .arg("override")
            .arg("--user")
            .arg("--nosocket=wayland")
            .arg("--socket=x11")
            .arg(application_plaintext)
            .status()
            .expect("Failed enforce list");
        if !enforcement.success() {
            eprintln!("Failed to enforce {}", application_plaintext);
        }
    }
    println!("Enforced list");
}

fn unenforce_list() {
    let mut list_jsonvalue: serde_json::Value = list_as_jsonvalue()
        .expect("Failed to get list as JSON value");
    let list_jsonvalue_vec: &mut Vec<serde_json::Value> = list_jsonvalue
        .as_array_mut()
        .expect("Failed to get list as Vec");
    for application_jsonvalue in list_jsonvalue_vec {
        let application_plaintext: &str = application_jsonvalue
            .as_str()
            .expect("Failed to get application as string")
            .trim_matches('"');
        let unenforcement: std::process::ExitStatus = Command::new("flatpak")
            .arg("override")
            .arg("--user")
            .arg("--socket=wayland")
            .arg("--nosocket=x11")
            .arg(application_plaintext)
            .status()
            .expect("Failed unenforce list");
        if !unenforcement.success() {
            eprintln!("Failed to unenforce {}", application_plaintext);
        }
    }
    println!("Unenforced list");
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Add { application: String },
    Remove { application: String },
    Enforce {},
    Unenforce {},
    List {}
}

fn main() -> Result<(), std::io::Error> {
    let cli: Cli = Cli::parse();
    match &cli.command {
        Commands::Add { application } => {
            add_application(application);
        },
        Commands::Remove { application } => {
            remove_application(application);
        },
        Commands::Enforce {} => {
            enforce_list();
        },
        Commands::Unenforce {} => {
            unenforce_list();
        },
        Commands::List {} => {
            display_list();
        }
    }

    return Ok(());
}
