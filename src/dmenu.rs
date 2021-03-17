use itertools::Itertools;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn run<T>(hm: &std::collections::HashMap<String, T>, dmenu_path: String) -> &T {
    let mut child = Command::new(dmenu_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let child_stdin = child.stdin.as_mut().unwrap();

    let dmenu_items = hm.keys().join("\n");

    child_stdin.write_all(dmenu_items.as_bytes()).unwrap();
    // Close stdin to finish and avoid indefinite blocking
    drop(child_stdin);

    let output = child.wait_with_output().unwrap();
    let output_str = std::str::from_utf8(output.stdout.as_slice()).unwrap().to_owned();

    println!("output = {:?}", output);

    hm.get(output_str.trim()).unwrap()
}
