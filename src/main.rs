// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

mod app;
mod log;
mod ui;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;

use app::App;
use ui::draw_ui;

use crate::log::log::LogType;

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    app.loading = true;
    app.log
        .print_mes(LogType::Info, "Fetching Containers")
        .await;
    terminal.draw(|f| draw_ui(f, &mut app))?;

    app.fetch_containers().await;
    if !app.loading {
        app.log.print_mes(LogType::Info, "Container loaded").await;
    } else {
        app.log
            .print_mes(LogType::Error, "Failed to fetching container")
            .await;
    }

    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            app.fetch_containers().await;
                        }
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.prev_tab(),
                        KeyCode::Char('d') => app.delete().await,
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.expanded_index.is_some() {
                                app.menu_prev();
                            } else {
                                app.select_prev_container();
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.expanded_index.is_some() {
                                app.menu_next();
                            } else {
                                app.select_next_container();
                            }
                        }
                        KeyCode::Enter => {
                            if app.expanded_index.is_some() {
                                app.execute_menu_action().await;
                            } else {
                                app.toggle_expand();
                            }
                        }
                        KeyCode::Esc => {
                            app.expanded_index = None;
                            app.menu_selection = 0;
                        }
                        KeyCode::Left | KeyCode::Char('h') => app.menu_prev(),
                        KeyCode::Right | KeyCode::Char('l') => app.menu_next(),
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
