extern crate rustylifx;
extern crate clap;

use rustylifx::{colour, messages, network, response};
use rustylifx::colour::{HSB, HSBK};
use rustylifx::network::Device;

use std::thread;
use std::time::Duration;

use clap::{Arg, App};

fn main() {
    // Configure flags.
    let matches = App::new("Lifx Command")
        .version("0.1")
        .author("Russell Mackenzie")
        .about("Control Lifx devices from the command line.")
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("HOST ADDRESS")
            .help("Specifies the address of the target device")
            .takes_value(true))
        .arg(Arg::with_name("power")
            .short("p")
            .long("power")
            .value_name("POWER LEVEL")
            .help("Changes the power level on/off")
            .takes_value(true))
        .arg(Arg::with_name("colour")
            .short("c")
            .long("colour")
            .value_name("COLOUR NAME")
            .help("Changes the colour")
            .takes_value(true))
        .arg(Arg::with_name("flash")
            .short("f")
            .long("flash")
            .value_name("FLASH COLOUR NAME")
            .help("Specifies the name of the colour to flash")
            .takes_value(true))
        .arg(Arg::with_name("interval")
            .short("i")
            .long("interval")
            .value_name("FLASH INTERVAL")
            .help("The length of the flash")
            .takes_value(true))
        .arg(Arg::with_name("duration")
            .short("d")
            .long("duration")
            .value_name("TRANSITION DURATION")
            .help("The duration of the colour transition")
            .takes_value(true))
        .arg(Arg::with_name("echo")
            .short("e")
            .long("echo")
            .value_name("DISPLAY CURRENT STATE")
            .help("Display the current state of the device")
            .takes_value(false))
        .arg(Arg::with_name("hue")
            .short("h")
            .long("hue")
            .value_name("HUE")
            .help("Set the hue of the device")
            .takes_value(true))
        .arg(Arg::with_name("saturation")
            .short("s")
            .long("saturation")
            .value_name("SATURATION")
            .help("Set the saturation of the device")
            .takes_value(true))
        .arg(Arg::with_name("brightness")
            .short("b")
            .long("brightness")
            .value_name("BRIGHTNESS")
            .help("Set the brightness of the device")
            .takes_value(true))
        .get_matches();

    // Find the device, by flag, else broadcast.
    let device = match matches.value_of("address").unwrap_or("") {
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

    // Check if state display was specified.
    if matches.is_present("state") {
        let device = get_device_state(device);
        display(&device);
        return;
    }

    // Set the power level on/off.
    if let Some(v) = matches.value_of("power") {
        let _ = match v {
            "on"  => messages::set_device_on(&device),
            "off" => messages::set_device_off(&device),
            _ => panic!("power state is invalid, should be on or off."),
        };
    };

    // Check if transition duration was specified.
    let duration = match matches.value_of("duration") {
        Some(v) => {
            match v.parse::<u32>() {
                Ok(n) => n,
                Err(e) => panic!("duration is not a valid number: {}", e),
            }
        }
        None => 0,
    };

    // Set the colour by name if flag exists.
    if let Some(v) = matches.value_of("colour") {
        let _ = messages::set_device_state(&device, &colour::get_colour(v), 1000, duration);
        return;
    }

    // Set the colour by HSB if specified.
    // HSB: 360º, 100%, 100%
    let mut hue = match matches.value_of("hue") {
        Some(v) => {
            match v.parse::<i16>() {
                Ok(n) => {
                    if n >= 0 && n <= 360 {
                        n
                    } else {
                        panic!("hue is outside the valid range, should be 0 - 360 (degrees)")
                    }
                }
                Err(e) => panic!("hue is not a valid number: {}", e),
            }
        }
        None => -1,
    };

    let mut saturation = match matches.value_of("saturation") {
        Some(v) => {
            match v.parse::<i16>() {
                Ok(n) => {
                    if n >= 0 && n <= 100 {
                        n
                    } else {
                        panic!("saturation is outside the valid range, should be 0 - 100 (percent)")
                    }
                }
                Err(e) => panic!("saturation is not a valid number: {}", e),
            }
        }
        None => -1,
    };

    let mut brightness = match matches.value_of("brightness") {
        Some(v) => {
            match v.parse::<i16>() {
                Ok(n) => {
                    if n >= 0 && n <= 100 {
                        n
                    } else {
                        panic!("brightness is outside the valid range, should be 0 - 100 (percent)")
                    }
                }
                Err(e) => panic!("brightness is not a valid number: {}", e),
            }
        }
        None => -1,
    };

    if hue >= 0 || saturation >= 0 || brightness >= 0 {
        if hue < 0 {
            hue = 360;
        }

        if saturation < 0 {
            saturation = 100;
        }

        if brightness < 0 {
            brightness = 100;
        }

        let _ = messages::set_device_state(&device,
                                           &colour::HSB {
                                               hue: hue as u16,
                                               saturation: saturation as u8,
                                               brightness: brightness as u8,
                                           },
                                           1000,
                                           duration);
        return;
    }

    // Check if the flash interval was specified.
    let interval = match matches.value_of("interval") {
        Some(v) => {
            match v.parse::<u64>() {
                Ok(n) => n,
                Err(e) => panic!("interval is not a valid number: {}", e),
            }
        }
        None => 1000,
    };

    // Flash if flag exists.
    if let Some(v) = matches.value_of("flash") {
        flash(device, colour::get_colour(v), interval);
        return;
    }

    // TODO: ponder fade
    // let fade_len: u32 = 3000;
    // fade(&device, colour::get_colour("crimson"), fade_len);
    // thread::sleep(Duration::from_millis((fade_len + 1) as u64));
    // Fade device back to initial state.
    // fade(&device, initial_state.unwrap(), 3000);
}

