use color_eyre::eyre::Result;
use lazy_static::lazy_static;

use crate::build_time::BUILD_TIME;
use crate::cli::command::Command;
use crate::config::Config;
use crate::output::Output;

#[derive(Debug, clap::Args)]
#[clap(about = "Show rtx version", alias = "-v", alias = "v")]
pub struct Version {}

lazy_static! {
    pub static ref VERSION: String = format!(
        "{} (built on {})",
        env!("CARGO_PKG_VERSION"),
        BUILD_TIME.format("%Y-%m-%d")
    );
}

impl Command for Version {
    fn run(self, _config: Config, out: &mut Output) -> Result<()> {
        let v = VERSION.to_string();
        rtxprintln!(out, "{v}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_cli;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_version() {
        let stdout = assert_cli!("version");
        assert_str_eq!(stdout, VERSION.to_string() + "\n");
    }
}
