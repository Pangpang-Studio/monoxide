use anyhow::Result;
use dioxus_devtools::subsecond;

use monoxide_font::make_font;

#[tokio::main]
async fn main() -> Result<()> {
    monoxide_playground::Playground::dispatch(|| subsecond::call(make_font)).await
}