fn get_device_state(device: Device) -> Device {
    // TODO: sort out this hacky sleep.
    thread::sleep(Duration::from_millis(1000));
    messages::get_device_state(device).unwrap()
}

fn flash(device: Device, flash_colour: HSB, duration_ms: u64) {
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

    let initial_state = match payload {
        Some(v) => {
            let h = colour::hue_word_to_degrees(v.hsbk.hue);
            let s = colour::saturation_word_to_percent(v.hsbk.saturation as u16);
            let b = colour::brightness_word_to_percent(v.hsbk.brightness as u16);
            Some(colour::HSB::new(h, s, b))
        }
        None => None,
    };

    if let Some(v) = initial_state {
        // Change device state temporarily.
        let _ = messages::set_device_state(&device, &flash_colour, 2500, 0);
        thread::sleep(Duration::from_millis(duration_ms));

        // Return device to initial state.
        let _ = messages::set_device_state(&device, &v, 2500, 0);
    }
}

fn display(device: &Device) {
    let resp = match device.response {
        Some(ref v) => v,
        None => {
            println!("No response from device.");
            return;
        }
    };

    println!("\nDevice State:");
    println!("Source: {:?}", resp.source);
    println!("Mac addr: {:?}", resp.mac_address);
    println!("Firmware: {:?}", resp.firmware);

    println!("\nResponse:");
    println!("Size: {}", resp.size);

    // packed byte
    println!("Sequence num: {:?}", resp.sequence_number);
    println!("Reserved_1 (timestamp?): {:?}", resp.reserved_1);
    println!("Message type: {:?}", resp.message_type);
    println!("Reserved_2: {:?}", resp.reserved_2);

    match resp.payload {
        response::Payload::StateService(ref v) => {
            println!("Service: {:?}", v.service);
            println!("Port: {:?}", v.port);
            println!("Unknown: {:?}", v.unknown);
        }
        response::Payload::State(ref v) => {
            println!("Body: {:?}", v.body);
            println!("HSBK: {:?}", v.hsbk);
            println!("current hue: {:?}", v.hsbk.hue);
            println!("current hue degrees: {:?}º",
                     colour::hue_word_to_degrees(v.hsbk.hue));
            println!("current sat: {:?}", v.hsbk.saturation);
            println!("current sat percent: {:?}%",
                     colour::saturation_word_to_percent(v.hsbk.saturation as u16));
            println!("current bri: {:?}", v.hsbk.brightness);
            println!("current bri percent: {:?}%",
                     colour::brightness_word_to_percent(v.hsbk.brightness as u16));
            println!("current kel: {:?}", v.hsbk.kelvin);
        }
        _ => (),
    };

    println!("==========");
}
