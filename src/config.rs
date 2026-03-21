use std::{env, fs, io, path::PathBuf};

const DEF_CONF: &str = include_str!("./default_config");

// *brakoll - d: add config file logic and parser, p: 60, t: feature, s: closed
pub fn get() -> io::Result<ConfigVars> {
    let home = env::home_dir().unwrap();
    let config_folder_path = home.join(".config/raket/");
    let config_file_path = home.join(".config/raket/config");

    if !config_file_path.exists() {
        gen_config(&config_folder_path)?;
    }

    return Ok(parse_config(config_file_path)?);
}

fn gen_config(dest: &PathBuf) -> io::Result<()> {
    fs::create_dir(dest)?;
    let file = dest.join("config");
    fs::write(file, DEF_CONF)?;
    Ok(())
}

fn parse_config(file: PathBuf) -> io::Result<ConfigVars> {
    let contents = fs::read_to_string(file)?;

    let mut conf = ConfigVars::new();

    let iter = contents.lines().into_iter();
    for lines in iter {
        let l = lines.trim();

        match l {
            // skip comments and empty lines
            l if l.starts_with("#") => {
                continue;
            }
            l if l.is_empty() => {
                continue;
            }

            // icons
            l if l.starts_with("ico_entry_success") => {
                conf.ico_entry_success = get_value(lines);
            }
            l if l.starts_with("ico_entry_failed") => {
                conf.ico_entry_failed = get_value(lines);
            }

            // colors
            l if l.starts_with("col_entry_success") => {
                conf.col_entry_success = get_value(lines);
            }
            l if l.starts_with("col_entry_failed") => {
                conf.col_entry_failed = get_value(lines);
            }
            l if l.starts_with("col_main") => {
                conf.col_main = get_value(lines);
            }
            l if l.contains("col_git_branch") => {
                conf.col_git_branch = get_value(lines);
            }
            l if l.starts_with("col_git_status") => {
                conf.col_git_status = get_value(lines);
            }

            // settings
            l if l.starts_with("set_space") => {
                conf.set_space = parse_bool(&get_value(l));
            }
            l if l.starts_with("set_prompt_newline") => {
                conf.set_prompt_newline = parse_bool(&get_value(l));
            }
            l if l.starts_with("set_show_git_branch") => {
                conf.set_show_git_branch = parse_bool(&get_value(l));
            }
            l if l.starts_with("set_show_git_status") => {
                conf.set_show_git_status = parse_bool(&get_value(l));
            }
            _ => {}
        }
    }

    // placeholder
    Ok(conf)
}

fn parse_bool(s: &str) -> bool {
    s.to_ascii_lowercase().starts_with("tr")
}

fn get_value(l: &str) -> String {
    let (_, v) = l.split_once('=').unwrap();

    v.trim().to_string()
}

// *brakoll - d: collect variables in struct, p: 50, t: refactor, s: closed
pub struct ConfigVars {
    pub ico_entry_success: String,
    pub ico_entry_failed: String,
    pub col_entry_success: String,
    pub col_entry_failed: String,
    pub col_main: String,
    pub col_git_status: String,
    pub col_git_branch: String,
    pub set_space: bool,
    pub set_prompt_newline: bool,
    pub set_show_git_branch: bool,
    pub set_show_git_status: bool,
}

impl ConfigVars {
    /// set defaults
    fn new() -> Self {
        Self {
            ico_entry_success: "".to_string(),
            ico_entry_failed: "󰯈".to_string(),
            col_entry_success: "#9ec1a3".to_string(),
            col_entry_failed: "#aa4465".to_string(),
            col_main: "#aab3c0".to_string(),
            col_git_status: "#aab3c0".to_string(),
            col_git_branch: "#9ec1a3".to_string(),
            set_space: true,
            set_prompt_newline: true,
            set_show_git_branch: true,
            set_show_git_status: true,
        }
    }
    #[allow(dead_code)]
    pub fn debug_print(&mut self) {
        println!("icon entry: {}", self.ico_entry_success);
        println!("icon entry: {}", self.ico_entry_failed);
        println!("color main: {}", self.col_main);
        println!("col git branch: {}", self.col_git_branch);
        println!("col git paren: {}", self.col_git_status);
        println!("set space: {}", self.set_space);
        println!("set prompt nl: {}", self.set_prompt_newline);
    }
}
