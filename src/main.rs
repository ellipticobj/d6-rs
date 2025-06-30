use atty::Stream;
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
        let interval = Duration::from_millis(10);
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
