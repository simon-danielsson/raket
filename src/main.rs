// *brakoll - d: init readme and license, p: 100, t: docs, s: closed
// *brakoll - d: color enums to avoid hardcoding ansi color codes, p: 50, t: feature, s: closed

use std::process::Command;
use std::{env, fmt};

mod ansi;

fn main() {
    // args
    let col_main = "#aab3c0";
    let col_git_br = "#9ec1a3";
    let git_paren = col_main;

    let add_newline = true;
    let entry_on_new_line = true;

    // init
    let mut r: Raket = Raket::new();

    // derive components
    r.get_cwd(col_main);
    r.get_git_branch(col_git_br);
    r.get_entry_sym(col_main);

    // build
    let prompt = r.build(git_paren, entry_on_new_line);

    // print
    let mut newline = "\n";
    if !add_newline {
        newline = "";
    }

    print!("{newline}{prompt}");
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

    // *brakoll - d: args for placing entry on new line and adding space between each command, p: 60, t: feature, s: closed
    fn build(&mut self, col_git_branch_paren: &str, entry_on_new_line: bool) -> String {
        let mut prompt = String::new();
        // add components
        for c in self.components.clone() {
            // handle git branch
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
            } else if c.ctype == ComponentType::Entry {
                if entry_on_new_line {
                    prompt = format!("{}\n{}", prompt, c)
                } else {
                    prompt = format!("{} {}", prompt, c)
                }
            } else {
                prompt = format!("{}{}", prompt, c)
            }
        }
        prompt
    }

    fn get_entry_sym(&mut self, color: &str) {
        self.components.push(PromptComponent {
            ctype: ComponentType::Entry,
            fg_col_hex: color.to_string(),
            content: String::from(" "),
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