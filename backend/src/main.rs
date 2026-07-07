use ::structs::initialize_map;
use crate::structs::initialize;

mod run;
mod structs;
mod tasks;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    initialize();
    initialize_map();
    crate::run::run().await
}

