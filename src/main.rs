use crossterm::event::KeyCode;
use disku::Config;
use ratatui::DefaultTerminal;
use ratatui::widgets::TableState;
use std::path::Path;
use std::{env, io, process};

mod ui;

fn main() {
    let mut args = env::args();
    args.next();

    let root = if args.len() > 2 {
        ".".to_string()
    } else {
        args.next().unwrap_or_else(|| ".".to_string())
    };

    if let Err(e) = run(root) {
        eprintln!("Program crashed: {e}");
        process::exit(1);
    }
}

fn run(root: String) -> Result<(), &'static str> {
    let root_path = Path::new(&root);
    if !root_path.exists() {
        return Err("No such file or directory");
    }

    let config = Config::build(root)?;

    let mut table_state = TableState::default();
    table_state.select_first();
    ratatui::run(|terminal| app(terminal, config, &mut table_state))
        .map_err(|_| "TUI execution failed")?;

    Ok(())
}

fn app(
    terminal: &mut DefaultTerminal,
    config: Config,
    table_state: &mut TableState,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, &config, table_state))?;

        if let Some(key) = crossterm::event::read()?.as_key_event() {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => table_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => table_state.select_previous(),
                KeyCode::Char('g') | KeyCode::Home => table_state.select_first(),
                KeyCode::Char('G') | KeyCode::PageDown => table_state.select_last(),
                KeyCode::Char('q') => break Ok(()),
                _ => {}
            }
        }
    }
}
