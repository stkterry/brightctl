use std::{fs::DirEntry, path::Path};

use clap::{builder::OsStr, Parser, Subcommand};



struct Device<'a> {
    id: DirEntry,
    class: &'a str,
    brightness: u32,
    max_brightness: u32,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get device info.
    Info,
    /// Get brightness of current device.
    Get,
    /// Get maximum brightness of current device.
    Max,
    /// Set brightness for current device.
    Set { value: OsStr }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {

    /// List devices with available brightness controls.
    #[arg(short, long, required=false)]
    list: bool,

    /// Produce machine-readable output.
    #[arg(short, long, required=false)]
    machine_readable: bool,

    /// Specify Device Name
    #[arg(short, long, required=false)]
    device: Option<OsStr>,

    /// Specify Class Name
    #[arg(short, long, required=false)]
    class: Option<OsStr>,

    /// Commands
    #[command(subcommand)]
    command: Option<Commands>
}

fn main() {
    let path = Path::new("sys/class");

    let args = Arguments::parse();

    let classes = match args.class {
        Some(ref class) => vec![class.to_str().unwrap()],
        None => vec!["backlight", "leds"]
    };



}
