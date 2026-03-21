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
  <a href="#usage">Usage</a> •
  <a href="#deps">Dependencies</a> •
  <a href="#license">License</a>
</p>  
  
---
<div id="feat"></div>

## Features
  
- For designing your folder structures Raket features its own scripting language with support for custom variables, striking a balance between flexibility and ease-of-use. [Neovim plugin for syntax highlighting and such is available here](https://github.com/simon-danielsson/raket.nvim).
- Subcommands for normalizing, correcting and sorting raw audio stems in record-time (not implemented yet).
  
---
<div id="install"></div>

## Install
  
``` bash
cargo install raket
```
  
---
<div id="usage"></div>

## Usage
  
### CLI
  
Generate initialized raket template file (without target dir, current directory is assumed):  
  
``` terminal
raket init <target dir>
```
  
Generate folder structure from template file (without target dir, current directory is assumed):  
  
``` terminal
raket -f <raket template file> <target dir>
```
  
In your raket template file you can set variables,
and they can all be overridden through the CLI at your leisure. Just make sure that the variable you're overriding exists in you raket file and that you're supplying the right kind of value! (e.g a folder variable override would need a filepath, a string variable would need a string and so on)
  
``` terminal
$ raket ... -var variable="value"
$ raket ... -v variable="value"
$ raket ... --var variable="value"

Example:
$ raket -f template.raket --var stems="/Users/usr/Downloads/stems/" --var client="New Client" --var rpp="/Users/usr/music/templates/mix.rpp"
```
  
Display help and version information:  
  
``` terminal
raket help
```
  
### Template file (.raket)
  
The core workflow of Raket consists of feeding the CLI with raket template files. A raket file (file with the .raket extension) is divided up into three sections: variables, settings and hierarchy. This format accepts comments (prefixed with '#'), but comment lines should be separate from the parameter lines since doing otherwise could lead to undefined behaviour. For more details, read the comments in the example below!
  
#### Full example
  
``` conf
# there are only three different datatypes to keep track of:

# 1: <string> - used for name variables

# 2: <file> - an empty generic file, where the extension in its name will specify the extension of the file once generated
# variables with the datatype <file> will inherit the same extension logic as a generic file, only that the contents of the file variable path supplied will be the contents of the file generated

# 3: <folder> - a generic folder
# variables with the datatype <folder> will inherit the contents of the folder specified in the variables section

variables [
    # built-in: date
    client: string = "client"
    project: string = "project"
    service: string = "mix"
    rpp: file = "/Users/usr/music/templates/mix.RPP"
    stems: folder = "/Users/usr/Downloads/stems_from_client/"
    # you can create any variables you want
    fruit: string = "banana"
]

settings [
    format_names: false
    format_date: "%d-%m-%Y"
]

hierarchy [
    folder "[$service] $client - $project, $date" {
        folder "project" {
            rpp "$project $date.RPP"
            file "$fruit notes.md"
        }

        # it's legal to add extra files/folders to a folder variable
        stems "stems" {
            file "todo.md"
        }

        folder "export" {
            folder "$fruit drafts" {}
        }
    }
]
```
  
---
<div id="deps"></div>
  
## Dependencies
  
+ [chrono](https://github.com/chronotope/chrono)  
  
---
<div id="license"></div>

## License
  
This project is licensed under the [MIT License](https://github.com/simon-danielsson/raket/blob/main/LICENSE).  
 
