use atty::Stream;
use dirs::home_dir;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::num::Wrapping;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

struct Configuration {
    faces: &'static [&'static str],
    animdur: f32,
    animation: bool,
    dicesize: u128,
}

const DEFAULTCONF: Configuration = Configuration {
    faces: &["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"],
    animdur: 0.6,
    animation: true,
    dicesize: 6,
};

fn main() {
    let configdat = readconfig("d6.cfg");
    let config: Configuration = parseconfig(configdat);

    let faces: Vec<&str> = Vec::from(config.faces);
    let animdur: f32 = config.animdur;
    let animation: bool = config.animation;
    let mut dicesize: u128 = config.dicesize;

    // if user runs d6 <int>, then set dicesize to the integer
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let intnum = &args[1];
        if isnumeric(intnum) {
            let newnum: u128 = intnum.parse().unwrap();
            if newnum <= 0 {
                eprintln!("cannot roll a d0. rolling a d6.")
            } else {
                dicesize = intnum.parse().unwrap();
            }
        }
    }

    // only show the animation if animations are enabled and if the terminal is interactive
    if animation && atty::is(Stream::Stdout) {
        let endtime = Instant::now() + Duration::from_secs_f32(animdur);
        let interval = Duration::from_millis(30);
        while Instant::now() <= endtime {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            // use faces.len here so that roll doesnt go out of bounds
            let roll = (bitmixer(nanos) % faces.len() as u128) as usize;
            print!("\r{}", faces[roll]);
            io::stdout().flush().unwrap();
            sleep(interval);
        }
    }

    // generate pseudo random number using system time
    let nanos: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mixednanos: u128 = bitmixer(nanos);

    let finalroll: usize = (mixednanos % dicesize as u128) as usize;
    let finalsymbol: &str = if finalroll > faces.len() {
        "x"
    } else {
        &faces[finalroll]
    };
    if atty::is(Stream::Stdout) {
        // interactive terminal
        println!("\r{} ({})", finalsymbol, finalroll + 1);
    } else {
        // only print the value if it is piped
        println!("{}", finalroll + 1);
    }
}

fn isnumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

fn bitmixer(val: u128) -> u128 {
    // so that multiplying wont make the number go out of bounds
    let mut wrappedval = Wrapping(val);
    // bit mixer to fix the rng
    wrappedval ^= wrappedval >> 71;
    wrappedval *= Wrapping(2293847102873847293);
    wrappedval ^= wrappedval >> 17;
    wrappedval *= Wrapping(1717171717171771);
    wrappedval ^= wrappedval >> 45;

    wrappedval.0
}

fn readconfig(filename: &str) -> String {
    let mut filepath = match home_dir() {
        Some(d) => d,
        None => {
            return String::new();
        }
    };

    filepath.push(".config/");
    filepath.push(filename);

    match fs::read_to_string(&filepath) {
        Ok(data) => data,
        Err(_) => String::new(),
    }
}

fn parseconfig(data: String) -> Configuration {
    let mut config: Configuration = DEFAULTCONF;
    if data.is_empty() {
        return DEFAULTCONF;
    }

    for line in data.lines() {
        let parts: Vec<&str> = line.split(':').map(|p| p.trim()).collect::<Vec<&str>>();

        if parts.len() != 2 {
            eprintln!("config error: too many arguments in: '{}'", line);
            eprintln!("ignoring config");
            return DEFAULTCONF;
        }

        match parts[0] {
            "animation" => match parts[1].parse::<bool>() {
                Ok(value) => config.animation = value,
                Err(_) => {
                    eprintln!(
                        "config error: invalid boolean for 'animation' at: '{}'",
                        line
                    );
                    eprintln!("ignoring config");
                    return DEFAULTCONF;
                }
            },
            "animdur" => match parts[1].parse::<f32>() {
                Ok(value) => config.animdur = value,
                Err(_) => {
                    eprintln!("config error: invalid f32 for 'animdur' at: '{}'", line);
                    eprintln!("ignoring config");
                    return DEFAULTCONF;
                }
            },
            "dicesize" => match parts[1].parse::<u128>() {
                Ok(value) => config.dicesize = value,
                Err(_) => {
                    eprintln!("config error: invalid u128 for 'dicesize' at: '{}'", line);
                    eprintln!("ignoring config");
                    return DEFAULTCONF;
                }
            },
            _ => {
                eprintln!("config error: invalid token '{}' at: '{}'", parts[0], line);
                eprintln!("ignoring config");
                return DEFAULTCONF;
            }
        }
    }
    config
}
