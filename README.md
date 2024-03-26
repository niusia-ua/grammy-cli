# grammY CLI

An **unofficial** command line interface for [grammY](https://grammy.dev).

**Created for the author's educational purposes only.**

Currently, the grammY team does not have a goal to develop a similar CLI.
The reasons are described [here in the warning](https://github.com/grammyjs/create-grammy/blob/main/README.md).
In the opinion of the author of this repository, if an official CLI for grammY is developed, it should be done exclusively on Deno, as it will be more relevant to the grammY project and allow anyone familiar with at least Node.js to contribute to the CLI.

## Planned features

### `grammy new <name> [options]`

Creates and initializes a new grammY project.

### `grammy generate <schematic> <name> [options]`

Generates and/or modifies files based on a schematic.

### `grammy add <name> [options]`

Imports a library that has been packaged as a grammY library, running its install schematic.

### `grammy info`

Displays information about installed grammY packages and other helpful system info.
