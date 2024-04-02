use include_dir::{include_dir, Dir};

pub static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/");

// http://patorjk.com/software/taag/#p=display&h=0&f=Speed&t=grammY
pub const GRAMMY_ASCII_ART: &str = r#"                                                 __  __
_______ _______________ ________ ___ _______ ___ _ \/ /
__  __ `/__  ___/_  __ `/__  __ `__ \__  __ `__ \__  /
_  /_/ / _  /    / /_/ / _  / / / / /_  / / / / /_  /
_\__, /  /_/     \__,_/  /_/ /_/ /_/ /_/ /_/ /_/ /_/
/____/"#;

pub const KNOWN_GRAMMY_VERSION_MATCHES_WITH_BOT_API: [(&str, &str); 19] = [
  ("^1.22.0", "7.2"),
  ("^1.21.0", "7.1"),
  ("^1.20.0", "7.0"),
  ("^1.19.0", "6.9"),
  ("^1.18.0", "6.8"),
  (">=1.16.0, <1.18.0", "6.7"),
  ("^1.15.0", "6.6"),
  ("^1.14.0", "6.5"),
  ("^1.13.0", "6.4"),
  ("^1.12.0", "6.3"),
  ("^1.11.0", "6.2"),
  (">=1.9.0, <1.11.0", "6.1"),
  ("^1.8.0", "6.0"),
  ("^1.7.0", "5.7"),
  ("^1.6.0", "5.6"),
  ("^1.5.0", "5.5"),
  ("^1.4.0", "5.4"),
  ("^1.3.0", "5.3"),
  (">=1.0.0, <1.3.0", "5.2"),
];

pub const KNOWN_CONFIG_FILES: [&str; 3] = ["deno.json", "deno.jsonc", "package.json"];
