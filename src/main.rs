pub mod analyze;
pub mod ast;
pub mod compile;

use ast::*;
use compile::*;
use lalrpop_util::lalrpop_mod;

use clap::Parser;
use serde::Serialize;
use std::fs;
use std::path;

lalrpop_mod!(pub bithoven); // synthesized by LALRPOP

/// A simple program to demonstrate argument parsing
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to file to compile
    #[arg(short, long)]
    path: String,
}

// Define the data structure you want to write to JSON
#[derive(Serialize)]
struct BithovenOutput {
    asm: String,
    hex: String,
    bytes: Vec<u8>,
}

fn main() {
    let args = Args::parse();

    let bithoven = read_bithoven(&args.path);
    // UTXO: stack + scripts - bitcoin HTLC
    let utxo: Bithoven = bithoven::BithovenParser::new().parse(&bithoven).unwrap();

    let script = compile(utxo.output_script.clone(), &utxo.pragma.target);

    // 1. Create an instance of your struct
    let output: BithovenOutput = BithovenOutput {
        hex: bitcoin::Script::from_bytes(&script).to_hex_string(),
        asm: bitcoin::Script::from_bytes(&script).to_asm_string(),
        bytes: bitcoin::Script::from_bytes(&script).to_bytes(),
    };

    // 2. Get the file name using path
    let file_name = path::Path::new(&args.path)
        .file_name()
        .expect("Incorrect file name for Bithoven")
        .to_str()
        .expect("Incorrect file name for Bithoven")
        .to_owned();

    // 3. Create a file to write to.
    let file = fs::File::create(file_name + ".json").unwrap();

    serde_json::to_writer_pretty(file, &output).unwrap();

    println!("PRAGMA: {:?}", utxo.pragma);
    println!("STACK: {:?}", utxo.input_stack);
    println!("AST: {:?}", utxo.output_script);

    println!("{:?}", bitcoin::Script::from_bytes(&script).to_asm_string());
}

fn read_bithoven(file_path: &String) -> String {
    // Attempt to read the file
    let bytes = fs::read(file_path).expect("Not Bithoven file.");

    str::from_utf8(&bytes)
        .expect("Bithoven file should consist of utf8.")
        .to_string()
}
