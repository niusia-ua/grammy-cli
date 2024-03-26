use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help = true)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
  /// Generate grammY project
  New {
    /// Project name according to NPM rules
    /// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#name
    name: String,

    /// Runtime to use
    #[arg(long, value_enum, default_value = "deno")]
    runtime: Runtime,
  },

  /// Generate grammY component
  #[clap(visible_alias = "g")]
  Generate {
    /// Schematic to use
    #[arg(value_enum)]
    schematic: Schematic,

    /// Component name
    name: String,
  },

  /// Add grammY plugins
  Add {
    /// List of plugins to install
    #[arg(value_enum)]
    plugin: Vec<Plugin>,
  },

  /// Display project information, installed plugins, and other useful system information
  Info,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum Runtime {
  NodeJS,
  Deno,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum Schematic {
  Handler,
  Middleware,
  Conversation,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum Plugin {
  Sessions,
  Conversations,
  Menu,
  StatelessQuestion,
  Runner,
  Hydrate,
  AutoRetry,
  TransformerThrottler,
  Ratelimiter,
  Files,
  I18n,
  Fluent,
  Router,
  Emoji,
  ParseMode,
  ChatMembers,
}

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Command::New { name, runtime } => println!(
      "You want to create a new project called `{}` that will use the {:?} engine!",
      name, runtime
    ),
    Command::Generate { schematic, name } => println!(
      "You want to generate the `{:?}` grammY component called `{}`!",
      schematic, name
    ),
    Command::Add { plugin } => println!(
      "You want to add the following grammY plugins: {:?}!",
      plugin
    ),
    Command::Info => println!("You want to display information about your project!"),
  };
}
