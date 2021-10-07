#![warn(rust_2018_idioms, missing_debug_implementations)]
use structopt::StructOpt;
use std::path::PathBuf;
use std::convert::TryInto;

use freqm::*;
use freqm::ne_repeater::*;

#[derive(Debug, StructOpt)]
struct FreqmOpts {
    #[structopt(subcommand)]
    command: FreqmCmd,
}

#[derive(Debug, StructOpt)]
enum FreqmCmd {
    /// list supported radio models with the features we support for them
    Models {

    },
    
    NeCsv { 
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = FreqmOpts::from_args();

    match opt.command {
        FreqmCmd::NeCsv { file } => {
            let mut csv = csv::ReaderBuilder::new()
                .has_headers(false)
                .flexible(true)
                .from_path(file)?;

            for r in csv.records() {
                let r = r?;
                println!("{:?}", r);
                let record: NeRepeaterRecord = r.try_into()?;

                println!("{:?}", record); 

                let r2: Repeater = record.try_into()?;

                println!("{:?}", r2);

            }
        },
        FreqmCmd::Models { } => {
            todo!("list-models");
        }
    }

    Ok(())
}
