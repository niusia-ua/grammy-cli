# grammY CLI

An **unofficial** command line interface for [grammY](https://grammy.dev).
You can use it to quickly create a project from existing [templates](https://github.com/niusia-ua/grammy-cli/tree/main/templates).
More useful features are planned to be added in the future.

> [!WARNING]
> **Created for the author's educational purposes only.**
> Therefore, the author does not guarantee that this application will work correctly and will be supported.

> [!NOTE]
> Currently, the grammY team does not have a goal to develop a similar CLI.
> The reasons are described [here in the warning](https://github.com/grammyjs/create-grammy/blob/main/README.md).
> In the opinion of the author of this repository, if an official CLI for grammY is developed, it should be done exclusively on Deno.
> It will be more relevant to the grammY project and allow anyone familiar with at least Node.js to contribute to the CLI.

## Install

Currently, you need to have Rust installed to install the grammY CLI.

From source:

```shell
# First, clone the repository
# cd grammy-cli
cargo install
```

From GitHub:

```shell
cargo install --git https://github.com/niusia-ua/grammy-cli.git
```

## Usage

Currently, you can only simply create a new project with `grammy-cli new`.

![An example of using the new command](/assets/command-new-example.png)

You can also use the `grammy-cli info` to display some system information in the terminal.

![An example of using the info command](/assets/command-info-example.png)
