use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![];

    lines.push(Line::from(vec![
        Span::styled("Trunk: ", Style::default().fg(Color::Gray)),
        Span::styled(&app.trunk, Style::default().fg(Color::Cyan)),
    ]));
    lines.push(Line::from(""));

    for (stack_idx, stack) in app.stacks.iter().enumerate() {
        if stack_idx > 0 {
            lines.push(Line::from(""));
        }

        for (branch_idx, entry) in stack.iter().enumerate() {
            let is_selected =
                stack_idx == app.selected_stack && branch_idx == app.selected_branch;

            let mut spans = vec![];

            for _ in 0..entry.depth {
                spans.push(Span::styled("│ ", Style::default().fg(Color::DarkGray)));
            }

            let is_last = branch_idx == stack.len() - 1;
            let connector = if entry.depth == 0 {
                if stack.len() == 1 {
                    "─ "
                } else if is_last {
                    "└─"
                } else {
                    "├─"
                }
            } else if is_last {
                "└─"
            } else {
                "├─"
            };
            spans.push(Span::styled(connector, Style::default().fg(Color::DarkGray)));

            let marker = if entry.is_current { "◉ " } else { "○ " };
            let marker_style = if entry.is_current {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            spans.push(Span::styled(marker, marker_style));

            let name_style = if is_selected {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else if entry.is_current {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            spans.push(Span::styled(&entry.name, name_style));

            if let Some(mr_num) = entry.mr_number {
                spans.push(Span::styled(
                    format!(" #{} ", mr_num),
                    Style::default().fg(Color::Cyan),
                ));

                if let Some(mr) = app.mr_details.get(&mr_num) {
                    let state_str = format!("{}", mr.state);
                    let state_color = match mr.state {
                        stkd_provider_api::MergeRequestState::Open => Color::Green,
                        stkd_provider_api::MergeRequestState::Merged => Color::Magenta,
                        stkd_provider_api::MergeRequestState::Closed => Color::Red,
                        stkd_provider_api::MergeRequestState::Draft => Color::Yellow,
                    };
                    spans.push(Span::styled(
                        format!("[{}]", state_str),
                        Style::default().fg(state_color),
                    ));
                }
            }

            if entry.is_current {
                spans.push(Span::styled(
                    " [active]",
                    Style::default().fg(Color::Green),
                ));
            }
            if is_selected && !entry.is_current {
                spans.push(Span::styled(
                    " [selected]",
                    Style::default().fg(Color::Blue),
                ));
            }

            lines.push(Line::from(spans));
        }
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Stacks ")
                .title_alignment(Alignment::Left),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
