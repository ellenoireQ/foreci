// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Sparkline, Table, Tabs},
};

use crate::{
    app::{App, Tab},
    log::log::LogType,
};

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(6),
        ])
        .split(f.area());

    draw_tabs(f, chunks[0], app);
    draw_content(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
}

fn draw_tabs(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let titles = vec!["Containers", "Images", "Analytics", "Logs", "Settings"];

    let selected = match app.current_tab {
        Tab::Containers => 0,
        Tab::Images => 1,
        Tab::Deployments => 2,
        Tab::Logs => 3,
        Tab::Settings => 4,
    };

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::ALL)
                .title("foreci"),
        )
        .select(selected)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn draw_containers(f: &mut Frame, area: Rect, app: &mut App) {
    let mut items: Vec<ListItem> = Vec::new();

    if app.loading {
        items.push(ListItem::new("⏳ Loading..."));
    } else if app.containers.is_empty() {
        items.push(ListItem::new("No containers. Press 'r' to run a job."));
    } else {
        for (idx, container) in app.containers.clone().iter().enumerate() {
            let display = format!("🖿 {} [{}]", container.name, container.service);
            items.push(
                ListItem::new(display).style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            );
            if app.expanded_index == Some(idx) {
                app.toggle_details();
                let s = format!("{}", app.details_state.clone());
                app.log.print_mes(LogType::Info, s.as_str());
                let menu_items = ["  Start", "  Stop", "  Delete"];
                for (menu_idx, menu_item) in menu_items.iter().enumerate() {
                    let style = if menu_idx == app.menu_selection {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    };
                    items.push(ListItem::new(*menu_item).style(style));
                }
            }
        }
    }

    let list = List::new(items)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().border_type(ratatui::widgets::BorderType::Rounded))
        .highlight_symbol("→ ");
    let block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Details");
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Fill(1)])
        .split(area);
    let inner = block.inner(main[1]);
    f.render_stateful_widget(list, main[0], &mut app.container_state);
    f.render_widget(block, main[1]);
    match app.container_idx {
        Some(idx) => {
            let mans = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Fill(1)])
                .split(inner);

            if app.loading {
                return;
            }
            if let Some(idx) = app.container_idx {
                if let Some(ctn) = app.containers.get(idx) {
                    let rows = vec![
                        Row::new(vec![
                            Cell::from("Name"),
                            Cell::from(":"),
                            Cell::from(ctn.name.clone()),
                        ]),
                        Row::new(vec![
                            Cell::from("Service"),
                            Cell::from(":"),
                            Cell::from(ctn.service.clone()),
                        ]),
                        Row::new(vec![
                            Cell::from("Container"),
                            Cell::from(":"),
                            Cell::from(ctn.container_name.clone()),
                        ]),
                        Row::new(vec![
                            Cell::from("Hostname"),
                            Cell::from(":"),
                            Cell::from(ctn.hostname.clone()),
                        ]),
                        Row::new(vec![
                            Cell::from("Image"),
                            Cell::from(":"),
                            Cell::from(ctn.image.clone()),
                        ]),
                        Row::new(vec![
                            Cell::from("Port"),
                            Cell::from(":"),
                            Cell::from(ctn.ports.clone()),
                        ]),
                        Row::new(vec![
                            Cell::from("Build Ctx"),
                            Cell::from(":"),
                            Cell::from(ctn.build_context.clone()),
                        ]),
                        Row::new(vec![
                            Cell::from("Dockerfile"),
                            Cell::from(":"),
                            Cell::from(ctn.dockerfile.clone()),
                        ]),
                        Row::new(vec![
                            Cell::from("Env"),
                            Cell::from(":"),
                            Cell::from(ctn.environment.join(", ")),
                        ]),
                        Row::new(vec![
                            Cell::from("Volumes"),
                            Cell::from(":"),
                            Cell::from(ctn.volumes.join(", ")),
                        ]),
                        Row::new(vec![
                            Cell::from("Networks"),
                            Cell::from(":"),
                            Cell::from(ctn.networks.join(", ")),
                        ]),
                        Row::new(vec![
                            Cell::from("Restart"),
                            Cell::from(":"),
                            Cell::from(ctn.restart.clone()),
                        ]),
                    ];

                    let widths = [
                        Constraint::Length(12),
                        Constraint::Length(2),
                        Constraint::Fill(1),
                    ];

                    let table = Table::new(rows, widths).block(Block::default()).widths(&[
                        Constraint::Length(12),
                        Constraint::Length(2),
                        Constraint::Fill(1),
                    ]);

                    f.render_widget(table, mans[0]);
                }
            }
        }
        None => {}
    }
}

