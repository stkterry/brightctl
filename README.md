# brightctl

<b>brightctrl</b> is a Rust based version of [brightnessctl](https://github.com/Hummer12007/brightnessctl) and as such closely mimics its behavior. The application allows the user to read and control device brightness via systemd.

I wrote this application as an exploration / exercise, without the intention of having identical functionality.  It has fewer options and the CLI is slightly more strict when setting brightness for a given device.  

The current version is missing the save/store flags to track and restore brightness after closing screen lids, rebooting, etc.  I'll likely add this shortly, but who knows.

## Installation
Clone the repository and build the program yourself.
Unlike <b>brightnessctl</b>, this application currently expects systemd/logind to be present.

I may eventually create releases for different distributions if I get to feeling froggy.

