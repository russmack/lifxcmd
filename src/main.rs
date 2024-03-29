extern crate clap;
extern crate rustylifx;
extern crate termcolor;

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::thread;
use std::time::Duration;

use clap::{App, Arg};
use termcolor::Color;

use rustylifx::colour::{self, Hsb};
use rustylifx::network::Device;
use rustylifx::{messages, network, response};

pub mod cli;

const BIN_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    print_program_header();

    // Configure flags.
    let matches = App::new("Lifx Command")
        .version(BIN_VERSION)
        .author("Russell Mackenzie")
        .about("Control Lifx devices from the command line.")
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("HOST ADDRESS")
                .help("Specifies the address of the target device")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("subnet")
                .short("n")
                .long("subnet")
                .value_name("SUBNET")
                .help("Specify the device subnet")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("power")
                .short("p")
                .long("power")
                .value_name("POWER LEVEL")
                .help("Changes the power level on/off")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("colour")
                .short("c")
                .long("colour")
                .value_name("COLOUR NAME")
                .help("Changes the colour")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("flash")
                .short("f")
                .long("flash")
                .value_name("FLASH COLOUR NAME")
                .help("Specifies the name of the colour to flash")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .value_name("FLASH INTERVAL")
                .help("The length of the flash")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("duration")
                .short("d")
                .long("duration")
                .value_name("TRANSITION DURATION")
                .help("The duration of the colour transition")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("report")
                .short("r")
                .long("report")
                .value_name("DISPLAY CURRENT STATE")
                .help("Display the current state of the device")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("hue")
                .short("h")
                .long("hue")
                .value_name("HUE")
                .help("Set the hue of the device")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("saturation")
                .short("s")
                .long("saturation")
                .value_name("SATURATION")
                .help("Set the saturation of the device")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("brightness")
                .short("b")
                .long("brightness")
                .value_name("BRIGHTNESS")
                .help("Set the brightness of the device")
                .takes_value(true),
        )
        .get_matches();

    let subnet = match matches.value_of("subnet").unwrap_or("") {
        "" => {
            cli::print_info_sending("No subnet specified, defaulting to 192.168.1.255.");
            Ipv4Addr::new(192, 168, 1, 255)
        }
        ip => match ip.parse() {
            Ok(v) => v,
            Err(e) => {
                cli::exit_error(&format!("Invalid subnet ipv4 address: {}", e));
                return;
            }
        },
    };

    // Find the device, by flag, else broadcast.
    let device = match matches.value_of("address").unwrap_or("") {
        "" => {
            // Locate device.
            cli::print_info_sending("Locating device...");
            match messages::get_service(subnet) {
                Ok(v) => v,
                Err(e) => {
                    cli::exit_error(&format!("Failed finding device: {}", e));
                    return;
                }
            }
        }
        ip => {
            // Set device.
            const PORT: u16 = 56700;
            network::Device {
                socket_addr: format!("{}:{}", ip, PORT)
                    .parse()
                    .expect("invalid socket address"),
                response: None,
            }
        }
    };

    // Check if state display was specified.
    if matches.is_present("report") {
        cli::print_info_sending("Requesting device status report...");
        let device = get_device_state(&device);
        display_device_state(&device);
        return;
    }

    // Set the power level on/off.
    if let Some(v) = matches.value_of("power") {
        let res = match v {
            "on" => {
                cli::print_info_sending("Setting device power to on...");
                messages::set_device_on(&device)
            }
            "off" => {
                cli::print_info_sending("Setting device power to off...");
                messages::set_device_off(&device)
            }
            _ => {
                cli::exit_usage("Power state is invalid, should be on or off.");
                return;
            }
        };

        if res.is_err() {
            cli::exit_error(&format!(
                "Failed setting device power state: {:?}",
                res.err()
            ));
            return;
        }
    };

    // Check if transition duration was specified.
    let duration = match matches.value_of("duration") {
        Some(v) => match v.parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                cli::exit_usage(&format!("Duration is not a valid number: {}", e));
                return;
            }
        },
        None => 0,
    };

    // Set the colour by name if flag exists.
    if let Some(v) = matches.value_of("colour") {
        cli::print_info_sending("Setting device colour...");
        let _ = messages::set_device_state(&device, &colour::get_colour(v), 1000, duration);
    }

    // Set the colour by HSB if specified.
    // HSB: 360º, 100%, 100%
    let mut hue = match matches.value_of("hue") {
        Some(v) => match v.parse::<i16>() {
            Ok(n) => {
                if (0..=360).contains(&n) {
                    cli::print_info_sending("Setting device hue...");
                    n
                } else {
                    cli::exit_usage("Hue is outside the valid range, should be 0 - 360 (degrees)");
                    return;
                }
            }
            Err(e) => {
                cli::exit_usage(&format!("Hue is not a valid number: {}", e));
                return;
            }
        },
        None => -1,
    };

    let mut saturation = match matches.value_of("saturation") {
        Some(v) => match v.parse::<i16>() {
            Ok(n) => {
                if (0..=100).contains(&n) {
                    cli::print_info_sending("Setting device saturation...");
                    n
                } else {
                    cli::exit_usage(
                        "Saturation is outside the valid range, should be 0 - 100 (percent)",
                    );
                    return;
                }
            }
            Err(e) => {
                cli::exit_usage(&format!("Saturation is not a valid number: {}", e));
                return;
            }
        },
        None => -1,
    };

    let mut brightness = match matches.value_of("brightness") {
        Some(v) => match v.parse::<i16>() {
            Ok(n) => {
                if (0..=100).contains(&n) {
                    cli::print_info_sending("Setting device brightness...");
                    n
                } else {
                    cli::exit_usage(
                        "Brightness is outside the valid range, should be 0 - 100 (percent)",
                    );
                    return;
                }
            }
            Err(e) => {
                cli::exit_usage(&format!("Brightness is not a valid number: {}", e));
                return;
            }
        },
        None => -1,
    };

    if hue >= 0 || saturation >= 0 || brightness >= 0 {
        if hue < 0 {
            hue = 360
        }
        if saturation < 0 {
            saturation = 100
        }
        if brightness < 0 {
            brightness = 100
        }

        let _ = messages::set_device_state(
            &device,
            &colour::Hsb {
                hue: hue as u16,
                saturation: saturation as u8,
                brightness: brightness as u8,
            },
            1000,
            duration,
        );
    }

    // Check if the flash interval was specified.
    let interval = match matches.value_of("interval") {
        Some(v) => match v.parse::<u64>() {
            Ok(n) => n,
            Err(e) => {
                cli::exit_usage(&format!("Interval is not a valid number: {}", e));
                return;
            }
        },
        None => 1000,
    };

    // Flash if flag exists.
    if let Some(v) = matches.value_of("flash") {
        cli::print_info_sending("Flashing device to another colour...");
        flash(&device, colour::get_colour(v), interval);
    };

    // TODO: ponder fade
    // let fade_len: u32 = 3000;
    // fade(&device, colour::get_colour("crimson"), fade_len);
    // thread::sleep(Duration::from_millis((fade_len + 1) as u64));
    // Fade device back to initial state.
    // fade(&device, initial_state.unwrap(), 3000);

    cli::exit_done("");
}

