#![feature(fs_try_exists)]
mod iset;
mod preprocess;
mod token;
use clap::Parser;
use regex::Regex;
use std::{
    error::Error,
    fs::{self, try_exists, File},
    io::{BufWriter, Write},
};
use token::{Token, TokenGroup};

pub type Iset = iset::M1;

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
fn main() {
    let args = Args::parse();
    let filepath = args.filepath;
    if let Ok(program) = read_to_chars(&filepath) {
        match compile(program) {
            Err(errors) => {
                for error in errors {
                    eprintln!("{:?}", error);
                }
            }
            Ok(asm) => {
                let output = args.output;
                if let Err(e) = finish(asm, output) {
                    eprintln!("{}", e);
                }
            }
        }
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Syntax Error: {0}")]
    Syntax(String),
    #[error("Could not compile to binary {0}")]
    BinaryCompile(String),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to read
    #[arg(value_parser=input_arg_validator,help="brainfuck file to parse, must be a .bf file")]
    filepath: String,
    /// filepath of output asm and bin files
    #[arg(value_parser=output_arg_validator, short = 'o', default_value = "output")]
    output: String,
}

fn input_arg_validator(filepath: &str) -> Result<String, clap::Error> {
    {
        // condition one, filepath is to a brainfuck file.
        if Regex::new("\\w+\\.bf").unwrap().is_match(filepath) {
            Ok(())
        } else {
            Err(clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                format!("{} is not a valid brainfuck file.", filepath),
            ))
        }
    }?;
    {
        // condition two, filepath exists
        match try_exists(filepath) {
            Ok(true) => Ok(()),
            Ok(false) => Err(clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                format!("{} is not a valid file.", filepath),
            )),
            Err(_) => Err(clap::Error::raw(
                clap::error::ErrorKind::Io,
                format!("an error occured while attempting to read {}", filepath),
            )),
        }
    }?;
    Ok(filepath.to_string())
}

fn output_arg_validator(filename: &str) -> Result<String, clap::Error> {
    if Regex::new("\\.").unwrap().is_match(filename) {
        Err(clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            format!("{} is not a valid brainfuck file.", filename),
        ))
    } else {
        Ok(filename.to_owned())
    }
}

fn read_to_chars(filepath: &str) -> std::io::Result<Vec<char>> {
    Ok(fs::read_to_string(filepath)?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>())
}

fn compile(chars: Vec<char>) -> Result<String, Vec<Box<dyn Error>>> {
    let chars = preprocess::strip_comments(chars);
    preprocess::check_loops(&chars)?;
    let tokens = Token::proc_chars(&chars);
    let tokens = TokenGroup::group_tokens(tokens);
    let mut asm = String::new();
    let mut label_st_count = 0usize;
    let mut label_end_count = 0usize;
    asm.push_str(&Iset::init_header());
    use Token::*;
    for token_group in tokens {
        let val = token_group.ammount_merged;
        let cmd = match token_group.token {
            IncPtr => Iset::gen_inc_ptr(val),
            DecPtr => Iset::gen_dec_ptr(val),
            IncVal => Iset::gen_inc_val(val),
            DecVal => Iset::gen_dec_val(val),
            LpStart => Iset::gen_lp_start({
                let val = label_st_count;
                label_st_count += 1;
                val
            }),
            LpEnd => Iset::gen_lp_end({
                let val = label_end_count;
                label_end_count += 1;
                val
            }),
            Read => Iset::gen_read(),
            Write => Iset::gen_write(),
        };
        asm.push_str(&cmd);
    }
    asm.push_str(&Iset::init_footer());
    Ok(asm)
}

fn finish(asm: String, output: String) -> Result<(), Box<dyn Error>> {
    let filepath = format!("{}{}", &output, Iset::FILE_EX);
    BufWriter::new(File::create(filepath).unwrap()).write_all(asm.as_bytes())?;
    let mut commands = Iset::compile_to_bin_cmd(&output);
    for command in commands.iter_mut() {
        let output = command.output()?;
        if !output.status.success() {
            Err(CompilerError::BinaryCompile(
                std::str::from_utf8(&output.stderr)?.to_string(),
            ))?
        }
    }
    Ok(())
}

