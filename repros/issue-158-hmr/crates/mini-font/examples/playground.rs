use anyhow::Result;
use dioxus_devtools::subsecond;
use mini_font::on_reload;

#[tokio::main]
async fn main() -> Result<()> {
    mini_playground::Playground::dispatch(|| subsecond::call(on_reload)).await
}
