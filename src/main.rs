use atty::Stream;
use dirs::home_dir;
use std::fs;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    // TODO: add a way to change these vars
    let faces = ["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"];
    let animdur: f32 = 0.6;
    let dicesize = 6;
    let animation = true;

    // only show the animation if animations are enabled and if the terminal is interactive
    if animation && atty::is(Stream::Stdout) {
        let endtime = Instant::now() + Duration::from_secs_f32(animdur);
        let interval = Duration::from_millis(30);
        while Instant::now() <= endtime {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let roll = (nanos % 6) as usize;
            print!("\r{}", faces[roll]);
            io::stdout().flush().unwrap();
            sleep(interval);
        }
    }

    // generate pseudo random number using system time
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let finalroll = (nanos % dicesize) as usize;
    if atty::is(Stream::Stdout) {
        // interactive terminal
        println!("\r{} ({})", faces[finalroll], finalroll + 1);
    } else {
        // only print the value if it is piped
        println!("{}", finalroll + 1);
    }
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

fn parseconfig(data: String) -> Vec<String> {
    if data.is_empty() {
        return Vec::new();
    }
    for line in data.lines() {
        let parts: Vec<&str> = line.split(':').collect::<Vec<&str>>();
        if parts.len() != 2 {
            eprintln!("config error at: {}", line);
            eprintln!("ignoring config");
            return Vec::new();
        }
        // TODO: parse one line of the config
    }
    Vec::new()
}
