#[path = "font.rs"]
mod font;

use anyhow::Result;
use dioxus_devtools::subsecond;

use crate::font::make_font;

#[tokio::main]
async fn main() -> Result<()> {
    dioxus_devtools::connect_subsecond();
    monoxide_playground::Playground::dispatch(|| subsecond::call(make_font)).await
}