fn draw_images(f: &mut Frame, area: Rect, app: &mut App) {
    let mut items: Vec<ListItem> = Vec::new();

    if app.loading {
        items.push(ListItem::new("⏳ Loading images..."));
    } else if app.images.is_empty() {
        items.push(ListItem::new("No images found. Press 'i' to refresh."));
    } else {
        for image in &app.images {
            let display = format!("🐳 {}:{}", image.repository, image.tag);
            items.push(
                ListItem::new(display).style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            );
        }
    }

    let list = List::new(items)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().border_type(ratatui::widgets::BorderType::Rounded))
        .highlight_symbol("→ ");

    let block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Image Details");

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Fill(1)])
        .split(area);

    let inner = block.inner(main[1]);
    f.render_stateful_widget(list, main[0], &mut app.image_state);
    f.render_widget(block, main[1]);

    if let Some(idx) = app.image_idx {
        if let Some(image) = app.images.get(idx) {
            let rows = vec![
                Row::new(vec![
                    Cell::from("Repository"),
                    Cell::from(":"),
                    Cell::from(image.repository.clone()),
                ]),
                Row::new(vec![
                    Cell::from("Tag"),
                    Cell::from(":"),
                    Cell::from(image.tag.clone()),
                ]),
                Row::new(vec![
                    Cell::from("Image ID"),
                    Cell::from(":"),
                    Cell::from(image.image_id.clone()),
                ]),
                Row::new(vec![
                    Cell::from("Created"),
                    Cell::from(":"),
                    Cell::from(image.created.clone()),
                ]),
                Row::new(vec![
                    Cell::from("Size"),
                    Cell::from(":"),
                    Cell::from(image.size.clone()),
                ]),
            ];

            let widths = [
                Constraint::Length(12),
                Constraint::Length(2),
                Constraint::Fill(1),
            ];

            let table = Table::new(rows, widths).block(Block::default()).widths(&[
                Constraint::Length(12),
                Constraint::Length(2),
                Constraint::Fill(1),
            ]);

            f.render_widget(table, inner);
        }
    }
}
fn sparkline_window(_app: &mut App, width: usize, _scroll: usize) -> Vec<u64> {
    let len = _app.cpu_data.len();
    if len == 0 {
        return vec![];
    }

    let start = len.saturating_sub(width);
    _app.cpu_data[start..len].to_vec()
}

fn draw_analytics(f: &mut Frame, area: Rect, _app: &mut App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let top_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[0]);

    let bottom_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    let top_left_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title("CPU Usage");

    //    let mut speed_scroll: u64 = 0;
    //    speed_scroll += 20;
    _app.update_cpu_scroll();
    let values = sparkline_window(
        _app,
        top_left_block.inner(top_cols[0]).width as usize,
        _app.scroll_offset,
    );

    let max_val = values.iter().copied().max().unwrap_or(10).max(10);

    let cpu_sparkline = Sparkline::default()
        .block(Block::default())
        .data(&values)
        .max(max_val)
        .style(Style::default().fg(Color::Green));

    f.render_widget(top_left_block.clone(), top_cols[0]);
    f.render_widget(cpu_sparkline, top_left_block.inner(top_cols[0]));

    let top_right_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Memory Usage");
    let top_right_content = Paragraph::new("Used: 8.2 GB\nFree: 7.8 GB\nTotal: 16 GB")
        .block(Block::default())
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(top_right_block.clone(), top_cols[1]);
    f.render_widget(top_right_content, top_right_block.inner(top_cols[1]));

    let bottom_left_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Network I/O");
    let bottom_left_content =
        Paragraph::new("↓ Download: 1.2 MB/s\n↑ Upload: 0.5 MB/s\nTotal: 45 GB")
            .block(Block::default())
            .style(Style::default().fg(Color::Yellow));
    f.render_widget(bottom_left_block.clone(), bottom_cols[0]);
    f.render_widget(bottom_left_content, bottom_left_block.inner(bottom_cols[0]));

    let bottom_right_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Disk Usage");
    let bottom_right_content = Paragraph::new("Used: 234 GB\nFree: 266 GB\nTotal: 500 GB")
        .block(Block::default())
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(bottom_right_block.clone(), bottom_cols[1]);
    f.render_widget(
        bottom_right_content,
        bottom_right_block.inner(bottom_cols[1]),
    );
}

fn draw_content(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    let title = match app.current_tab {
        Tab::Containers => "Containers",
        Tab::Images => "Images",
        Tab::Deployments => "Analytics",
        Tab::Logs => "Logs",
        Tab::Settings => "Settings",
    };

    let block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title(title);

    f.render_widget(block.clone(), area);

    let inner = block.inner(area);

    match app.current_tab {
        Tab::Containers => draw_containers(f, inner, app),
        Tab::Images => draw_images(f, inner, app),
        Tab::Deployments => draw_analytics(f, inner, app),
        Tab::Logs => {
            //
        }
        Tab::Settings => {
            //
        }
    }
}

fn draw_footer(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    let rows_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .split(area);

    let footer = Paragraph::new(
        "q: Quit | r: Refresh | Tab: Switch | ↕: Select | ←/→: Scroll Log | Enter: Menu",
    )
    .block(Block::default().borders(Borders::NONE))
    .centered();

    let footersc_title = app.log.to_display_string();
    let footer1 = Paragraph::new(footersc_title)
        .block(
            Block::default()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::ALL)
                .title("Logs"),
        )
        .scroll((0, app.log_scroll));

    f.render_widget(footer, rows_column[1]);
    f.render_widget(footer1, rows_column[0]);
}
