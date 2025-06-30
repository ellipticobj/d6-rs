use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    // TODO: add a way to change these vars
    let faces = ["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"];
    let animdur: f32 = 0.6;
    let dicesize = 6;
    let animation = true;

    if animation {
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
    println!("\r{} ({})", faces[finalroll], finalroll + 1);
}