fn print_program_header() {
    //let icon_hex = 0x2518;
    let icon_hex = 0x0f04;
    let icon = format!("{}", std::char::from_u32(icon_hex).unwrap_or('�'));

    println!();
    cli::print_string("-----------------------------", Color::Green, false);
    cli::print_line_info_prefix(
        &icon,
        "Lifxcmd version",
        &format!("{}\n", BIN_VERSION),
        Color::Magenta,
        Color::Green,
    );
    cli::print_string("-----------------------------\n", Color::Green, false);
}

fn get_device_state(device: &Device) -> Device {
    // TODO: sort out this hacky sleep.
    thread::sleep(Duration::from_millis(1000));
    messages::get_device_state(device).unwrap()
}

fn flash(device: &Device, flash_colour: Hsb, duration_ms: u64) {
    cli::print_info_sending("Getting current colour...");
    let device = get_device_state(device);

    // Extract current HSVK from device state data.
    let resp = match device.response {
        Some(ref v) => v,
        None => panic!("no response"),
    };

    let payload = match resp.payload {
        response::Payload::State(ref v) => Some(v),
        _ => None,
    };

    // TODO BUG:
    // Expected after flash:
    // Current hue                   : 21845
    // Current hue degrees           : 120º
    // Current saturation            : 0
    // Current saturation percent    : 0%
    // Current brightness            : 22937
    // Current brightness percent    : 34%
    // Current kel                   : 2500
    //
    // Actual after flash:
    // Current hue                   : 43690
    // Current hue degrees           : 240º
    // Current saturation            : 0
    // Current saturation percent    : 0%
    // Current brightness            : 20315
    // Current brightness percent    : 30%
    // Current kel                   : 2500
    let initial_state = match payload {
        Some(v) => {
            let h = colour::hue_word_to_degrees(v.hsbk.hue);
            let s = colour::saturation_word_to_percent(v.hsbk.saturation);
            let b = colour::brightness_word_to_percent(v.hsbk.brightness);
            Some(colour::Hsb::new(h, s, b))
        }
        None => None,
    };

    if let Some(v) = initial_state {
        // Change device state temporarily.
        cli::print_info_sending("Flashing new colour...");
        let _ = messages::set_device_state(&device, &flash_colour, 2500, 0);
        thread::sleep(Duration::from_millis(duration_ms));

        // Return device to initial state.
        cli::print_info_sending("Setting colour back...");
        let _ = messages::set_device_state(&device, &v, 2500, 0);
    }
}

