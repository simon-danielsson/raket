// *brakoll - d: init readme and license, p: 100, t: docs, s: closed
// *brakoll - d: new info, p: 100, t: docs, s: closed
// *brakoll - d: color enums to avoid hardcoding ansi color codes, p: 50, t: feature, s: closed
// *brakoll - d: update readme with new info, p: 60, t: docs, s: closed

use std::process::Command;
use std::{env, fmt, io};

use crate::config::ConfigVars;

mod ansi;
mod config;

// *brakoll - d: add extra git details, p: 20, t: feature, s: closed
// *brakoll - d: add variable/logic for failed return code icon, p: 30, t: feature, s: closed

// *brakoll - d: capture return code, p: 30, t: feature, s: closed
/// returns true if the last program exited successfully
fn status_code() -> bool {
    let args: Vec<String> = env::args().collect();

    if let Some(status_arg) = args.iter().find(|a| a.starts_with("--status=")) {
        let status = &status_arg["--status=".len()..];
        match status.parse().unwrap() {
            0 => return true,
            _ => return false,
        }
    } else {
        return true;
    }
}

fn main() -> io::Result<()> {
    // config variables
    let vars: ConfigVars = config::get()?;

    // init
    let mut r: Raket = Raket::new();

    {
        let content = r.get_cwd();
        r.components.push(PromptComponent {
            ctype: ComponentType::CWD,
            fg_col_hex: vars.col_main.to_string(),
            content: format!("{}", content).to_string(),
        });
    }

    // *brakoll - d: place component additions in main fn instead, p: 100, t: refactor, s: closed

    // *brakoll - d: move is_empty() methods to more appropriate places, p: 10, t: refactor, s: closed
    // *brakoll - d: parents in git branch still there if no git repo, p: 100, t: fix, s: closed
    if vars.set_show_git_branch {
        let content = r.get_git_branch();
        if !content.is_empty() {
            r.components.push(PromptComponent {
                ctype: ComponentType::GitBranch,
                fg_col_hex: vars.col_git_branch.to_string(),
                content: format!("{}", content).to_string(),
            });
        }
    }

    if vars.set_show_git_status {
        let content = r.get_git_status();
        if !content.is_empty() {
            r.components.push(PromptComponent {
                ctype: ComponentType::GitStatus,
                fg_col_hex: vars.col_git_status.to_string(),
                content,
            });
        }
    }

    // *brakoll - d: add parens to cargo env, p: 50, t: feature, s: closed
    if vars.set_show_cargo_env {
        let content = r.get_cargo_env();
        if !content.is_empty() {
            r.components.push(PromptComponent {
                ctype: ComponentType::CargoEnv,
                fg_col_hex: vars.col_cargo_env.to_string(),
                content: format!("via  {}", content).to_string(),
            });
        }
    }

    // *brakoll - d: implement python uv env setting, p: 100, t: feature, s: closed
    if vars.set_show_uv_env {
        let content = r.get_uv_env();
        if !content.is_empty() {
            r.components.push(PromptComponent {
                ctype: ComponentType::UvEnv,
                fg_col_hex: vars.col_uv_env.to_string(),
                content: format!("via  {}", content).to_string(),
            });
        }
    }

    r.get_entry_sym(
        &vars.col_entry_success,
        &vars.ico_entry_success,
        &vars.col_entry_failed,
        &vars.ico_entry_failed,
    );

    // build
    let prompt = r.build(vars.set_prompt_newline);

    // print
    let mut newline = "\n";
    if !vars.set_space {
        newline = "";
    }

    print!("{newline}{prompt}");
    Ok(())
}

#[derive(Clone, PartialEq)]
enum ComponentType {
    UvEnv,
    CargoEnv,
    GitStatus,
    GitBranch,
    Entry,
    CWD,
}

// *brakoll - d: abstract color args and component building, p: 100, t: refactor, s: closed
#[derive(Clone)]
struct PromptComponent {
    ctype: ComponentType,
    fg_col_hex: String,
    content: String,
}

