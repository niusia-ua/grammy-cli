use anyhow::Result;
use semver::{Version, VersionReq};

const KNOWN_GRAMMY_VERSION_MATCHES_WITH_BOT_API: [(&str, &str); 19] = [
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

pub fn get_bot_api_version(grammy_version: &Version) -> Result<String> {
  let result = KNOWN_GRAMMY_VERSION_MATCHES_WITH_BOT_API
    .iter()
    .find(|(req, _)| VersionReq::parse(req).unwrap().matches(grammy_version));
  match result {
    Some((_, bot_api_version)) => Ok(bot_api_version.to_string()),
    None => Ok(String::from("Unknown")),
  }
}

pub fn clear_semver(version: &str) -> String {
  if version.starts_with("https://deno.land/x/") {
    extract_semver_from_deno_x_url(version)
  } else {
    version.replace(['=', '>', '<', '^', '~'], "")
  }
}

fn extract_semver_from_deno_x_url(url: &str) -> String {
  let package = url.split('/').find(|s| s.starts_with("grammy")).unwrap();
  let version = package.split('@').last().unwrap();
  version.replace('v', "")
}
