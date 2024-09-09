use std::io::BufRead;
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

fn main() {
    // Spawn the subprocess
    let mut child_process = Command::new("cargo")
        .args(&["run", "--bin", "lengthy"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn lengthy subprocess");

    // Create a channel for receiving the subprocess output
    let (tx, rx): (std::sync::mpsc::Sender<String>, Receiver<String>) = channel();

    let output = child_process.stdout.take().unwrap();

    // Spawn a thread to read the subprocess output and send it through the channel
    let _out_rcvr = thread::spawn(move || {
        let reader = std::io::BufReader::new(output);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    tx.send(line).unwrap();
                }
                Err(e) => {
                    eprintln!("Error reading subprocess output: {}", e);
                }
            }
        }
    });

    // Receive the subprocess output from the channel
    for line in rx {
        println!("Received output: {}", line);
    }

    // Wait for the subprocess to finish and get its exit code
    let exit_code = child_process.wait().expect("Failed to wait for subprocess");
    println!("Subprocess exited with code: {}", exit_code);
}