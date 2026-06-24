mod run;
mod structs;
mod tasks;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    crate::run::run().await
}
