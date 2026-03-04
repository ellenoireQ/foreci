// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Sparkline, Table},
};

use crate::app::{App, Tab};

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
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(f.area());

    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(36),
            Constraint::Fill(1),
        ])
        .split(root[0]);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Fill(1),
        ])
        .split(content[0]);

    draw_left_containers(f, left[0], app);
    draw_left_images(f, left[1], app);
    draw_left_running(f, left[2], app);
    draw_right_panel(f, content[1], app);
    draw_logs(f, root[1], app);
    draw_keybindings(f, root[2], app);

    if app.env_editor_open {
        draw_env_editor(f, app);
    }
}

fn active_border(is_active: bool) -> Style {
    if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn draw_left_containers(f: &mut Frame, area: Rect, app: &mut App) {
    let is_active = app.current_tab == Tab::Containers;
    let border_style = active_border(is_active);

    let mut items: Vec<ListItem> = Vec::new();

    if app.loading && is_active {
        items.push(ListItem::new("⏳ Loading..."));
    } else if app.containers.is_empty() {
        items.push(
            ListItem::new("(empty)")
                .style(Style::default().fg(Color::DarkGray)),
        );
    } else {
        for (idx, container) in app.containers.iter().enumerate() {
            let display = format!("🖿 {}", container.name);
            items.push(
                ListItem::new(display).style(
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
            );
            if app.expanded_index == Some(idx) {
                let menu_items = [
                    "  Build & Start",
                    "  Start",
                    "  Stop",
                    "  Delete Container",
                ];
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
        .highlight_symbol("→ ")
        .block(
            Block::default()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::ALL)
                .title("Containers")
                .border_style(border_style),
        );

    if is_active {
        f.render_stateful_widget(list, area, &mut app.container_state);
    } else {
        f.render_widget(list, area);
    }
}

fn draw_left_images(f: &mut Frame, area: Rect, app: &mut App) {
    let is_active = app.current_tab == Tab::Images;
    let border_style = active_border(is_active);

    let mut items: Vec<ListItem> = Vec::new();

    if app.loading && is_active {
        items.push(ListItem::new("⏳ Loading images..."));
    } else if app.images.is_empty() {
        items.push(
            ListItem::new("(empty)")
                .style(Style::default().fg(Color::DarkGray)),
        );
    } else {
        for (idx, image) in app.images.iter().enumerate() {
            let display = format!("🐳 {}:{}", image.repository, image.tag);
            items.push(
                ListItem::new(display).style(
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
            );
            if app.image_expanded_index == Some(idx) {
                let menu_items = ["  Delete"];
                for (menu_idx, menu_item) in menu_items.iter().enumerate() {
                    let style = if menu_idx == app.image_menu_selection {
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
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
        .highlight_symbol("→ ")
        .block(
            Block::default()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::ALL)
                .title("Images")
                .border_style(border_style),
        );

    if is_active {
        f.render_stateful_widget(list, area, &mut app.image_state);
    } else {
        f.render_widget(list, area);
    }
}

fn draw_left_running(f: &mut Frame, area: Rect, app: &mut App) {
    let is_active = app.current_tab == Tab::Deployments;
    let border_style = active_border(is_active);

    let mut items: Vec<ListItem> = Vec::new();

    if app.loading && is_active {
        items.push(ListItem::new("⏳ Loading..."));
    } else if app.running_containers.is_empty() {
        items.push(
            ListItem::new("(none running)")
                .style(Style::default().fg(Color::DarkGray)),
        );
    } else {
        for container in &app.running_containers {
            let name = container
                .names
                .first()
                .map(|n| n.trim_start_matches('/').to_string())
                .unwrap_or_else(|| container.id.clone());
            let is_sel = app.selected_container_id.as_ref() == Some(&container.id);
            let prefix = if is_sel { "● " } else { "  " };
            let style = if is_sel {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan)
            };
            items.push(ListItem::new(format!("{}{}", prefix, name)).style(style));
        }
    }

    let list = List::new(items)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("→ ")
        .block(
            Block::default()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::ALL)
                .title("Running")
                .border_style(border_style),
        );

    if is_active {
        f.render_stateful_widget(list, area, &mut app.running_container_state);
    } else {
        f.render_widget(list, area);
    }
}

fn draw_right_panel(f: &mut Frame, area: Rect, app: &mut App) {
    match app.current_tab {
        Tab::Containers => draw_container_detail(f, area, app),
        Tab::Images => draw_image_detail(f, area, app),
        Tab::Deployments => draw_analytics(f, area, app),
    }
}

fn draw_container_detail(f: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Details")
        .border_style(Style::default().fg(Color::Yellow));
    let inner = block.inner(area);
    f.render_widget(block, area);

    if app.loading {
        f.render_widget(Paragraph::new("⏳ Loading..."), inner);
        return;
    }

    if let Some(idx) = app.container_idx {
        if let Some(ctn) = app.containers.get(idx) {
            let rows = vec![
                Row::new(vec![Cell::from("Name"),       Cell::from(":"), Cell::from(ctn.name.clone())]),
                Row::new(vec![Cell::from("Service"),    Cell::from(":"), Cell::from(ctn.service.clone())]),
                Row::new(vec![Cell::from("Container"),  Cell::from(":"), Cell::from(ctn.container_name.clone())]),
                Row::new(vec![Cell::from("Hostname"),   Cell::from(":"), Cell::from(ctn.hostname.clone())]),
                Row::new(vec![Cell::from("Image"),      Cell::from(":"), Cell::from(ctn.image.clone())]),
                Row::new(vec![Cell::from("Port"),       Cell::from(":"), Cell::from(ctn.ports.clone())]),
                Row::new(vec![Cell::from("Build Ctx"),  Cell::from(":"), Cell::from(ctn.build_context.clone())]),
                Row::new(vec![Cell::from("Dockerfile"), Cell::from(":"), Cell::from(ctn.dockerfile.clone())]),
                Row::new(vec![Cell::from("Env"),        Cell::from(":"), Cell::from(ctn.environment.join(", "))]),
                Row::new(vec![Cell::from("Volumes"),    Cell::from(":"), Cell::from(ctn.volumes.join(", "))]),
                Row::new(vec![Cell::from("Networks"),   Cell::from(":"), Cell::from(ctn.networks.join(", "))]),
                Row::new(vec![Cell::from("Restart"),    Cell::from(":"), Cell::from(ctn.restart.clone())]),
            ];
            let table = Table::new(rows, &[
                Constraint::Length(12),
                Constraint::Length(2),
                Constraint::Fill(1),
            ]);
            f.render_widget(table, inner);
            return;
        }
    }

    f.render_widget(
        Paragraph::new("Select a container on the left.")
            .style(Style::default().fg(Color::DarkGray)),
        inner,
    );
}

fn draw_image_detail(f: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Image Details")
        .border_style(Style::default().fg(Color::Yellow));
    let inner = block.inner(area);
    f.render_widget(block, area);

    if app.loading {
        f.render_widget(Paragraph::new("⏳ Loading..."), inner);
        return;
    }

    if let Some(idx) = app.image_idx {
        if let Some(image) = app.images.get(idx) {
            let rows = vec![
                Row::new(vec![Cell::from("Repository"), Cell::from(":"), Cell::from(image.repository.clone())]),
                Row::new(vec![Cell::from("Tag"),        Cell::from(":"), Cell::from(image.tag.clone())]),
                Row::new(vec![Cell::from("Image ID"),   Cell::from(":"), Cell::from(image.image_id.clone())]),
                Row::new(vec![Cell::from("Created"),    Cell::from(":"), Cell::from(image.created.clone())]),
                Row::new(vec![Cell::from("Size"),       Cell::from(":"), Cell::from(image.size.clone())]),
            ];
            let table = Table::new(rows, &[
                Constraint::Length(12),
                Constraint::Length(2),
                Constraint::Fill(1),
            ]);
            f.render_widget(table, inner);
            return;
        }
    }

    f.render_widget(
        Paragraph::new("Select an image on the left.")
            .style(Style::default().fg(Color::DarkGray)),
        inner,
    );
}

fn sparkline_window(app: &mut App, width: usize) -> Vec<u64> {
    let len = app.cpu_data.len();
    if len == 0 { return vec![]; }
    let start = len.saturating_sub(width);
    app.cpu_data_as_slice()[start..len].to_vec()
}

fn sparkline_mem_window(app: &mut App, width: usize) -> Vec<u64> {
    let len = app.mem_data.len();
    if len == 0 { return vec![]; }
    let start = len.saturating_sub(width);
    app.mem_data_as_slice()[start..len].to_vec()
}

fn sparkline_net_rx_window(app: &mut App, width: usize) -> Vec<u64> {
    let len = app.net_data.len();
    if len == 0 { return vec![]; }
    let start = len.saturating_sub(width);
    app.net_data_as_slice()[start..len].iter().map(|n| n.net_rx).collect()
}

fn sparkline_net_tx_window(app: &mut App, width: usize) -> Vec<u64> {
    let len = app.net_data.len();
    if len == 0 { return vec![]; }
    let start = len.saturating_sub(width);
    app.net_data_as_slice()[start..len].iter().map(|n| n.net_tx).collect()
}

fn draw_analytics(f: &mut Frame, area: Rect, app: &mut App) {
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

    // CPU
    let cpu_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title(format!("CPU Usage - {:.2}%", app.analytics.cpu_percent));
    app.update_cpu_scroll();
    let cpu_vals = sparkline_window(app, cpu_block.inner(top_cols[0]).width as usize);
    let cpu_max = cpu_vals.iter().copied().max().unwrap_or(10).max(10);
    let cpu_sparkline = Sparkline::default()
        .data(&cpu_vals).max(cpu_max)
        .style(Style::default().fg(Color::Green));
    f.render_widget(cpu_block.clone(), top_cols[0]);
    f.render_widget(cpu_sparkline, cpu_block.inner(top_cols[0]));

    // Memory
    let mem_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title(format!("Memory Usage - {:.2}%", app.analytics.mem_percent));
    let mem_vals = sparkline_mem_window(app, mem_block.inner(top_cols[1]).width as usize);
    let mem_max = mem_vals.iter().copied().max().unwrap_or(10).max(10);
    let mem_sparkline = Sparkline::default()
        .data(&mem_vals).max(mem_max)
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(mem_block.clone(), top_cols[1]);
    f.render_widget(mem_sparkline, mem_block.inner(top_cols[1]));

    // Network I/O
    let net_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title(format!(
            "Network I/O - ↑{} ↓{}",
            format_bytes(app.analytics.net_tx),
            format_bytes(app.analytics.net_rx),
        ));
    let net_inner = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(net_block.inner(bottom_cols[0]));

    let vtx = sparkline_net_tx_window(app, net_inner[0].width as usize);
    let vrx = sparkline_net_rx_window(app, net_inner[1].width as usize);
    let ctx = app.net_data_as_slice().last().map(|n| n.net_tx).unwrap_or(0);
    let crx = app.net_data_as_slice().last().map(|n| n.net_rx).unwrap_or(0);

    let upload = Sparkline::default()
        .block(Block::default().borders(Borders::ALL)
            .title(format!("↑ {}/s", format_bytes(ctx * 100))))
        .data(&vtx).max(vtx.iter().copied().max().unwrap_or(1024).max(1024))
        .style(Style::default().fg(Color::Cyan));
    let download = Sparkline::default()
        .block(Block::default().borders(Borders::ALL)
            .title(format!("↓ {}/s", format_bytes(crx * 100))))
        .data(&vrx).max(vrx.iter().copied().max().unwrap_or(1024).max(1024))
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(net_block.clone(), bottom_cols[0]);
    f.render_widget(upload, net_inner[0]);
    f.render_widget(download, net_inner[1]);

    // Disk Usage
    let disk_block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title("Disk Usage");
    let disk_inner = disk_block.inner(bottom_cols[1]);
    let disk_text = format!(
        "Images:      {}\nContainers:  {}\nVolumes:     {}\nBuild Cache: {}\nTotal:       {}",
        format_bytes(app.analytics.disk_images),
        format_bytes(app.analytics.disk_containers),
        format_bytes(app.analytics.disk_volumes),
        format_bytes(app.analytics.disk_build_cache),
        format_bytes(app.analytics.disk_total),
    );
    f.render_widget(disk_block, bottom_cols[1]);
    f.render_widget(
        Paragraph::new(disk_text).style(Style::default().fg(Color::Magenta)),
        disk_inner,
    );
}

fn draw_logs(f: &mut Frame, area: Rect, app: &mut App) {
    let para = Paragraph::new(app.log.to_display_string())
        .block(
            Block::default()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::ALL)
                .title("Logs"),
        )
        .scroll((0, app.log_scroll));
    f.render_widget(para, area);
}

fn draw_keybindings(f: &mut Frame, area: Rect, app: &App) {
    let text = if app.current_tab == Tab::Containers && !app.env_editor_open {
        " q: Quit  r: Refresh  Tab: Switch  ↑↓: Navigate  Enter: Menu  e: Edit Env  Esc: Close"
    } else {
        " q: Quit  r: Refresh  Tab/Shift+Tab: Switch Panel  ↑↓: Navigate  Enter: Menu  Esc: Close"
    };
    f.render_widget(
        Paragraph::new(text)
            .style(Style::default().fg(Color::DarkGray))
            .centered(),
        area,
    );
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_env_editor(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 70, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL)
        .title(" Edit Environment Variables ")
        .border_style(Style::default().fg(Color::Green));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(inner);

    let items: Vec<ListItem> = app
        .env_editor_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let is_sel = i == app.env_editor_selected;
            let is_editing = is_sel && app.env_editor_editing;

            let text = if is_editing {
                format!("{}▌", app.env_editor_buffer)
            } else if line.is_empty() {
                "(empty)".to_string()
            } else {
                line.clone()
            };

            let style = if is_sel {
                if is_editing {
                    Style::default().fg(Color::Black).bg(Color::Green)
                } else {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                }
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(text).style(style)
        })
        .collect();

    f.render_widget(List::new(items), chunks[0]);

    let hint = if app.env_editor_editing {
        " Enter: confirm  Esc: cancel edit"
    } else {
        " Enter: edit  a: add  d/x: delete  s: save & close  Esc: close without saving"
    };
    f.render_widget(
        Paragraph::new(hint).style(Style::default().fg(Color::DarkGray)),
        chunks[1],
    );
}
