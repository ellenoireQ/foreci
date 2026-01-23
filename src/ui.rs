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

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1}GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}KB", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

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
fn sparkline_mem_window(_app: &mut App, width: usize, _scroll: usize) -> Vec<u64> {
    let len = _app.mem_data.len();
    if len == 0 {
        return vec![];
    }

    let start = len.saturating_sub(width);
    _app.mem_data[start..len].to_vec()
}

fn sparkline_net_rx_window(_app: &mut App, width: usize) -> Vec<u64> {
    let len = _app.net_data.len();
    if len == 0 {
        return vec![];
    }

    let start = len.saturating_sub(width);
    _app.net_data[start..len].iter().map(|n| n.net_rx).collect()
}

fn sparkline_net_tx_window(_app: &mut App, width: usize) -> Vec<u64> {
    let len = _app.net_data.len();
    if len == 0 {
        return vec![];
    }

    let start = len.saturating_sub(width);
    _app.net_data[start..len].iter().map(|n| n.net_tx).collect()
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

    // CPU Usage with percentage
    let cpu_percent = _app.analytics.cpu_percent;
    let cpu_title = format!("CPU Usage - {:.2}%", cpu_percent);
    let top_left_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title(cpu_title);

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

    // Memory Usage with percentage
    let mem_percent = _app.analytics.mem_percent;
    let mem_title = format!("Memory Usage - {:.2}%", mem_percent);
    let top_right_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title(mem_title);

    let values_mem = sparkline_mem_window(
        _app,
        top_right_block.inner(top_cols[1]).width as usize,
        _app.scroll_offset,
    );
    let max_vals = values_mem.iter().copied().max().unwrap_or(10).max(10);

    let mem_sparkline = Sparkline::default()
        .block(Block::default())
        .data(&values_mem)
        .max(max_vals)
        .style(Style::default().fg(Color::Green));

    f.render_widget(top_right_block.clone(), top_cols[1]);
    f.render_widget(mem_sparkline, top_right_block.inner(top_cols[1]));

    // Network I/O with current values
    let net_rx = _app.analytics.net_rx;
    let net_tx = _app.analytics.net_tx;
    let net_title = format!(
        "Network I/O - ↑{} ↓{}",
        format_bytes(net_tx),
        format_bytes(net_rx)
    );
    let bottom_left_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title(net_title);

    let network_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bottom_left_block.inner(bottom_cols[0]));

    let values_tx = sparkline_net_tx_window(_app, network_row[0].width as usize);
    let max_tx = values_tx.iter().copied().max().unwrap_or(1024).max(1024);

    let values_rx = sparkline_net_rx_window(_app, network_row[1].width as usize);
    let max_rx = values_rx.iter().copied().max().unwrap_or(1024).max(1024);

    // Get current rate for display
    let current_tx = _app.net_data.last().map(|n| n.net_tx).unwrap_or(0);
    let current_rx = _app.net_data.last().map(|n| n.net_rx).unwrap_or(0);
    let tx_title = format!("Upload - {}/s", format_bytes(current_tx * 100)); // *100 to reverse scaling
    let rx_title = format!("Download - {}/s", format_bytes(current_rx * 100));

    let upload_sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(tx_title))
        .data(&values_tx)
        .max(max_tx)
        .style(Style::default().fg(Color::Cyan));
    let download_sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(rx_title))
        .data(&values_rx)
        .max(max_rx)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(bottom_left_block.clone(), bottom_cols[0]);
    f.render_widget(upload_sparkline, network_row[0]);
    f.render_widget(download_sparkline, network_row[1]);

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
