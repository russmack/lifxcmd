# lifxcmd

A Rust command line program for controlling a LIFX light bulb.

[![Build Status](https://travis-ci.org/russmack/lifxcmd.svg?branch=master)](https://travis-ci.org/russmack/lifxcmd)

---
#### Status: Usable.
----

## Build
```
git clone https://github.com/russmack/lifxcmd.git
cargo build
```

## Usage
```
./lifxcmd -h

OPTIONS:
    -a, --address <HOST ADDRESS>            Specifies the address of the target device
    -c, --colour <COLOUR NAME>              Changes the colour
    -d, --duration <TRANSITION DURATION>    The duration of the colour transition
    -f, --flash <FLASH COLOUR NAME>         Specifies the name of the colour to flash
    -i, --interval <FLASH INTERVAL>         The length of the flash
    -s, --state                             Show the current state of the device
```
Examples
```
Change colour to slate_gray instantly:
./lifxcmd -c slate_gray

A three second flash to coral:
./lifxcmd -f coral -i 3000
```

## Features

- [X] Locate device
- [X] Specify device
- [X] Change device colour
- [X] Specify duration of colour transition
- [X] Flash another colour
- [X] Specify length of flash interval
- [ ] Print current state of device
- [ ] List supported colour names
- [ ] Use RGB to specify colours

## License
BSD 3-Clause: [LICENSE.txt](LICENSE.txt)

[<img alt="LICENSE" src="http://img.shields.io/pypi/l/Django.svg?style=flat-square"/>](LICENSE.txt)