fn display_device_state(device: &Device) {
    let resp = match device.response {
        Some(ref v) => v,
        None => {
            println!("No response from device.");
            return;
        }
    };

    let mut device_state: HashMap<&str, String> = HashMap::new();
    device_state.insert("Source", format!("{}", resp.source));
    device_state.insert("Mac addr", resp.mac_address.to_string());
    device_state.insert("Firmware", resp.firmware.to_string());
    device_state.insert("Size", format!("{}", resp.size));
    device_state.insert("Sequence num", format!("{}", resp.sequence_number));
    device_state.insert("Reserved_1", format!("{}", resp.reserved_1));
    device_state.insert("Reserved_2", format!("{}", resp.reserved_2));
    device_state.insert("Message type", format!("{}", resp.message_type));

    match resp.payload {
        response::Payload::StateService(ref v) => {
            device_state.insert("Service", format!("{}", v.service));
            device_state.insert("Port", format!("{}", v.port));
            device_state.insert("Unknown", v.unknown.to_string());
        }
        response::Payload::State(ref v) => {
            device_state.insert("Current hue", format!("{:?}", v.hsbk.hue));
            device_state.insert(
                "Current hue degrees",
                format!("{:?}º", colour::hue_word_to_degrees(v.hsbk.hue)),
            );
            device_state.insert("Current saturation", format!("{:?}", v.hsbk.saturation));
            device_state.insert(
                "Current saturation percent",
                format!(
                    "{:?}%",
                    colour::saturation_word_to_percent(v.hsbk.saturation)
                ),
            );
            device_state.insert("Current brightness", format!("{:?}", v.hsbk.brightness));
            device_state.insert(
                "Current brightness percent",
                format!(
                    "{:?}%",
                    colour::brightness_word_to_percent(v.hsbk.brightness)
                ),
            );
            device_state.insert("Current kel", format!("{:?}", v.hsbk.kelvin));
        }
        ref v => {
            device_state.insert("Unrecognised response", format!("{:?}", v));
        }
    };

    let state_report = cli::format_device_state(&device_state);

    println!("{}", &state_report);
}
