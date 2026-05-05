use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    if let Some(ref notif) = app.notification {
        let area = frame.area();
        let notif_area = bottom_rect(80, 3, area);

        let color = if notif.is_error {
            Color::Red
        } else {
            Color::Green
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .style(Style::default().bg(Color::Black));

        let icon = if notif.is_error { "✗" } else { "✓" };
        let line = Line::from(vec![
            Span::styled(
                format!(" {} ", icon),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(&notif.message, Style::default().fg(Color::White)),
        ]);

        let paragraph = Paragraph::new(line)
            .block(block)
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, notif_area);
    }
}

fn bottom_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(height),
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
