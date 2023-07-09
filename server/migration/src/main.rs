use std::env;

use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");


    cli::run_cli(migration::Migrator).await;
}
