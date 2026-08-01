use dioxus_devtools::subsecond;
use monoxide_font::make_font;
use monoxide_playground::Playground;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Playground::dispatch(|| subsecond::call(make_font)).await
}
