mod app;
mod cli;
mod event;
mod tui;
mod ui;

use clap::Parser;
use color_eyre::Result;

use crate::app::App;
use crate::cli::{Cli, Commands, GreetArgs, GreetStyle};
use crate::event::{AppEvent, EventHandler, handle_key_event};

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Tui(args) => cmd_tui(args.tick_rate)?,
        Commands::Greet(args) => cmd_greet(args)?,
    }

    Ok(())
}

fn cmd_tui(tick_rate: u64) -> Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::new();
    let event_handler = EventHandler::new(tick_rate);

    loop {
        terminal.draw(|frame| ui::render(&app, frame))?;

        if let Some(event) = event_handler.next()? {
            match event {
                AppEvent::Key(key) => handle_key_event(&mut app, key),
                AppEvent::Tick => app.tick(),
            }
        }

        if app.should_quit {
            break;
        }
    }

    tui::restore()?;
    Ok(())
}

fn cmd_greet(args: GreetArgs) -> Result<()> {
    for _ in 0..args.count {
        let greeting = match args.style {
            GreetStyle::Normal => format!("Hello, {}!", args.name),
            GreetStyle::Formal => format!("Good day, dear {}.", args.name),
            GreetStyle::Casual => format!("Hey {}!", args.name),
        };
        println!("{}", greeting);
    }
    Ok(())
}
