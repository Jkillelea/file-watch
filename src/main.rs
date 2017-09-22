use std::env;
use std::thread;
use std::time::Duration;
use std::fs;
use std::process::Command;

fn main() {
    let command_name = match env::args().nth(1) {
        Some(n) => n,
        None => panic!("No Command Name!"),
    };
    let filename = match env::args().nth(2) {
        Some(f) => f,
        None    => panic!("No Filename!"),
    };

    let mut data = fs::metadata(&filename).unwrap();
    let mut last_mtime = data.modified().unwrap();

    loop {
        // try and get new file metadata. If we can't, just use the old one
        data = fs::metadata(&filename).unwrap_or(data);
        let mtime = data.modified().unwrap_or(last_mtime);

        if mtime != last_mtime {
            last_mtime = mtime;
            Command::new(&command_name) // reference because 'use of moved value' - we take the value on the first iteration and then it'd be gone
                    // .arg("-file-line-error") // sane error messages
                    .arg(&filename)
                    .spawn()
                    .expect("Command pdflatex spawn failed");
        }

        thread::sleep(Duration::new(0, 500_000_000)); // half a second
    }
}
