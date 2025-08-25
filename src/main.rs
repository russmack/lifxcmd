extern crate clap;
extern crate rustylifx;
extern crate termcolor;

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::thread;
use std::time::Duration;

use clap::{Arg, ArgAction, Command};
use termcolor::Color;

use rustylifx::colour::{self, Hsb};
use rustylifx::network::Device;
use rustylifx::{messages, network, response};

pub mod cli;

const BIN_VERSION: &str = env!("CARGO_PKG_VERSION");

// Configure command line arguments
fn configure_cli() -> clap::ArgMatches {
    Command::new("Lifx Command")
        .version(BIN_VERSION)
        .author("Russell Mackenzie")
        .about("Control Lifx devices from the command line.")
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .value_name("HOST ADDRESS")
                .help("Specifies the address of the target device")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("subnet")
                .short('n')
                .long("subnet")
                .value_name("SUBNET")
                .help("Specify the device subnet")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("power")
                .short('p')
                .long("power")
                .value_name("POWER LEVEL")
                .help("Changes the power level on/off")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("colour")
                .short('c')
                .long("colour")
                .value_name("COLOUR NAME")
                .help("Changes the colour")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("flash")
                .short('f')
                .long("flash")
                .value_name("FLASH COLOUR NAME")
                .help("Specifies the name of the colour to flash")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("interval")
                .short('i')
                .long("interval")
                .value_name("FLASH INTERVAL")
                .help("The length of the flash")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("duration")
                .short('d')
                .long("duration")
                .value_name("TRANSITION DURATION")
                .help("The duration of the colour transition")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("report")
                .short('r')
                .long("report")
                .value_name("DISPLAY CURRENT STATE")
                .help("Display the current state of the device")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("hue")
                .short('h')
                .long("hue")
                .value_name("HUE")
                .help("Set the hue of the device")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("saturation")
                .short('s')
                .long("saturation")
                .value_name("SATURATION")
                .help("Set the saturation of the device")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("brightness")
                .short('b')
                .long("brightness")
                .value_name("BRIGHTNESS")
                .help("Set the brightness of the device")
                .action(ArgAction::Set),
        )
        .get_matches()
}

// Parse subnet from command line arguments
fn parse_subnet(matches: &clap::ArgMatches) -> Ipv4Addr {
    match matches.get_one::<String>("subnet") {
        Some(ip) if !ip.is_empty() => ip.parse().unwrap_or_else(|e| {
            cli::exit_error(&format!("Invalid subnet ipv4 address: {}", e));
            Ipv4Addr::new(0, 0, 0, 0) // Unreachable due to exit_error
        }),
        _ => {
            cli::print_info_sending("No subnet specified, defaulting to 192.168.1.255.");
            Ipv4Addr::new(192, 168, 1, 255)
        }
    }
}

// Get device from address or broadcast
fn get_device(matches: &clap::ArgMatches, subnet: Ipv4Addr) -> network::Device {
    match matches.get_one::<String>("address") {
        Some(ip) if !ip.is_empty() => {
            const PORT: u16 = 56700;
            network::Device {
                socket_addr: format!("{}:{}", ip, PORT)
                    .parse()
                    .expect("invalid socket address"),
                response: None,
            }
        }
        _ => {
            cli::print_info_sending("Locating device...");
            messages::get_service(subnet).unwrap_or_else(|e| {
                cli::exit_error(&format!("Failed finding device: {}", e));
                network::Device {
                    socket_addr: "0.0.0.0:0".parse().unwrap(),
                    response: None,
                }
            })
        }
    }
}

// Handle device state report
fn handle_report(matches: &clap::ArgMatches, device: &network::Device) -> bool {
    if matches.get_flag("report") {
        cli::print_info_sending("Requesting device status report...");
        let device = get_device_state(device);
        display_device_state(&device);
        true
    } else {
        false
    }
}

