use std::env;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

fn main() {
    let home_dir = env::var("HOME").unwrap();
    let fd_output = Command::new("fd")
        .args([
            "--type",
            "d",
            "-a",
            "--hidden",
            "--exclude",
            ".git",
            ".",
            &home_dir,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute fd");

    let mut fzf_tmux = Command::new("fzf-tmux")
        .args(["-p", "--reverse"])
        .stdin(Stdio::from(fd_output.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute fzf-tmux");

    fzf_tmux.wait().unwrap();

    let mut final_output = String::new();
    if let Some(stdout) = fzf_tmux.stdout.take() {
        let mut bufread = BufReader::new(stdout);
        let mut buf = String::new();

        while let Ok(n) = bufread.read_line(&mut buf) {
            if n > 0 {
                final_output = buf.trim().into();
            } else {
                break;
            }
        }
    }

    if final_output.is_empty() {
        final_output.push('.');
    }

    println!("{}", final_output);
}
