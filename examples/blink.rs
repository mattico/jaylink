//! Uses the functions for controlling individual pin states to toggle them periodically.

use jaylink::*;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    /// Serial number of the probe to connect to.
    #[structopt(long = "serial")]
    serial: Option<String>,
}

fn main() {
    let opts = Opts::from_args();
    if let Err(e) = run(opts) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run(opts: Opts) -> Result<()> {
    let mut probe = JayLink::open_by_serial(opts.serial.as_ref().map(|s| &**s))?;

    loop {
        probe.set_tms(true)?;
        probe.set_tdi(true)?;
        probe.set_reset(true)?;
        probe.set_trst(true)?;
        println!("on");
        sleep(Duration::from_millis(500));
        probe.set_tms(false)?;
        probe.set_tdi(false)?;
        probe.set_reset(false)?;
        probe.set_trst(false)?;
        println!("off");
        sleep(Duration::from_millis(500));
    }
}