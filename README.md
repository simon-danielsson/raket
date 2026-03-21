<p align="center">
    <img src="media/logo.png" alt="raket" width="200"/>
</p>
  
<p align="center">
  <em>A simple bash prompt.</em>
</p>
  
<p align="center">
        <img src="https://img.shields.io/crates/v/raket?style=flat-square&color=blueviolet&link=https%3A%2F%2Fcrates.io%2Fcrates%raket" alt="Crates.io version" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
    <img src="https://img.shields.io/github/last-commit/simon-danielsson/raket/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#feat">Features</a> •
  <a href="#install">Install</a> •
  <a href="#config">Config</a> •
  <a href="#deps">Dependencies</a> •
  <a href="#license">License</a>
</p>  
  
---
<div id="feat"></div>

## Features
  
- Configuration file that's easy to set up!
  
> [!IMPORTANT]  
> Raket has currently only been tested on MacOS.
  
---
<div id="install"></div>

## Install
  
Install binary using cargo:
  
``` bash
cargo install raket
```
  
Add the following line to your .bashrc:
  
``` bash
PROMPT_COMMAND='PS1="$(raket)"'
```
  
---
<div id="config"></div>

## Config
  
Launch Raket for the first time to generate a default config file:  
`~/.config/raket/config`  
  
``` conf
# === icons ===

ico_entry = 

# === colors ===

col_main = #aab4c0
col_git_branch = #9ec1a3
col_git_paren = #aab3c0

# === settings ===

# add space in between each command
set_space = true

# if true, the prompt input will be below. if false, on the same line
set_prompt_newline = true
```
  
---
<div id="deps"></div>
  
## Dependencies
  
+ [raster](https://github.com/kosinix/raster)  
+ [regex](https://github.com/rust-lang/regex)  
  
---
<div id="license"></div>

## License
  
This project is licensed under the [MIT License](https://github.com/simon-danielsson/raket/blob/main/LICENSE).  
 
