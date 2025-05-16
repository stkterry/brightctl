# brightctl

<b>brightctrl</b> is a Rust based version of [brightnessctl](https://github.com/Hummer12007/brightnessctl) and as such closely mimics its behavior. The application allows the user to read and control device brightness via systemd.

I wrote this application as an exploration / exercise, without the intention of having identical functionality.  It has fewer options and the CLI is slightly more strict when setting brightness for a given device.  

The current version is missing the save/store flags to track and restore brightness after closing screen lids, rebooting, etc.  I'll likely add this shortly, but who knows.

## FAQ

#### <b>Why use it over brightnessctl?</b>

Well, technically it's a bit more performant. Here's a comparison using [hyperfine](http://github.com/sharkdp/hyperfine).  Though very conistently faster, the difference is obviously negligable... but it <i>IS</i> still faster.

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `brightctl set 50%` | 2.6 ± 0.2 | 2.3 | 4.6 | 1.00 |
| `brightnessctl set 50%` | 3.0 ± 0.2 | 2.7 | 4.9 | 1.15 ± 0.09 |

The performance is otherwise identical when using the `--list` options in either, so results are simply omitted.

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
  -d, --device <DEVICE>   Specify Device Name
  -c, --class <CLASS>     Specify Class Name [default: backlight leds]
  -h, --help              Print help
  -V, --version           Print version
```