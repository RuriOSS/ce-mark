/*
 * Now I scream. WTH is this QwQ?
 * Don't blame me QwQ, all rust code is written by LLMs,
 * and I have never learned rust in fact.
 */
mod debug;
mod lineno;
mod linter;
mod nautilus;
mod scmp;
use clap::{Parser, Subcommand};
use colored::*;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[derive(Parser)]
#[command(name = "cwte")]
#[command(version = "0.1.0")]
#[command(about = "Cwte")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gen { input: String, output: String },
    Scmp { input: String, output: String },
}

fn cwte_generator(input: &str, output: &str) {
    let input_file = fs::File::open(input).expect("Failed to open input file");
    // Process the input file with prepare layer, and get the memfd file.
    let mut mfd_file = lineno::clang_format_prepare_layer(input_file);
    mfd_file = lineno::prepare_layer(mfd_file);
    // Process the input file with nautilus layer, and get the memfd file.
    mfd_file = nautilus::nautilus_layer(mfd_file, input);
    // Process the memfd file with linter layer, and get the new memfd file.
    mfd_file = linter::linter_layer(mfd_file, input);
    // Process the memfd file with final layer, and get the new memfd file.
    mfd_file = lineno::final_layer(mfd_file);
    // Format the output with clang_format_final_layer.
    mfd_file = lineno::clang_format_final_layer(mfd_file);
    // Write the content of memfd to the output file.
    let mut output_file = fs::File::create(&output).expect("Failed to create output file");
    let mut memfd_content = Vec::new();
    mfd_file
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek memfd");
    mfd_file
        .read_to_end(&mut memfd_content)
        .expect("Failed to read memfd");
    output_file
        .write_all(&memfd_content)
        .expect("Failed to write to output file");
    println!(
        "{}{}",
        "\nCwte processing completed. Output written to ".green(),
        output.blue()
    );
    println!(
        "{}{}",
        "I hope I'm just a cute tail ".green(),
        "::::<".yellow()
    );
}
fn scmp_generator(input: &str, output: &str) {
    let input_file = fs::File::open(input).expect("Failed to open input file");
    // Process the input file with prepare layer, and get the memfd file.
    let mut mfd_file = lineno::clang_format_prepare_layer(input_file);
    mfd_file = lineno::prepare_layer(mfd_file);
    // Process the input file with scmp layer, and get the memfd file.
    mfd_file = scmp::scmp_layer(mfd_file, input);
    // Process the memfd file with final layer, and get the new memfd file.
    mfd_file = lineno::final_layer(mfd_file);
    // Format the output with clang_format_final_layer.
    mfd_file = lineno::clang_format_final_layer(mfd_file);
    // Write the content of memfd to the output file.
    let mut output_file = fs::File::create(&output).expect("Failed to create output file");
    let mut memfd_content = Vec::new();
    mfd_file
        .seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek memfd");
    mfd_file
        .read_to_end(&mut memfd_content)
        .expect("Failed to read memfd");
    output_file
        .write_all(&memfd_content)
        .expect("Failed to write to output file");
    println!(
        "{}{}",
        "\nCwte processing completed. Output written to ".green(),
        output.blue()
    );
    println!(
        "{}{}",
        "I hope I'm just a cute tail ".green(),
        "::::<".yellow()
    );
}
fn main() {
    /*
     * We will never release any memfd file, kernel will help us do that.
     * Say thanks to the kernel, say thanks to memfd,
     * and have an ice cream.
     */
    #[cfg(debug_assertions)]
    debug::setup_panic_hook();
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} [gen|scmp] <file> <output>", args[0]);
        return;
    };

    let cli = Cli::parse();
    match cli.command {
        Commands::Gen { input, output } => {
            cwte_generator(&input, &output);
        }
        Commands::Scmp { input, output } => {
            scmp_generator(&input, &output);
        }
    }
}
