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

    let git_comment = '|';
    let comments: &[_] = &['/', git_comment];
    let lines: Vec<&str> = clipboard
        .lines()
        .map(|line| {
            let trimmed = line.trim().trim_start_matches(comments).trim();
            if trimmed == "" {
                "\n"
            } else {
                trimmed
            }
        })
        .collect();
    let result = lines.join(" ");
    let result = result.replace(" \n ", "\n");
    // println!("{}", result);

    let mut pbcopy = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .expect("pbcopy run");
    let stdin = pbcopy.stdin.as_mut().expect("pbcopy stdin");
    write!(stdin, "{}", result).expect("write");
    pbcopy.wait().expect("wait pbcopy");
}
