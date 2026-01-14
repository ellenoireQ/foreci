use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
};

use crate::app::{App, Tab};

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
    draw_footer(f, chunks[2]);
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

fn draw_content(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let title = match app.current_tab {
        Tab::Containers => "Containers",
        Tab::Images => "Images",
        Tab::Deployments => "Deployments",
        Tab::Logs => "Logs",
        Tab::Settings => "Settings",
    };

    let block = Block::default().borders(Borders::ALL).title(title);

    f.render_widget(block, area);
}

fn draw_footer(f: &mut Frame, area: ratatui::layout::Rect) {
    let footer = Paragraph::new("q: Quit | Tab: Next | Shift+Tab: Prev")
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
