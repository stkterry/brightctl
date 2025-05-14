use std::{fs::{read_to_string, DirEntry}, path::Path};

use clap::{builder::OsStr, Parser, Subcommand};
use anyhow::{Error, Result};

#[inline(always)]
fn percent(top: u32, bottom: u32) -> u8 {
    (top as f32 / bottom as f32 * 100f32) as u8
}

struct Device<'a> {
    id: DirEntry,
    class: &'a str,
    brightness: u32,
    max_brightness: u32,
}


impl <'a>Device<'a> {

    #[inline]
    fn print_human(device: &Self) {
        let Self { id, class, brightness, max_brightness } = device;
        println!(
            "Device '{}' of class '{}':
            Brightness: {} ({}%)
            Max Brightness: {}\n",
            id.file_name().to_str().unwrap(), class, 
            brightness, percent(*brightness, *max_brightness), 
            max_brightness
        );
    }

    #[inline]
    fn print_machine(device: &Self) {
        let Self { id, class, brightness, max_brightness } = device;
        println!(
            "{},{},{},{}%,{}",
            id.file_name().to_str().unwrap(), class, 
            brightness, percent(*brightness, *max_brightness), 
            max_brightness
        );
    }
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
    let path = Path::new("/sys/class");

    let args = Arguments::parse();

    let classes = match args.class {
        Some(ref class) => vec![class.to_str().unwrap()],
        None => vec!["backlight", "leds"]
    };

    let printfn = match args.machine_readable {
        false => Device::print_human,
        true => Device::print_machine,
    };


    if args.list {

        let cnt: u16 = classes.into_iter().flat_map(|class| {
            match path.join(class).read_dir() {
                Ok(dir) => Some((class, dir)),
                Err(_) => None
            }
        }).fold(0u16, |cnt, (class, path)| {
            path.into_iter()
                .flat_map(|entry| read_device(class, entry?))
                .inspect(printfn)
                .count() as u16 + cnt
        });

        match (cnt == 0, args.class) {
            (false, _)  => {},
            (_, None) => println!("Failed to read any devices."),
            (_, Some(class)) => println!("Failed to read any devices of class {:?}", class),
        }

        return
    }

    // If a device was specified, find the corresponding entry by comparing the directory name
    // otherwise return the first valid device
    let device = if let Some(id) = args.device {
        classes.into_iter().find_map(|class| {
            path.join(class)
                .read_dir().ok()?
                .flatten()
                .find(|entry| entry.file_name().as_os_str() == id)
                .and_then(|entry| read_device(class, entry).ok())
        })
    } else {
        classes.into_iter().find_map(|class| {
            path.join(class)
                .read_dir().ok()?
                .flatten()
                .find_map(|entry| read_device(class, entry).ok())
        })
    };


    if let Some(dev) = device {
        printfn(&dev);
    }

}


#[inline]
fn read_device(class: &str, entry: DirEntry) -> Result<Device, Error> {

    let max_brightness = read_to_string(entry.path().join("max_brightness"))?
        .trim().parse::<u32>()?;
    
    let brightness = read_to_string(entry.path().join("brightness"))?
        .trim().parse::<u32>()?;

    Ok(Device {
        id: entry,
        class,
        brightness,
        max_brightness
    })
}
