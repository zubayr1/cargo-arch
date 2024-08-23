use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "cargo-arch")]
#[command(about = "Custom Cargo command for generating Arch projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => create_boilerplate(name),
    }
}

fn create_boilerplate(project_name: &str) {
    let project_dir = Path::new(project_name);

    if project_dir.exists() {
        eprintln!("Error: Directory `{}` already exists!", project_name);
        std::process::exit(1);
    }

    // Create the directory structure
    fs::create_dir_all(project_dir.join("program/src"))
        .expect("Failed to create program/src directory");
    fs::create_dir_all(project_dir.join("src")).expect("Failed to create src directory");

    // Create Cargo.toml files
    let root_cargo_toml = format!(
        r#"[workspace]
members = [
    "program",
]

[package]
name = "{0}-program"
version = "0.1.0"
edition = "2021"

[dependencies]
common = {{ path = "../common" }}
sdk = {{ path = "../../sdk" }}

bitcoincore-rpc = "0.18.0"
hex = "0.4.3"
borsh = {{ version = "1.4.0", features = ["derive"] }}

[build-dependencies]
risc0-build = {{ version = "0.21.0" }}

[dev-dependencies]
serial_test = "3.1.1"

[package.metadata.risc0]
methods = ["program"]
"#,
        project_name
    );

    fs::write(project_dir.join("Cargo.toml"), root_cargo_toml)
        .expect("Failed to write root Cargo.toml");

    let program_cargo_toml = format!(
        r#"[package]
name = "{0}-program"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
hex = "0.4.3"
anyhow = "1.0.82"
serde_json = "1.0.108"
borsh = {{ version = "1.4.0", features = ["derive"] }}
serde = {{ version = "1.0", features = ["derive"], default-features = false  }}
bitcoin = {{ git = "https://github.com/Arch-Network/bitcoin-arch-v2" }}
risc0-zkvm = {{ version = "0.21.0", default-features = false, features = ["std"] }}
sdk = {{ path = "../sdk" }}
"#,
        project_name
    );

    fs::write(project_dir.join("program/Cargo.toml"), program_cargo_toml)
        .expect("Failed to write program Cargo.toml");

    // Create source files
    let main_rs_content = r#"fn main() {
    println!("Hello, Arch!");
}"#;

    fs::write(project_dir.join("program/src/main.rs"), main_rs_content)
        .expect("Failed to write main.rs");

    let lib_rs_content = r#"pub fn example_function() {
    println!("This is an example function in lib.rs");
}"#;

    fs::write(project_dir.join("src/lib.rs"), lib_rs_content).expect("Failed to write lib.rs");

    // Create build.rs file
    let build_rs_content = r#"use std::fs;

fn main() {
    let methods = risc0_build::embed_methods();
    println!("BUILD");
    for method in &methods {
        println!("Method Name: {}", method.name);
        println!("Method ELF: {:?}", method.elf);
    }
    for method in methods {
        let _ = fs::write(format!("target/{}.elf", method.name), method.elf);
    }
}
"#;

    fs::write(project_dir.join("build.rs"), build_rs_content).expect("Failed to write build.rs");

    println!("Created new Arch project at `{}`", project_name);
}
