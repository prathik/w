use chrono;
use std::fs::OpenOptions;
use std::io::prelude::*;

use std::io;

fn flush() {
    io::stdout()
        .flush()
        .map_err(|err| println!("{:?}", err))
        .ok();
}

fn main() {
    let start_hour: u32 = std::env::args()
        .nth(1)
        .expect("hour not given")
        .parse()
        .unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("w.csv")
        .unwrap();

    let date = chrono::Local::today();
    for h in start_hour..start_hour + 3 {
        let activity_hour = date.and_hms(h, 0, 0);
        print!("Enter activity at {}: ", activity_hour.format("%H:%M"));
        flush();
        let mut activity = String::new();
        io::stdin()
            .read_line(&mut activity)
            .expect("error: unable to read user input");
        activity.pop();
        print!("Enter joy: ");
        flush();
        let mut joy = String::new();
        io::stdin()
            .read_line(&mut joy)
            .expect("error: unable to read user input");
        joy.pop();

        print!("Enter importance: ");
        flush();
        let mut importance = String::new();
        io::stdin()
            .read_line(&mut importance)
            .expect("error: unable to read user input");
        importance.pop();

        if let Err(e) = writeln!(
            file,
            "{},{},{},{}",
            activity_hour.to_rfc3339(),
            activity,
            joy,
            importance
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
