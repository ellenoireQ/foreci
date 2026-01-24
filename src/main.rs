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
use rand::Rng;
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
    app.log.print_mes(LogType::Info, "Fetching Containers");
    terminal.draw(|f| draw_ui(f, &mut app))?;

    app.fetch_containers().await;
    if !app.loading {
        app.log.print_mes(LogType::Info, "Container loaded");
    } else {
        app.log
            .print_mes(LogType::Error, "Failed to fetching container");
    }

    loop {
        app.poll_logs();
        app.poll_analytics(); // Always poll analytics to keep graph moving

        if app.current_tab == app::Tab::Deployments {
            if let Some(ref container_id) = app.selected_container_id.clone() {
                app.start_analytics_stream(&container_id);
            }
        }

        terminal.draw(|f| draw_ui(f, &mut app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') | KeyCode::Char('R') => match app.current_tab {
                            app::Tab::Containers => app.fetch_containers().await,
                            app::Tab::Images => app.fetch_images().await,
                            app::Tab::Deployments => app.fetch_running_containers().await,
                            _ => {}
                        },
                        KeyCode::Tab => {
                            app.next_tab();
                            if app.current_tab == app::Tab::Images && app.images.is_empty() {
                                app.fetch_images().await;
                            }
                            if app.current_tab == app::Tab::Deployments
                                && app.running_containers.is_empty()
                            {
                                app.fetch_running_containers().await;
                            }
                        }
                        KeyCode::BackTab => {
                            app.prev_tab();
                            if app.current_tab == app::Tab::Images && app.images.is_empty() {
                                app.fetch_images().await;
                            }
                            if app.current_tab == app::Tab::Deployments
                                && app.running_containers.is_empty()
                            {
                                app.fetch_running_containers().await;
                            }
                        }
                        KeyCode::Char('d') => app.delete().await,
                        KeyCode::Up | KeyCode::Char('k') => match app.current_tab {
                            app::Tab::Containers => {
                                if app.expanded_index.is_some() {
                                    app.menu_prev();
                                } else {
                                    app.select_prev_container();
                                }
                            }
                            app::Tab::Images => {
                                if app.image_expanded_index.is_some() {
                                    app.image_menu_prev();
                                } else {
                                    app.select_prev_image();
                                }
                            }
                            app::Tab::Deployments => app.select_prev_running_container(),
                            _ => {}
                        },
                        KeyCode::Down | KeyCode::Char('j') => match app.current_tab {
                            app::Tab::Containers => {
                                if app.expanded_index.is_some() {
                                    app.menu_next();
                                } else {
                                    app.select_next_container();
                                }
                            }
                            app::Tab::Images => {
                                if app.image_expanded_index.is_some() {
                                    app.image_menu_next();
                                } else {
                                    app.select_next_image();
                                }
                            }
                            app::Tab::Deployments => app.select_next_running_container(),
                            _ => {}
                        },
                        KeyCode::Enter => match app.current_tab {
                            app::Tab::Containers => {
                                if app.expanded_index.is_some() {
                                    app.execute_menu_action().await;
                                } else {
                                    app.toggle_expand();
                                }
                            }
                            app::Tab::Images => {
                                if app.image_expanded_index.is_some() {
                                    app.execute_image_menu_action().await;
                                } else {
                                    app.toggle_image_expand();
                                }
                            }
                            app::Tab::Deployments => {
                                app.select_running_container();
                            }
                            _ => {}
                        },
                        KeyCode::Esc => {
                            app.expanded_index = None;
                            app.menu_selection = 0;
                            app.image_expanded_index = None;
                            app.image_menu_selection = 0;
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            if app.expanded_index.is_some() {
                                app.menu_prev();
                            } else {
                                app.scroll_log_left();
                            }
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            if app.expanded_index.is_some() {
                                app.menu_next();
                            } else {
                                app.scroll_log_right();
                            }
                        }
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
