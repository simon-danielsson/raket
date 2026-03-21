// *brakoll - d: init readme and license, p: 100, t: docs, s: closed
// *brakoll - d: color enums to avoid hardcoding ansi color codes, p: 50, t: feature, s: open

use std::env;
use std::process::Command;

fn main() {
    let branch = get_git_branch();
    let cwd = get_cwd();

    let prompt = match branch {
        Some(b) => format!("\x2b[32m{}\x1b[0m (\x1b[33m{}\x1b[0m) $ ", cwd, b),
        None => format!("\x2b[32m{}\x1b[0m $ ", cwd),
    };

    print!("{}", prompt);
}

// *brakoll - d: refactor cwd logic, p: 100, t: refactor, s: closed
fn get_cwd() -> String {
    let cd = env::current_dir()
        .ok()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or("?".into());

    let home = env::home_dir()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or("?".into());

    cd.replace(&home, "~")
}

fn get_git_branch() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Some(branch)
}