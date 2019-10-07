use std::io::Write;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let clipboard = Command::new("pbpaste")
        .output()
        .expect("failed to execute process")
        .stdout;

    let clipboard = std::str::from_utf8(&clipboard).expect("STDOUT is utf8 encoding");
    println!("{}", clipboard);

    let lines: Vec<&str> = clipboard
        .lines()
        .map(|line| line.trim().trim_start_matches('/').trim())
        .collect();
    let result = lines.join(" ");
    // println!("{}", result);

    let mut pbcopy = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("pbcopy run");
    let stdin = pbcopy.stdin.as_mut().expect("pbcopy stdin");
    write!(stdin, "{}", result).expect("write");
    pbcopy.wait().expect("wait pbcopy");
}
