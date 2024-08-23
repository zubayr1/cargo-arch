# Cargo-Arch
cargo-arch is a custom Cargo subcommand designed to streamline the creation of Arch project boilerplate code. This tool generates the required files and directory structure for new Arch projects with a single command.

## Features
Automatic Project Setup: Quickly set up a new Arch project with the necessary directory structure and files.
Simple Command: Use cargo-arch new <project_name> to create a new project.

## Installation Guide
### Prerequisites
Before installing cargo-arch, ensure that the following tools are installed on your system:

*Rust and Cargo*: These are essential for building and managing Rust projects. Install them from rust-lang.org.

### Installation Steps
Clone the Repository

Clone the cargo-arch repository to your local machine:

git clone <repository-url>
cd cargo-arch
Build and Install

Build and install the cargo-arch command globally using Cargo:

cargo install --path . --force
This will compile the project and install the cargo-arch executable in your Cargo bin directory (typically located at ~/.cargo/bin).

## Usage
To create a new Arch project, use the following command:

cargo-arch new <project_name>
Replace <project_name> with your desired project name. This command will create a new directory with the appropriate files and structure.

### Example Usage
For instance, to create a project named my_project, run:

cargo-arch new my_project
This will generate the following directory structure:

my_project/
├── build.rs
├── Cargo.toml
├── program/
│ ├── Cargo.toml
│ └── src/
│ └── main.rs
└── src/
└── lib.rs
