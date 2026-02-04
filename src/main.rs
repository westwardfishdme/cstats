use crate::{args::*, operations::statistics::StatData};
use clap::Parser;
use clap_stdin::FileOrStdin;

mod args;
mod operations;

enum FileType {
    Csv,
    Plaintext,
    None,
}

fn get_data(in_file: FileOrStdin, ftype: FileType) -> Option<Box<[f64]>> {
    let mut data: Vec<f64> = Vec::new();

    let mut file = in_file.contents().ok()?;

    let delimit = match ftype {
        FileType::Csv => {
            file = file.replace(",", " ");
            file.split_whitespace()
        }
        FileType::Plaintext => file.split_whitespace(),
        FileType::None => return None,
    };

    for line in delimit {
        let value = match line.parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        data.push(value);
    }

    if data.len() == 0 {
        return None;
    }
    let _box: Box<[f64]> = data.into_boxed_slice();
    Some(_box)
}

fn parsefile(f: ArgFile, ftype: FileType) -> Result<StatData, &'static str> {
    let in_file = get_data(f.dataset, ftype);
    if in_file.is_none() {
        return Err("Cannot parse the file!");
    }
    let sd = StatData::new(in_file.unwrap());
    Ok(sd)
}

// parse arguments
fn parse_stats(s: &StatArgs) -> Result<(), &'static str> {
    // check the filetype
    let ftype = match s.in_format.to_lowercase().as_str() {
        "txt" => FileType::Plaintext,
        "csv" => FileType::Csv,
        _ => FileType::None,
    };

    //determine the printed output
    match s.subcommands.to_owned() {
        StatSubcommands::All(f) => {
            let sd = parsefile(f, ftype)?;
            println!("{}", sd)
        }
        StatSubcommands::Sum(f) => {
            let sd = parsefile(f, ftype)?;
            println!("sum = {}", sd.sum);
        }
    };
    Ok(())
}

// main
fn main() -> Result<(), &'static str> {
    let args = BaseArgs::parse();

    match args.subcommands {
        StatsCommands::Stats(statarg) => {
            parse_stats(&statarg)?;
        }
    }
    Ok(())
}
