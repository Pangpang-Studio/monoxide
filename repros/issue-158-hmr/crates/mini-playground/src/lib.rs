use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use dioxus_devtools::subsecond;
use tokio::sync::mpsc;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Playground {
    #[clap(subcommand)]
    cmd: Subcommand,
}

#[derive(clap::Parser)]
enum Subcommand {
    Serve(ServeCommand),
}

#[derive(Debug, clap::Parser)]
pub struct ServeCommand {
    #[clap(long, default_value = "3030")]
    _port: u16,
}

impl Playground {
    pub async fn dispatch(mut on_reload: impl FnMut()) -> Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
            )
            .init();

        let args = Playground::parse();
        match args.cmd {
            Subcommand::Serve(cmd) => cmd.run(&mut on_reload).await,
        }
    }
}

impl ServeCommand {
    async fn run(self, on_reload: &mut impl FnMut()) -> Result<()> {
        let (tx, mut rx) = mpsc::channel::<()>(8);

        dioxus_devtools::connect_subsecond();

        let send = Arc::new(move || {
            _ = tx.try_send(());
        });

        send();
        subsecond::register_handler(send);

        while rx.recv().await.is_some() {
            on_reload();
        }

        Ok(())
    }
}
