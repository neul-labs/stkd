use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Modal};

use super::centered_rect;

pub fn draw(frame: &mut Frame, app: &App) {
    if let Some(ref modal) = app.modal {
        let area = frame.area();
        let popup_area = centered_rect(60, 40, area);

        frame.render_widget(Clear, popup_area);

        match modal {
            Modal::Confirm {
                title,
                message,
                on_confirm: _,
            } => draw_confirm(frame, popup_area, title, message),
            Modal::Input {
                title,
                value,
                on_submit: _,
            } => draw_input(frame, popup_area, title, value),
            Modal::Progress { title, message } => draw_progress(frame, popup_area, title, message),
        }
    }
}

fn draw_confirm(frame: &mut Frame, area: Rect, title: &str, message: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::Black));

    let text = Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(message, Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(vec![
            Span::styled("y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" = yes  ", Style::default().fg(Color::Gray)),
            Span::styled("n", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(" = no  ", Style::default().fg(Color::Gray)),
            Span::styled("esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" = cancel", Style::default().fg(Color::Gray)),
        ]),
    ]);

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn draw_input(frame: &mut Frame, area: Rect, title: &str, value: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::Black));

    let text = Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Enter branch name:",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("> {}", value),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" = confirm  ", Style::default().fg(Color::Gray)),
            Span::styled("Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(" = cancel", Style::default().fg(Color::Gray)),
        ]),
    ]);

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn draw_progress(frame: &mut Frame, area: Rect, title: &str, message: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::Black));

    let text = Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("⠋ {}", message),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Please wait...",
            Style::default().fg(Color::DarkGray),
        )),
    ]);

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
