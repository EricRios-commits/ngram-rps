const DEFAULT_WINDOW_SIZE: usize = 3;
const DEFAULT_BACKOFF: bool = false;

#[derive(Debug)]
pub struct Config {
    window_size: usize,
    use_backoff: bool,
}

impl Config {
  pub fn window_size(&self) -> usize {
    self.window_size
  }

  pub fn use_backoff(&self) -> bool {
    self.use_backoff
  }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_size: DEFAULT_WINDOW_SIZE,
            use_backoff: DEFAULT_BACKOFF,
        }
    }
}

pub fn parse_config(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let mut config = Config::default();
    for arg in args {
        match arg.as_str() {
            "--backoff" => config.use_backoff = true,
            other => {
                config.window_size = other
                    .parse()
                    .map_err(|_| format!("Unrecognized argument {other}"))?;
            }
        }
    }
    Ok(config)
}