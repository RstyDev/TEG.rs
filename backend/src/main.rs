use structs::Map;
mod tasks;
mod run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    crate::run::run().await
}