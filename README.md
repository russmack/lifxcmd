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
```

## Features

- [X] Locate device
- [X] Specify device
- [X] Change device colour
- [X] Specify duration of colour transition
- [X] Flash another colour
- [X] Specify length of flash interval
- [ ] Print current state of device

## License
BSD 3-Clause: [LICENSE.txt](LICENSE.txt)

[<img alt="LICENSE" src="http://img.shields.io/pypi/l/Django.svg?style=flat-square"/>](LICENSE.txt)
