// *brakoll - d: init readme and license, p: 100, t: docs, s: closed
// *brakoll - d: new info, p: 100, t: docs, s: closed
// *brakoll - d: color enums to avoid hardcoding ansi color codes, p: 50, t: feature, s: closed
// *brakoll - d: update readme with new info, p: 60, t: docs, s: closed

use std::process::Command;
use std::{env, fmt, io};

use crate::config::ConfigVars;

mod ansi;
mod config;

// *brakoll - d: add extra git details, p: 20, t: feature, s: open
// *brakoll - d: add cargo env status, p: 10, t: feature, s: open
// *brakoll - d: add variable/logic for failed return code icon, p: 30, t: feature, s: prog

// *brakoll - d: capture return code, p: 30, t: feature, s: closed
fn status_code() -> u32 {
    let args: Vec<String> = env::args().collect();

    if let Some(status_arg) = args.iter().find(|a| a.starts_with("--status=")) {
        let status = &status_arg["--status=".len()..];
        return status.parse().unwrap();
    } else {
        return 0;
    }
}

fn main() -> io::Result<()> {
    // config variables
    let vars: ConfigVars = config::get()?;

    // vars.debug_print();

    // return code of last run program
    let status = status_code();
    println!("last status: {status}");

    // init
    let mut r: Raket = Raket::new();

    // derive components
    r.get_cwd(&vars.col_main);
    r.get_git_branch(&vars.col_git_branch);
    r.get_entry_sym(&vars.col_main, &vars.ico_entry);

    // build
    let prompt = r.build(&vars.col_git_paren, vars.set_prompt_newline);

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
    fn build(&mut self, col_git_branch_paren: &str, set_prompt_newline: bool) -> String {
        let mut prompt = String::new();
        // add components
        for c in self.components.clone() {
            // git branch
            if c.ctype == ComponentType::GitBranch && c.content != "" {
                let l_par = ansi::apply_color(
                    col_git_branch_paren.to_string(),
                    "(".to_string(),
                );
                let r_par = ansi::apply_color(
                    col_git_branch_paren.to_string(),
                    ")".to_string(),
                );
                prompt = format!(
                    "{pr} {lp}{br}{rp}",
                    pr = prompt,
                    lp = l_par,
                    br = c,
                    rp = r_par
                )
                // prompt entry
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

    fn get_entry_sym(&mut self, color: &str, icon: &str) {
        self.components.push(PromptComponent {
            ctype: ComponentType::Entry,
            fg_col_hex: color.to_string(),
            content: String::from(icon),
        });
    }

    // *brakoll - d: refactor cwd logic, p: 100, t: refactor, s: closed
    fn get_cwd(&mut self, color: &str) {
        let cd = env::current_dir()
            .ok()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("?".into());

        let home = env::home_dir()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("?".into());

        let cwd_content = cd.replace(&home, &self.home_sym);

        self.components.push(PromptComponent {
            ctype: ComponentType::CWD,
            fg_col_hex: color.to_string(),
            content: cwd_content,
        });
    }

    fn get_git_branch(&mut self, color: &str) {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .ok()
            .unwrap();

        if !output.status.success() {
            self.components.push(PromptComponent {
                ctype: ComponentType::GitBranch,
                fg_col_hex: color.to_string(),
                content: String::new(),
            });
            return;
        }

        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

        self.components.push(PromptComponent {
            ctype: ComponentType::GitBranch,
            fg_col_hex: color.to_string(),
            content: format!("{}", branch).to_string(),
        });
    }
}