use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

pub fn copy(s: &str, paste_once: bool) {
    if is_program_in_path("wl-copy") {
        let mut child = Command::new("wl-copy")
            .arg(if paste_once { "--paste-once" } else { "" })
            .stdin(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
        child.stdin.take().unwrap().write_all(s.as_bytes()).unwrap();
    } else if is_program_in_path("xclip") {
        let mut child = Command::new("xclip")
            .stdin(Stdio::piped())
            .spawn()
            .expect("failed to execute process");
        child.stdin.take().unwrap().write_all(s.as_bytes()).unwrap();
    };
}