// Handle power state changes
fn handle_power(matches: &clap::ArgMatches, device: &network::Device) -> bool {
    if let Some(v) = matches.get_one::<String>("power") {
        let res = match v.as_str() {
            "on" => {
                cli::print_info_sending("Setting device power to on...");
                messages::set_device_on(device)
            }
            "off" => {
                cli::print_info_sending("Setting device power to off...");
                messages::set_device_off(device)
            }
            _ => {
                cli::exit_usage("Power state is invalid, should be on or off.");
                return true;
            }
        };
        if res.is_err() {
            cli::exit_error(&format!("Failed setting device power state: {:?}", res.err()));
            return true;
        }
        true
    } else {
        false
    }
}

// Parse duration from command line arguments
fn parse_duration(matches: &clap::ArgMatches) -> u32 {
    matches
        .get_one::<String>("duration")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or_else(|| {
            cli::exit_usage("Duration is not a valid number");
            0
        })
}

// Parse HSB values
fn parse_hsb(matches: &clap::ArgMatches) -> (i16, i16, i16) {
    let hue = matches
        .get_one::<String>("hue")
        .and_then(|v| v.parse::<i16>().ok())
        .map(|n| {
            if !(0..=360).contains(&n) {
                cli::exit_usage("Hue is outside the valid range, should be 0 - 360 (degrees)");
            }
            n
        })
        .unwrap_or(-1);

    let saturation = matches
        .get_one::<String>("saturation")
        .and_then(|v| v.parse::<i16>().ok())
        .map(|n| {
            if !(0..=100).contains(&n) {
                cli::exit_usage("Saturation is outside the valid range, should be 0 - 100 (percent)");
            }
            n
        })
        .unwrap_or(-1);

    let brightness = matches
        .get_one::<String>("brightness")
        .and_then(|v| v.parse::<i16>().ok())
        .map(|n| {
            if !(0..=100).contains(&n) {
                cli::exit_usage("Brightness is outside the valid range, should be 0 - 100 (percent)");
            }
            n
        })
        .unwrap_or(-1);

    (hue, saturation, brightness)
}

// Handle color changes
fn handle_color(matches: &clap::ArgMatches, device: &network::Device, duration: u32) {
    if let Some(v) = matches.get_one::<String>("colour") {
        cli::print_info_sending("Setting device colour...");
        let _ = messages::set_device_state(device, &colour::get_colour(v), 1000, duration);
    }
}

// Handle HSB changes
fn handle_hsb(device: &network::Device, hue: i16, saturation: i16, brightness: i16, duration: u32) {
    if hue >= 0 || saturation >= 0 || brightness >= 0 {
        let hue = if hue < 0 { 360 } else { hue };
        let saturation = if saturation < 0 { 100 } else { saturation };
        let brightness = if brightness < 0 { 100 } else { brightness };

        let _ = messages::set_device_state(
            device,
            &colour::Hsb {
                hue: hue as u16,
                saturation: saturation as u8,
                brightness: brightness as u8,
            },
            1000,
            duration,
        );
    }
}

// Parse flash interval
fn parse_interval(matches: &clap::ArgMatches) -> u64 {
    matches
        .get_one::<String>("interval")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(1000)
}

// Handle flash command
fn handle_flash(matches: &clap::ArgMatches, device: &network::Device, interval: u64) {
    if let Some(v) = matches.get_one::<String>("flash") {
        cli::print_info_sending("Flashing device to another colour...");
        flash(device, colour::get_colour(v), interval);
    }
}

// TODO: ponder fade
// let fade_len: u32 = 3000;
// fade(&device, colour::get_colour("crimson"), fade_len);
// thread::sleep(Duration::from_millis((fade_len + 1) as u64));
// Fade device back to initial state.
// fade(&device, initial_state.unwrap(), 3000);

fn main() {
    print_program_header();

    let matches = configure_cli();

    let subnet = parse_subnet(&matches);
    let device = get_device(&matches, subnet);

    if handle_report(&matches, &device) {
        return;
    }

    if handle_power(&matches, &device) {
        return;
    }

    let duration = parse_duration(&matches);
    let (hue, saturation, brightness) = parse_hsb(&matches);

    handle_color(&matches, &device, duration);
    handle_hsb(&device, hue, saturation, brightness, duration);

    let interval = parse_interval(&matches);
    handle_flash(&matches, &device, interval);

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
