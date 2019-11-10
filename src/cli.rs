use std::collections::HashMap;
use std::io::Write;
use std::process;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print_line_info_prefix(icon: &str, status: &str, message: &str, colour_primary: Color, colour_secondary: Color) {
    // Display icon.
    print_string(icon, colour_primary, false);

    // Display status left brace.
    let left_brace = " [";
    print_string(left_brace, colour_secondary, false);
    
    // Display status word.
    print_string(status, colour_primary, false);

    // Display status right brace.
    let right_brace = "]\t";
    print_string(right_brace, colour_secondary, false);
    
    // Display message.
    print_string(message, colour_secondary, false);
}

pub fn print_string(s: &str, colour: Color, bold: bool) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut colour_spec = ColorSpec::new();
    colour_spec.set_fg(Some(colour));
    colour_spec.set_bold(bold);

    match stdout.set_color(&colour_spec) {
        Ok(_) => (),
        Err(e) => println!("Failed setting terminal output colour: {}", e),
    };
    let s_out = format!("{}", s);
    if write!(&mut stdout,"{}", s_out).is_err() {
        print!("{}", s_out);
    };

    // Reset colour to default white.
    colour_spec.set_fg(Some(Color::White));
    match stdout.set_color(&colour_spec) {
        Ok(_) => (),
        Err(e) => println!("Failed setting terminal output colour: {}", e),
    };
}

pub fn print_response_header() {
    println!("\n");
    print_string("== ", Color::Cyan, false);
    print_string("Result", Color::White, true);
    print_string(" ===========================================================================", Color::Cyan, false);
    println!("\n");
}

pub fn exit_usage(s: &str) {
    print_response_header();

    // Display error.
    print_line_info_prefix("!", "invalid", s, Color::Yellow, Color::White);

    // Display usage.
    println!("");
    let s_out = format!("lifxcmd --help");
    print_line_info_prefix("→", "usage", &s_out, Color::Cyan, Color::White);
    println!("\n");

    process::exit(1);
}

pub fn exit_error(s: &str) {
    print_response_header();

    print_line_info_prefix("✘", "error", s, Color::Red, Color::White);
    println!("\n");

    process::exit(1);
}

pub fn exit_done(s: &str) {
    print_response_header();

    println!("{}", s);
    print_done();
    println!("\n");

    process::exit(0);
}

pub fn print_done() {
    print_line_info_prefix("∗", "Done", "", Color::Green, Color::White);
}

pub fn format_device_state(map: &HashMap<&str, String>) -> String {
    let print_order = [
        "Source",
        "Firmware",
        "Mac addr",
        "Message type",
        "Size",
        "Current hue",
        "Current hue degrees",
        "Current saturation",
        "Current saturation percent",
        "Current brightness",
        "Current brightness percent",
        "Current kel",
        "Sequence num",
        "Reserved_1",
        "Reserved_2",
    ];

    let mut report = String::new();
    for &k in &print_order {
        match map.get(k) {
            Some(v) =>
            report.push_str(&format!("{: <30}: {}\n", k, v)),
            None => {},
        }
    }

    report
}

