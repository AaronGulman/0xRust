
mod routes;
mod db;
mod config;
mod state;
mod error;
mod server;

use config::init;
use server::server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
   db::main().await?;
   init();
   server().await?;
   Ok(())

}