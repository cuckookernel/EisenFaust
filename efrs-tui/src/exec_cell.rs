
use std::io::{BufRead, self};
use std::process::{Child, Command, ChildStdout};
use std::thread::{JoinHandle, self};
use std::sync::mpsc::{Receiver, Sender, channel};

use async_process::Stdio;


pub enum CmdMsgType {
    StdOut,
    StdErr,
}

pub struct CmdMsg {
    pub typ: CmdMsgType,
    pub content: String
}


pub enum CellStatus {
    UnderConstruction,
    Running,
    Failed,
    // Finished - TODO
}

pub struct ExecInfo {
    pub child: Child,
    pub bridge_thread: JoinHandle<()>,
    pub output_receiver: Receiver<CmdMsg>,
}

pub struct ExecCell {
    pub cmd_tmpl: String,
    pub status: CellStatus,
    pub cmd_line: String,
    pub exec_info: Option<ExecInfo>,
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
}

impl ExecCell {
    pub fn new(cmd_tmpl: String) -> ExecCell {
        let cmd_line = cmd_tmpl.clone();
        ExecCell{
            cmd_tmpl,
            status: CellStatus::UnderConstruction,
            cmd_line,
            exec_info: None,
            stdout: vec![],
            stderr: vec![]
        }
    }

    pub fn run_cmd(&mut self) {
        let exec_info_opt = start_running_cmd(&self.cmd_line);
        match exec_info_opt {
            Ok(exec_info) => {
                self.exec_info = Some(exec_info);
                self.status = CellStatus::Running
            }
            Err(err) => {
                self.status = CellStatus::Failed;
                self.stdout = vec![format!("{:?}: {:?}", err.kind(), err)]
            }
        }
    }
}



pub fn start_running_cmd(cmd_line: &String) -> Result<ExecInfo, io::Error> {
    let line_parts: Vec<&str> = cmd_line.split(' ').collect();

    let cmd = line_parts[0];
    let args = line_parts[1..].to_vec();
    let mut child = start_child(cmd, &args)?;

    let (tx, output_receiver) = channel::<CmdMsg>();
    let std_out = child.stdout.take().unwrap();

    let bridge_thread = setup_bridge_thread(std_out, tx);

    Ok(ExecInfo{child, bridge_thread, output_receiver})
}


pub fn start_child(cmd: &str, args: &Vec<&str>) -> Result<Child, io::Error> {
    let child_process = Command::new(cmd)
    .args(args)
    .stdout(Stdio::piped())
    .spawn()?;

    Ok(child_process)
}

pub fn setup_bridge_thread(child_out: ChildStdout,
                           tx: Sender<CmdMsg>) -> JoinHandle<()> {

    // Spawn a thread to read the subprocess output and send it through the channel
    let bridge_thread = thread::spawn(move || {
        let reader = std::io::BufReader::new(child_out);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let msg = CmdMsg{typ: CmdMsgType::StdOut, content: line};
                    tx.send(msg).unwrap();
                }
                Err(e) => {
                    eprintln!("Error reading subprocess output: {}", e);
                }
            }
        }
    });

    return bridge_thread
}
