use std::process::Command;
use std::str;

fn run_argswap(args: &[&str]) -> (i32, String, String) {
    let mut exe_path = std::env::current_exe().unwrap();
    exe_path.pop();
    if exe_path.ends_with("deps") {
        exe_path.pop();
    }
    exe_path.push("argswap");

    let output = Command::new(exe_path)
        .args(args)
        .output()
        .expect("Failed to execute argswap binary");

    let code = output.status.code().unwrap_or(-1);
    let stdout = str::from_utf8(&output.stdout).unwrap().to_string();
    let stderr = str::from_utf8(&output.stderr).unwrap().to_string();

    (code, stdout, stderr)
}

#[test]
fn test_normal_swap_io() {
    // 0:echo, 1:foo, 2:hoo
    // $ argswap -i 1,2 -o 2,1 -- echo foo hoo
    let (code, stdout, _stderr) =
        run_argswap(&["-i", "1,2", "-o", "2,1", "--", "echo", "foo", "hoo"]);
    assert_eq!(code, 0);
    assert_eq!(stdout.trim(), "hoo foo");
}

#[test]
fn test_normal_drop() {
    // 0:echo, 1:foo, 2:hoo, 3:bar
    // $ argswap -d 1,3 -- echo foo hoo bar
    let (code, stdout, _stderr) = run_argswap(&["-d", "1,3", "--", "echo", "foo", "hoo", "bar"]);
    assert_eq!(code, 0);
    assert_eq!(stdout.trim(), "hoo");
}

#[test]
fn test_normal_adjacent_swap() {
    // 0:echo, 1:foo, 2:hoo, 3:bar
    // $ argswap -s 0,2 -- echo foo hoo bar
    let (code, _stdout, stderr) = run_argswap(&["-s", "0,2", "--", "echo", "foo", "hoo", "bar"]);
    assert_ne!(code, 0);
    assert!(stderr.contains("Failed to execute command 'foo'"));
}

#[test]
fn test_error_input_out_of_bounds() {
    let (code, _stdout, stderr) =
        run_argswap(&["-i", "1,3", "-o", "3,1", "--", "echo", "foo", "hoo"]);
    assert_ne!(code, 0);
    assert!(stderr.contains("Error: Input index 3 is out of bounds"));
}

#[test]
fn test_error_drop_out_of_bounds() {
    let (code, _stdout, stderr) = run_argswap(&["-d", "3", "--", "echo", "foo", "hoo"]);
    assert_ne!(code, 0);
    assert!(stderr.contains("Error: Drop index 3 is out of bounds"));
}

#[test]
fn test_error_swap_out_of_bounds() {
    let (code, _stdout, stderr) = run_argswap(&["-s", "2", "--", "echo", "foo", "hoo"]);
    assert_ne!(code, 0);
    assert!(stderr.contains("Error: Cannot swap index 2 and 3. Out of bounds"));
}
