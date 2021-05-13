//! Converts text file to a machine-readable format

// Signal handling code from https://github.com/vorner/signal-hook/blob/master/examples/print.rs

use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
use std::error::Error;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use structopt::StructOpt;

use cyoa::fileutils::cyoafile;

#[cfg(feature = "extended-siginfo")]
type Signals =
    signal_hook::iterator::SignalsInfo<signal_hook::iterator::exfiltrator::origin::WithOrigin>;

#[cfg(not(feature = "extended-siginfo"))]
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf, // Path of input file
    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf, // Path of output file
}

fn main() -> Result<(), Box<dyn Error>> {
    let term_now = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        flag::register(*sig, Arc::clone(&term_now))?;
    }

    let args = Cli::from_args();

    let input = std::fs::read_to_string(&args.input)?;
    let mut output = std::fs::File::create(&args.output)?;

    cyoafile::compress(&input, &mut output);

    Ok(())
}
