use std::io::{Result};
use make_serverless_app::{get_config, run};

fn main() ->  Result<()> {
    let config = get_config()?;
    run(&config)
}
