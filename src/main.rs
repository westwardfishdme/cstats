use crate::{args::*, operations::statistics::StatData};
use clap::Parser;
use clap_stdin::FileOrStdin;

mod args;
mod operations;

#[derive(PartialEq)]
enum FileType {
    Csv,
    Plaintext,
}

#[doc(hidden)]
fn get_data(in_file: FileOrStdin, ftype: FileType) -> Result<Box<[f64]>, &'static str> {
    //! Parses and reads a file for floating point integers.
    let mut data: Vec<f64> = Vec::new();

    let mut file = match in_file.contents() {
        Ok(v) => v,
        Err(_) => return Err("File could not be read!"),
    };

    let delimit = match ftype {
        // used a match here just incase future implementations
        // allow different delimiters. However, using cut || awk
        // would be fine as well.
        FileType::Csv => {
            file = file.replace(",", " ");
            file.split_whitespace()
        }
        FileType::Plaintext => file.split_whitespace(),
    };

    for line in delimit {
        if ftype != FileType::Csv && line.contains(",") {
            eprintln!("It looks like you are trying to parse a .csv file as a plaintext file!");
            eprintln!(
                "Please re-run with the following:\n\ncstats stats -i=csv <command> <file>\n"
            );
            return Err("Bad Filetype");
        }
        let value = match line.parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        data.push(value);
    }

    if data.len() == 0 {
        return Err("No data was able to be stored!");
    }
    let _box: Box<[f64]> = data.into_boxed_slice();
    Ok(_box)
}

#[doc(hidden)]
fn parsefile(f: ArgFile, ftype: FileType) -> Result<StatData, &'static str> {
    //! Wrapper for getting statistics data.
    let in_file = match get_data(f.dataset, ftype) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let sd = StatData::new(in_file);
    Ok(sd)
}

// parse arguments
#[doc(hidden)]
fn parse_stats(s: &StatArgs) -> Result<(), &'static str> {
    //! Interprets user arguments and determines the output.
    // check the filetype
    let ftype = match s.in_format.to_lowercase().as_str() {
        "txt" => FileType::Plaintext,
        "csv" => FileType::Csv,
        _ => return Err("Not a valid filetype!"),
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

    // more arguments may be added depending on
    // my need. Feel free to fork and add on if
    // necessary.
    match args.subcommands {
        StatsCommands::Stats(statarg) => {
            parse_stats(&statarg)?;
        }
    }
    Ok(())
}
