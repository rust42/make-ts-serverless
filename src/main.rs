use std::io::{Result};
use make_serverless_app::App;

fn main() ->  Result<()> {
    App::new()?.run()
}
