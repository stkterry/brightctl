
use std::{
    ffi::c_void, 
    fs::{read_to_string, DirEntry}, 
    path::Path
};

use clap::{builder::OsStr, Parser, Subcommand};
use anyhow::{anyhow, Result};
use systemd::bus::{Bus, BusName, InterfaceName, MemberName, ObjectPath};



#[derive(Debug)]
enum Adj {
    Abs,
    Pos,
    Neg,
}

struct Value {
    _adj: Adj,
    qty: u32,
    _per: bool,
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
            brightness, device.percent_brightness(), 
            max_brightness
        );
    }

    #[inline]
    fn print_machine(device: &Self) {
        let Self { id, class, brightness, max_brightness } = device;
        println!(
            "{},{},{},{}%,{}",
            id.file_name().to_str().unwrap(), class, 
            brightness, device.percent_brightness(), 
            max_brightness
        );
    }

    #[inline(always)]
    fn percent_brightness(&self) -> u8 {
        (self.brightness as f32 / self.max_brightness as f32 * 100f32) as u8
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
    #[arg(short, long)]
    list: bool,

    /// Produce machine-readable output.
    #[arg(short, long)]
    machine_readable: bool,

    /// Specify Device Name
    #[arg(short, long)]
    device: Option<OsStr>,

    /// Specify Class Name
    #[arg(short, long)]
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
            let k = path.join(class)
                .read_dir().ok()?
                .flat_map(|dev_path| map_device(class, dev_path?))
                .count() as u16;
            Some(k)
        }).sum();

        match (cnt == 0, args.class) {
            (false, _)  => {},
            (_, None) => println!("Failed to read any devices."),
            (_, Some(ref class)) => println!("Failed to read any devices of class '{}'", class.to_str().unwrap()),
        }

        return
    }

    // If a device was specified, find the corresponding entry by comparing the directory name
    // otherwise return the first valid device
    let device = if let Some(ref id) = args.device {
        classes.into_iter().find_map(|class| {
            path.join(class)
                .read_dir().ok()?
                .flatten()
                .find(|dev_path| dev_path.file_name().as_os_str() == *id)
                .and_then(|dev_path| map_device(class, dev_path).ok())
        })
    } else {
        classes.into_iter().find_map(|class| {
            path.join(class)
                .read_dir().ok()?
                .flatten()
                .find_map(|dev_path| map_device(class, dev_path).ok())
        })
    };

    // Exit if no matching devices are found.
    let device = match (args.device, device) {
        (_, Some(d)) => d,
        (None, _) => {
            println!("Failed to find a suitable device.");
            return;
        },
        (Some(ref id), _) => {
            println!("Device '{}' not found.", id.to_str().unwrap());
            return;
        }
    };

    if let Some(cmd) = args.command { match cmd {
        Commands::Info => printfn(&device),
        Commands::Get => println!("{}", device.brightness),
        Commands::Max => println!("{}", device.max_brightness),
        Commands::Set { value } => {

            let value = match parse_value(&device, &value) {
                Ok(v) => v,
                Err(err) => { println!("{err}"); return }
            };

            match set_brightness(&device, &value) {
                Ok(_) => {},
                Err(err) => { println!("{err}"); return }
            }
        },
    }}

}

#[inline]
fn parse_value(device: &Device, value: &OsStr) -> Result<Value> {

    let str = value.to_str()
        .ok_or_else(|| anyhow!("Value is not valid unicode."))?;

    let edx = *&str.len();
    let (adj, sdx) = match &str[0..1] {
        "+" => (Adj::Pos, 1),
        "-" => (Adj::Neg, 1),
        _ => (Adj::Abs, 0),
    };

    let (edx, per) = match &str[edx-1..edx] {
        "%" => (edx - 1, true),
        _ => (edx, false)
    };

    let val: u32 = str[sdx..edx].parse()?;

    let mb = device.max_brightness;
    let b = device.brightness;
    use Adj::*;
    let qty = match (per, &adj) {
        (true,  Abs) => mb.min((val * mb) / 100),
        (true,  Pos) => mb.min(b + ((val * mb) / 100)),
        (true,  Neg) => b - ((val * mb) / 100),
        (false, Abs) => mb.min(val),
        (false, Pos) => mb.min(b + val),
        (false, Neg) => b - val,
    };

    Ok(Value { _adj: adj, qty, _per: per })

}


#[inline]
fn map_device(class: &str, dev_path: DirEntry) -> Result<Device> {

    let max_brightness = read_to_string(dev_path.path().join("max_brightness"))?
        .trim().parse::<u32>()?;
    
    let brightness = read_to_string(dev_path.path().join("brightness"))?
        .trim().parse::<u32>()?;

    Ok(Device {
        id: dev_path,
        class,
        brightness,
        max_brightness
    })
}


#[inline]
fn set_brightness(device: &Device, value: &Value) -> Result<()> {

    let (dest, path, interface, member) = unsafe {(
        BusName::from_bytes_unchecked(b"org.freedesktop.login1\0"),
        ObjectPath::from_bytes_unchecked(b"/org/freedesktop/login1/session/auto\0"),
        InterfaceName::from_bytes_unchecked(b"org.freedesktop.login1.Session\0"),
        MemberName::from_bytes_unchecked(b"SetBrightness\0"),
    )};
    
    let mut msg = Bus::default_system()?
        .new_method_call(dest, path, interface, member)?;
    
    
    unsafe { 
        msg.append_basic_raw(115, device.class.to_owned().as_ptr() as *const c_void)?;
        msg.append_basic_raw(115, device.id.file_name().to_string_lossy().as_ptr() as *const c_void)?;
    };
    msg.append(value.qty)?;
    
    let _ = msg.call(0);
     
    Ok(())
}