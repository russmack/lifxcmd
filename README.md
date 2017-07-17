# lifxcmd

A Rust command line program for controlling a LIFX light bulb.

[![Build Status](https://travis-ci.org/russmack/lifxcmd.svg?branch=master)](https://travis-ci.org/russmack/lifxcmd)

---
#### Status: OK.
----

## Build
```
git clone https://github.com/russmack/lifxcmd.git
cargo build
```

## Usage
```
./lifxcmd -h

USAGE:
    lifxcmd [FLAGS] [OPTIONS]

FLAGS:
    -e, --echo       Display the current state of the device
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <HOST ADDRESS>            Specifies the address of the target device
    -b, --brightness <BRIGHTNESS>           Set the brightness of the device
    -c, --colour <COLOUR NAME>              Changes the colour
    -d, --duration <TRANSITION DURATION>    The duration of the colour transition
    -f, --flash <FLASH COLOUR NAME>         Specifies the name of the colour to flash
    -h, --hue <HUE>                         Set the hue of the device
    -i, --interval <FLASH INTERVAL>         The length of the flash
    -p, --power <POWER LEVEL>               Changes the power level on/off
    -s, --saturation <SATURATION>           Set the saturation of the device
```

Examples
```
Change colour to slate_gray instantly, by colour name:
./lifxcmd -c slate_gray

Change colour to green instantly by hue, saturation, brightness:
./lifxcmd -h 140 -s 100 -b 100

A three second flash to coral:
./lifxcmd -f coral -i 3000

Turn light off.
./lifxcmd -p off
```

## Features

- [X] Locate device
- [X] Specify device
- [X] Power on device
- [X] Power off device
- [X] Change device colour
- [X] Specify duration of colour transition
- [X] Flash another colour
- [X] Specify length of flash interval
- [X] Print current state of device
- [X] Use HSB to specify colours
- [ ] List supported colour names

## License
BSD 3-Clause: [LICENSE.txt](LICENSE.txt)

[<img alt="LICENSE" src="http://img.shields.io/pypi/l/Django.svg?style=flat-square"/>](LICENSE.txt)
