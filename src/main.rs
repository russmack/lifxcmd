extern crate rustylifx;

use rustylifx::{colour, messages, response};
use rustylifx::colour::{HSB, HSBK};
use rustylifx::network::{Device};

use std::thread;
use std::time::Duration;

fn main() {
    // TODO: option to locate device, or specify IP address.
    // TODO: fix having to sleep between requests.

    // Locate device.
    let device = messages::get_service().unwrap();
    thread::sleep(Duration::from_millis(1000));

    // Get current device state.
    let device = messages::get_device_state(device).unwrap();
    thread::sleep(Duration::from_millis(1000));
    
    flash(&device, colour::RED, 200);
    thread::sleep(Duration::from_millis(1000));
    // TODO: hsbk to hsb
    /*
    let current_state = match payload {
        Some(v) =>  Some(
                        colour::HSB::new(
                            v.hsbk.hue, 
                            v.hsbk.saturation, 
                            v.hsbk.brightness,
                        )
                    ),
        None    =>  None,
    };
    */
    // Extract current HSVK from device state data.
    let payload = match device.response.payload {
        response::Payload::State(ref v) => Some(v),
        _ => None,
    };
    let initial_state = match payload {
        Some(v) => {
            let h = colour::hue_word_to_degrees(v.hsbk.hue.parse::<u16>().unwrap());
            let s = colour::saturation_word_to_percent(v.hsbk.saturation.parse::<u16>().unwrap());
            let b = colour::brightness_word_to_percent(v.hsbk.brightness.parse::<u16>().unwrap());
            Some(colour::HSB::new(h, s, b))
        },
        None => None,
    };
    let fade_len: u32 = 3000;
    fade(&device, colour::CRIMSON, fade_len);
    thread::sleep(Duration::from_millis((fade_len+1) as u64));
    // Fade device back to initial state.
    fade(&device, initial_state.unwrap(), 3000);

    /*
    /// current_state is an Option<colour::HSB>
    let current_state = match payload {
        Some(v) => {
            println!("current payload body: {:?}", v.body);
            println!("current hue: {:?}", v.hsbk.hue);
            let h = colour::hue_word_to_degrees(v.hsbk.hue.parse::<u16>().unwrap());
            println!("current hue degrees: {:?}", h);
            println!("current sat: {:?}", v.hsbk.saturation);
            let s = colour::saturation_word_to_percent(v.hsbk.saturation.parse::<u16>().unwrap());
            println!("current sat percent: {:?}", s);
            println!("current bri: {:?}", v.hsbk.brightness);
            let b = colour::brightness_word_to_percent(v.hsbk.brightness.parse::<u16>().unwrap());
            println!("current bri percent: {:?}", b);
            println!("current kel: {:?}", v.hsbk.kelvin);
            Some(colour::HSB::new(h, s, b))
        },
        None => None,
    };
    */
    println!("\n");

}

fn flash(device: &Device, flash_colour: HSB, duration_ms: u64) {
    // Extract current HSVK from device state data.
    let payload = match device.response.payload {
        response::Payload::State(ref v) => Some(v),
        _ => None,
    };

    // TODO: hsbk to hsb
    /*
    let current_state = match payload {
        Some(v) =>  Some(
                        colour::HSB::new(
                            v.hsbk.hue, 
                            v.hsbk.saturation, 
                            v.hsbk.brightness,
                        )
                    ),
        None    =>  None,
    };
    */
    let current_state = match payload {
        Some(v) => {
            let h = colour::hue_word_to_degrees(v.hsbk.hue.parse::<u16>().unwrap());
            let s = colour::saturation_word_to_percent(v.hsbk.saturation.parse::<u16>().unwrap());
            let b = colour::brightness_word_to_percent(v.hsbk.brightness.parse::<u16>().unwrap());
            Some(colour::HSB::new(h, s, b))
        },
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
        },
        None => (),
    };
}

fn fade(device: &Device, to_colour: HSB, duration_ms: u32) {
    let _ = messages::set_device_state(&device, &to_colour, 2500, duration_ms);
}

