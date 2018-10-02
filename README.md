![EVA Logo](https://raw.githubusercontent.com/IanCampbellCode/EVA/master/resources/EVA.jpg)
_Stop forcing new hires to decrypt readmes to build your projects._
_ Arbitrate your builds_

# EVA <sub>_the rusty custom project build manager_</sub>

EVA is a command line tool for managing custom project build orders. Light weight and bare-bones, EVA sets out to do only one thing: _build your projects_. That's it, super simple. Provide EVA with a toml file pointing to repositories, branches and build commands; then sit back and relax. Built with ease of use in mind, this project is meant to be used.

## Table of contents

1. [Getting Started](#getting-started)
2. [Usage](#usage)
3. [Command Line Options](#command-line-options)
4. [Contributing](#contributing)
5. [Design Philosophy](#design-philosophy)
6. [License](#license)

## Getting started

To run EVA you only need to build and compile the project using cargo.

### Installation
The easiest way to install EVA is using cargo's install command. Once eva is installed, it can be called from anywhere.

```console
$ cargo install
```

[back to top](#table-of-contents)

## Usage

The `eva` command allows you to run eva from any location once installed. Once run, EVA will pull and build your projects into your current working directory, unless told to do otherwise using a flag.

```console
$ eva
```

If no other commands are given, EVA will look in the current directory for a repositories.toml file. If the file is not in the current working directory, a custom file location can be specified using the `-f <path>` flag.

```console
$ eva -f /home/git/great_project/repositories.toml
```

For the complete list of options, refer the [Command Line Options](#command-line-options) section below.

[back to top](#table-of-contents)

## Command Line Options

Currently, no command line options are implemented, but these are the planned commands:

- `-h`, `--help`<br />
  Show command line help, including a list of options, and sample use cases.

  - `-t <path>`, `--target <path>`<br />
  Provides a target directory for EVA to place the project files. If the directory does not exist, it will be created.

  - `-f <path>`, `--file <path>`<br />
  Provides a file location for EVA to find a repositories toml file.

  - `-i`, `--ignore-errors`<br />
  Forces EVA to continue running and ignore any non-critical errors.