impl fmt::Display for PromptComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            ansi::apply_color(self.fg_col_hex.clone(), self.content.clone())
        )
    }
}

struct Raket {
    components: Vec<PromptComponent>,
    home_sym: String,
}

impl Raket {
    fn new() -> Self {
        Self {
            components: Vec::new(),
            home_sym: String::from("~"),
        }
    }

    // *brakoll - d: args for placing entry on new line and adding set_space between each command, p: 60, t: feature, s: closed
    fn build(&mut self, set_prompt_newline: bool) -> String {
        let mut prompt = String::new();
        // add components
        for c in self.components.clone() {
            // git branch
            if c.ctype == ComponentType::GitBranch {
                prompt = format!("{} ({})", prompt, c)
                // *brakoll - d: remove brackets around cargo env, p: , t: fix, s: closed
            } else if c.ctype == ComponentType::CargoEnv {
                prompt = format!("{} {}", prompt, c)
            } else if c.ctype == ComponentType::UvEnv {
                prompt = format!("{} {}", prompt, c)
            } else if c.ctype == ComponentType::Entry {
                if set_prompt_newline {
                    prompt = format!("{}\n{} ", prompt, c)
                } else {
                    prompt = format!("{} {} ", prompt, c)
                }
            } else {
                prompt = format!("{}{}", prompt, c)
            }
        }
        prompt
    }

    fn get_entry_sym(
        &mut self,
        color_success: &str,
        icon_success: &str,
        color_failed: &str,
        icon_failed: &str,
    ) {
        let status = status_code();
        // println!("last status: {status}");
        let (entry_icon, entry_col) = match status {
            true => (icon_success, color_success),
            false => (icon_failed, color_failed),
        };

        self.components.push(PromptComponent {
            ctype: ComponentType::Entry,
            fg_col_hex: entry_col.to_string(),
            content: entry_icon.to_string(),
        });
    }

    // *brakoll - d: refactor cwd logic, p: 100, t: refactor, s: closed
    fn get_cwd(&mut self) -> String {
        let cd = env::current_dir()
            .ok()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("?".into());

        let home = env::home_dir()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("?".into());

        cd.replace(&home, &self.home_sym)
    }

    fn get_git_status(&mut self) -> String {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .ok()
            .unwrap();
        let output = String::from_utf8_lossy(&output.stdout).trim().to_string();

        let statuses = [("M ", ""), ("D ", ""), ("??", "")];

        let content = statuses
            .iter()
            .filter_map(|(pattern, icon)| {
                let count = output.matches(pattern).count();
                (count > 0).then(|| format!(" {icon} {count}"))
            })
            .collect::<Vec<_>>()
            .join("");
        content

        // let output2 = Command::new("git")
        //     .args(["status", "-sb"])
        //     .output()
        //     .ok()
        //     .unwrap();
        // let output2 = String::from_utf8_lossy(&output2.stdout).trim().to_string();
        // let mut git_remote_status = String::new();
        // if output2.contains("ahead") {
        //     git_remote_status = "󰃄".to_string();
        // } else if output2.contains("behind") {
        //     git_remote_status = "󱍺".to_string();
        // }
        //
        // format!("{}{}", git_remote_status, content)
    }

    fn get_uv_env(&mut self) -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg("uv version")
            .output()
            .expect("failed to execute command");

        let output = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // *brakoll - d: unwrapping leads to panic at uv env formatting, p: 100, t: fix, s: closed
        let mut version = "";
        if output != "" {
            (_, version) = output.split_once(' ').unwrap();
        }
        version.to_string()
    }

    // *brakoll - d: add cargo env status, p: 0, t: feature, s: closed
    fn get_cargo_env(&mut self) -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(
                "cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'",
            )
            .output()
            .expect("failed to execute command");

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn get_git_branch(&mut self) -> String {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .ok()
            .unwrap();
        if !output.status.success() {
            return String::new();
        }
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
}