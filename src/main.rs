extern crate rustylifx;
extern crate clap;

use rustylifx::{colour, messages, network, response};
use rustylifx::colour::{HSB, HSBK};
use rustylifx::network::Device;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use std::time::Duration;

use clap::{Arg, App};

fn main() {
    // Flag to locate device, or specify IP address.
    let matches = App::new("Lifx Command")
        .version("0.1")
        .author("Russell Mackenzie")
        .about("Control Lifx devices from the command line.")
        .arg(Arg::with_name("device")
            .short("d")
            .long("device")
            .value_name("IPADDRESS")
            .help("Specifies the address of the target device")
            .takes_value(true))
        .arg(Arg::with_name("colour")
            .short("c")
            .long("colour")
            .value_name("COLOUR NAME")
            .help("Changes the colour")
            .takes_value(true))
        .get_matches();

    let device = match matches.value_of("device").unwrap_or("") {
        "" => {
            // Locate device.
            messages::get_service().unwrap()
        }
        ip => {
            // Set device.
            const PORT: u16 = 56700;
            network::Device {
                socket_addr: format!("{}:{}", ip, PORT).parse().expect("invalid socket address"),
                response: None,
            }
        }
    };

    match matches.value_of("colour") {
        Some(v) => {
            messages::set_device_state(&device, &colour::get_colour(v), 1000, 0);
            return;
        }
        None => (),
    };


    // TODO: fix having to sleep between requests.


    thread::sleep(Duration::from_millis(1000));

    // Get current device state.
    let device = messages::get_device_state(device).unwrap();
    thread::sleep(Duration::from_millis(1000));

    flash(&device, colour::get_colour("red"), 200);
    thread::sleep(Duration::from_millis(1000));

    // Extract current HSVK from device state data.
    let resp = match device.response {
        Some(ref v) => v,
        None => panic!("no response"),
    };

    let payload = match resp.payload {
        response::Payload::State(ref v) => Some(v),
        _ => None,
    };

    let initial_state = match payload {
        Some(v) => {
            let h = colour::hue_word_to_degrees(v.hsbk.hue);
            let s = colour::saturation_word_to_percent(v.hsbk.saturation as u16);
            let b = colour::brightness_word_to_percent(v.hsbk.brightness as u16);
            Some(colour::HSB::new(h, s, b))
        }
        None => None,
    };

    let fade_len: u32 = 3000;
    fade(&device, colour::get_colour("crimson"), fade_len);
    thread::sleep(Duration::from_millis((fade_len + 1) as u64));

    // Fade device back to initial state.
    fade(&device, initial_state.unwrap(), 3000);

    thread::sleep(Duration::from_millis((fade_len + 1) as u64));
    let _ = messages::set_device_state(&device, &colour::get_colour("chartreuse"), 1000, 0);
    thread::sleep(Duration::from_millis((1000) as u64));
    fade(&device, colour::get_colour("crimson"), 300000);

    println!("\n");
}

fn flash(device: &Device, flash_colour: HSB, duration_ms: u64) {
    // Extract current HSVK from device state data.
    let resp = match device.response {
        Some(ref v) => v,
        None => panic!("no response"),
    };

    let payload = match resp.payload {
        response::Payload::State(ref v) => Some(v),
        _ => None,
    };

    let current_state = match payload {
        Some(v) => {
            let h = colour::hue_word_to_degrees(v.hsbk.hue);
            let s = colour::saturation_word_to_percent(v.hsbk.saturation as u16);
            let b = colour::brightness_word_to_percent(v.hsbk.brightness as u16);
            Some(colour::HSB::new(h, s, b))
        }
        None => None,
    };

    match current_state {
        Some(v) => {
            // Change device state temporarily.
            let _ = messages::set_device_state(&device, &flash_colour, 2500, 0);
            thread::sleep(Duration::from_millis(duration_ms));

            // Return device to initial state.
            let _ = messages::set_device_state(&device, &v, 2500, 0);
            println!("col:: {:?}", &v);
        }
        None => (),
    };
}

fn fade(device: &Device, to_colour: HSB, duration_ms: u32) {
    let _ = messages::set_device_state(&device, &to_colour, 2500, duration_ms);
}
