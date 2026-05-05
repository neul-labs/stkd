use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let mut spans = vec![];

    // Repo info
    spans.push(Span::styled(
        format!(" {} ", app.current_branch),
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));

    // Spinner
    if app.spinner_active {
        spans.push(Span::styled(
            " ⠋ ",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ));
    }

    // Status text
    if let Some(ref notif) = app.notification {
        let color = if notif.is_error {
            Color::Red
        } else {
            Color::Green
        };
        spans.push(Span::styled(
            format!(" {} ", notif.message),
            Style::default().fg(color),
        ));
    } else if app.stacks.is_empty() {
        spans.push(Span::styled(
            " No stacks ",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        let total_branches: usize = app.stacks.iter().map(|s| s.len()).sum();
        spans.push(Span::styled(
            format!(" {} stacks, {} branches ", app.stacks.len(), total_branches),
            Style::default().fg(Color::Gray),
        ));
    }

    // Help hint on the right
    spans.push(Span::styled(" ? help ", Style::default().fg(Color::DarkGray)));
    spans.push(Span::styled(" q quit ", Style::default().fg(Color::DarkGray)));

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}
