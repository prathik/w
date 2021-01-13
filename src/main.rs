use chrono;
use std::fs::OpenOptions;
use std::io::prelude::*;

use std::io;

fn main() {
    print!("Enter time: ");
    io::stdout()
        .flush()
        .map_err(|err| println!("{:?}", err))
        .ok();
    let mut time = String::new();
    io::stdin()
        .read_line(&mut time)
        .expect("error: unable to read user input");
    time.pop();

    print!("Enter activity: ");
    io::stdout()
        .flush()
        .map_err(|err| println!("{:?}", err))
        .ok();

    let mut activity = String::new();
    io::stdin()
        .read_line(&mut activity)
        .expect("error: unable to read user input");
    activity.pop();

    print!("Enter joy: ");
    io::stdout()
        .flush()
        .map_err(|err| println!("{:?}", err))
        .ok();
    let mut joy = String::new();
    io::stdin()
        .read_line(&mut joy)
        .expect("error: unable to read user input");
    joy.pop();

    print!("Enter importance: ");
    io::stdout().flush().expect("unable to flush");
    let mut importance = String::new();
    io::stdin()
        .read_line(&mut importance)
        .expect("error: unable to read user input");
    importance.pop();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("w.csv")
        .unwrap();

    if let Err(e) = writeln!(
        file,
        "{} {},{},{},{}",
        chrono::Local::now().format("%Y-%m-%d"),
        time,
        activity,
        joy,
        importance
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
