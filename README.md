# brightctl

<b>brightctrl</b> is a Rust based version of [brightnessctl](https://github.com/Hummer12007/brightnessctl) and as such closely mimics its behavior. The application allows the user to read and control device brightness via systemd.

I wrote this application as an exploration / exercise, without the intention of having identical functionality.  It has fewer options and the CLI is slightly more strict when setting brightness for a given device.  

## FAQ

#### <b>Why use it over brightnessctl?</b>

Well, technically it's a bit more performant. Here's a comparison using [hyperfine](http://github.com/sharkdp/hyperfine).

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `brightctl set 50%` | 2.6 ± 0.2 | 2.3 | 4.6 | 1.00 |
| `brightnessctl set 50%` | 3.0 ± 0.2 | 2.7 | 4.9 | 1.15 ± 0.09 |
|:---|---:|---:|---:|---:|
| `brightctl -s` | 0.9 ± 0.1 | 0.7 | 1.6 | 1.00 |
| `brightnessctl -s` | 1.2 ± 0.1 | 1.0 | 2.2 | 1.31 ± 0.17 |
|:---|---:|---:|---:|---:|
| `brightctl -r` | 2.0 ± 0.1 | 1.7 | 3.6 | 1.00 |
| `brightnessctl -r` | 2.3 ± 0.1 | 2.0 | 3.5 | 1.15 ± 0.11 |

The performance is otherwise identical when using the `--list` options in either, so results are simply omitted.

Though 3-4ms faster on average is small, it does translate as a 13-25% improvement, depending on the command.

You could argue the difference is pointless but... it <i>IS</i> faster.

---


## Installation
Clone the repository and build the program yourself.
Unlike <b>brightnessctl</b>, this application only works for systemd/login.

I may eventually create releases for different distributions if I get to feeling froggy.


## Usage

```
Device brightness control for systemd

Usage: brightctl [OPTIONS] [COMMAND]

Commands:
  info  Get device info
  get   Get brightness of current device
  max   Get maximum brightness of current device
  set   Set brightness for current device.
  help  Print this message or the help of the given subcommand(s)

Options:
  -l, --list              List devices with available brightness controls
  -m, --machine-readable  Produce machine-readable output
  -s, --save              Save previous state to a temporary file
  -r, --restore           Restore previously saved state
  -d, --device <DEVICE>   Specify Device Name
  -c, --class <CLASS>     Specify Class Name [default: backlight leds]
  -h, --help              Print help
  -V, --version           Print version
```