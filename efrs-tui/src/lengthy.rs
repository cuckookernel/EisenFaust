
use std::io::Error;
use std::time;
use std::thread::sleep;

pub fn main() -> Result<(), Error> {

    for i in 0..10 {
        println!("line: {}", i);
        sleep(time::Duration::from_millis(200))
    }

    Ok(())
}