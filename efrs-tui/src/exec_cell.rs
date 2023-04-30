
enum CellStatus {
    UnderConstruction,
    Running,
    Finished
}

pub struct ExecCell {
    cmd_tmpl: String,
    status: CellStatus,
    stdout: Vec<String>,
    stderr: Vec<String>
}
