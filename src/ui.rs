use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
};

use crate::{
    app::{App, Tab},
    log::log::{Log, LogList},
};

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_tabs(f, chunks[0], app);
    draw_content(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
}

fn draw_tabs(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let titles = vec!["Containers", "Images", "Deployments", "Logs", "Settings"];

    let selected = match app.current_tab {
        Tab::Containers => 0,
        Tab::Images => 1,
        Tab::Deployments => 2,
        Tab::Logs => 3,
        Tab::Settings => 4,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("foreci"))
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
        for (idx, container) in app.containers.iter().enumerate() {
            let status_icon = match container.status.as_str() {
                "running" => "running",
                "success" => "success",
                "failed" => "failed",
                _ => "wait",
            };
            let display = format!(
                "{} {} [{}]",
                status_icon, container.name, container.dockerfile
            );
            items.push(ListItem::new(display));
            if app.expanded_index == Some(idx) {
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
        .highlight_symbol("→ ");

    f.render_stateful_widget(list, area, &mut app.container_state);
}

fn draw_content(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    let title = match app.current_tab {
        Tab::Containers => "Containers",
        Tab::Images => "Images",
        Tab::Deployments => "Deployments",
        Tab::Logs => "Logs",
        Tab::Settings => "Settings",
    };

    let block = Block::default().borders(Borders::ALL).title(title);

    f.render_widget(block.clone(), area);

    let inner = block.inner(area);

    match app.current_tab {
        Tab::Containers => draw_containers(f, inner, app),
        Tab::Images => {
            //
        }
        Tab::Deployments => {
            //
        }
        Tab::Logs => {
            //
        }
        Tab::Settings => {
            //
        }
    }
}

fn draw_footer(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let mut items: Vec<ListItem> = Vec::new();

    let footer = Paragraph::new(
        "q: Quit | r: Run Job | Tab: Next | Shift+Tab: Prev | ↕ or k/j : Select List",
    )
    .block(Block::default().borders(Borders::ALL));

    let terus = app.log.to_display_string();
    let footer1 = Paragraph::new(terus).block(Block::default().borders(Borders::ALL).title("Logs"));

    let row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    f.render_widget(footer, row[0]);
    f.render_widget(footer1, row[1]);
}
