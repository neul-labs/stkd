use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::app::{App, Tab};

mod modal;
mod notification;
mod stack_tree;
mod status_bar;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let content_area = chunks[0];
    let status_area = chunks[1];

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(content_area);

    let tabs_area = content_chunks[0];
    let body_area = content_chunks[1];

    draw_tabs(frame, app, tabs_area);
    draw_body(frame, app, body_area);
    status_bar::draw(frame, app, status_area);
    notification::draw(frame, app);
    modal::draw(frame, app);

    if app.show_help {
        draw_help(frame, area, area);
    }
}

fn draw_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = vec!["Stacks", "Status", "Actions"]
        .into_iter()
        .enumerate()
        .map(|(i, t)| {
            let style = if i == app.current_tab as usize {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            Line::from(Span::styled(format!(" {} ", t), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title(" Stack TUI ")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .select(app.current_tab as usize);

    frame.render_widget(tabs, area);
}

fn draw_body(frame: &mut Frame, app: &App, area: Rect) {
    match app.current_tab {
        Tab::Stacks => draw_stacks_tab(frame, app, area),
        Tab::Status => draw_status_tab(frame, app, area),
        Tab::Actions => draw_actions_tab(frame, app, area),
    }
}

fn draw_stacks_tab(frame: &mut Frame, app: &App, area: Rect) {
    if app.stacks.is_empty() {
        let text = Text::from(vec![
            Line::from(""),
            Line::from("No tracked branches found."),
            Line::from(""),
            Line::from(Span::styled(
                "Run 'gt create <branch>' or 'gt track <branch>' to get started.",
                Style::default().fg(Color::DarkGray),
            )),
        ]);
        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
        return;
    }

    stack_tree::draw(frame, app, area);
}

fn draw_status_tab(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Min(0),
        ])
        .split(area);

    let mut branch_lines = vec![
        Line::from(vec![
            Span::styled("Current Branch  ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(&app.current_branch, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
    ];

    if let Some(entry) = app.selected_entry() {
        if let Some(ref parent) = entry.parent {
            branch_lines.push(Line::from(vec![
                Span::styled("Parent         ", Style::default().fg(Color::Gray)),
                Span::raw(parent),
            ]));
        }
        if let Some(mr_num) = entry.mr_number {
            branch_lines.push(Line::from(vec![
                Span::styled("MR             ", Style::default().fg(Color::Gray)),
                Span::styled(format!("#{}", mr_num), Style::default().fg(Color::Cyan)),
            ]));
            if let Some(ref url) = entry.mr_url {
                branch_lines.push(Line::from(vec![
                    Span::styled("URL            ", Style::default().fg(Color::Gray)),
                    Span::styled(url, Style::default().fg(Color::Blue)),
                ]));
            }
        }
    }

    let branch_block = Block::default()
        .borders(Borders::ALL)
        .title(" Branch ");
    let branch_para = Paragraph::new(Text::from(branch_lines)).block(branch_block);
    frame.render_widget(branch_para, chunks[0]);

    let mut mr_lines = vec![];
    if let Some(entry) = app.selected_entry() {
        if let Some(mr_num) = entry.mr_number {
            if let Some(mr) = app.mr_details.get(&mr_num) {
                mr_lines.push(Line::from(vec![
                    Span::styled("State          ", Style::default().fg(Color::Gray)),
                    Span::styled(format!("{}", mr.state), state_color(mr.state)),
                ]));
                if let Some(mergeable) = mr.mergeable {
                    let color = if mergeable { Color::Green } else { Color::Red };
                    let text = if mergeable { "Yes" } else { "No" };
                    mr_lines.push(Line::from(vec![
                        Span::styled("Mergeable      ", Style::default().fg(Color::Gray)),
                        Span::styled(text, color),
                    ]));
                }
                if !mr.labels.is_empty() {
                    mr_lines.push(Line::from(vec![
                        Span::styled("Labels         ", Style::default().fg(Color::Gray)),
                        Span::raw(mr.labels.join(", ")),
                    ]));
                }
            } else {
                mr_lines.push(Line::from(vec![
                    Span::styled("Status         ", Style::default().fg(Color::Gray)),
                    if app.spinner_active {
                        Span::styled("Fetching...", Style::default().fg(Color::Yellow))
                    } else {
                        Span::styled("Press 'g' to fetch from provider", Style::default().fg(Color::DarkGray))
                    },
                ]));
            }
        } else {
            mr_lines.push(Line::from("No merge request created for this branch."));
            mr_lines.push(Line::from(Span::styled(
                "Press 's' to submit and create an MR.",
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let mr_block = Block::default()
        .borders(Borders::ALL)
        .title(" Merge Request ");
    let mr_para = Paragraph::new(Text::from(mr_lines)).block(mr_block);
    frame.render_widget(mr_para, chunks[1]);

    let mut pos_lines = vec![];
    if let Some(stack) = app.stacks.get(app.selected_stack) {
        if stack.len() > 1 {
            pos_lines.push(Line::from(vec![
                Span::styled("Position       ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{} of {}", app.selected_branch + 1, stack.len())),
            ]));
            if app.selected_branch > 0 {
                if let Some(above) = stack.get(app.selected_branch - 1) {
                    pos_lines.push(Line::from(vec![
                        Span::styled("Above          ", Style::default().fg(Color::Gray)),
                        Span::raw(&above.name),
                    ]));
                }
            }
            if app.selected_branch + 1 < stack.len() {
                if let Some(below) = stack.get(app.selected_branch + 1) {
                    pos_lines.push(Line::from(vec![
                        Span::styled("Below          ", Style::default().fg(Color::Gray)),
                        Span::raw(&below.name),
                    ]));
                }
            }
        } else {
            pos_lines.push(Line::from("Single branch stack."));
        }
    }

    let pos_block = Block::default()
        .borders(Borders::ALL)
        .title(" Stack Position ");
    let pos_para = Paragraph::new(Text::from(pos_lines)).block(pos_block);
    frame.render_widget(pos_para, chunks[2]);

    let mut wt_lines = vec![];
    if app.working_tree.clean {
        wt_lines.push(Line::from(Span::styled(
            "Working tree is clean",
            Style::default().fg(Color::Green),
        )));
    } else {
        if app.working_tree.staged > 0 {
            wt_lines.push(Line::from(vec![
                Span::styled("Staged         ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{} file(s)", app.working_tree.staged),
                    Style::default().fg(Color::Green),
                ),
            ]));
        }
        if app.working_tree.modified > 0 {
            wt_lines.push(Line::from(vec![
                Span::styled("Modified       ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{} file(s)", app.working_tree.modified),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
        }
        if app.working_tree.untracked > 0 {
            wt_lines.push(Line::from(vec![
                Span::styled("Untracked      ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{} file(s)", app.working_tree.untracked),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    }

    let wt_block = Block::default()
        .borders(Borders::ALL)
        .title(" Working Tree ");
    let wt_para = Paragraph::new(Text::from(wt_lines)).block(wt_block);
    frame.render_widget(wt_para, chunks[3]);
}

fn draw_actions_tab(frame: &mut Frame, _app: &App, area: Rect) {
    let actions = vec![
        ("c", "Create branch", "Create a new stacked branch"),
        ("d", "Delete branch", "Delete the selected branch"),
        ("s", "Submit", "Push and create/update MRs"),
        ("y", "Sync", "Fetch, delete merged, restack"),
        ("l", "Land", "Merge the current branch/stack"),
        ("r", "Restack", "Rebase branches onto parents"),
        ("u", "Up", "Move up one branch in stack"),
        ("n", "Down", "Move down one branch in stack"),
        ("t", "Top", "Jump to stack tip"),
        ("b", "Bottom", "Jump to stack root"),
        ("Enter", "Checkout", "Checkout selected branch"),
        ("g", "Refresh", "Fetch MR status from provider"),
        ("q", "Quit", "Exit the TUI"),
        ("?", "Help", "Show this help overlay"),
    ];

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            " Keyboard Shortcuts ",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Line::from(""),
    ];

    for (key, action, desc) in actions {
        lines.push(Line::from(vec![
            Span::styled(format!(" {:<10} ", key), Style::default().fg(Color::Yellow)),
            Span::styled(format!("{:<16}", action), Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(desc, Style::default().fg(Color::Gray)),
        ]));
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title(" Actions "));

    frame.render_widget(paragraph, area);
}

fn draw_help(frame: &mut Frame, _area: Rect, area: Rect) {
    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            " Stack TUI Help ",
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  j/↓  Move down    k/↑  Move up    h/←  Prev stack    l/→  Next stack"),
        Line::from("  Tab  Next tab     Shift+Tab  Prev tab"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  Enter  Checkout branch    c  Create branch    d  Delete branch"),
        Line::from("  s      Submit MRs        y  Sync              r  Restack"),
        Line::from("  u      Move up           n  Move down         t  Go to top"),
        Line::from("  b      Go to bottom      g  Refresh status    l  Land (merge)"),
        Line::from(""),
        Line::from("Global:"),
        Line::from("  q      Quit              ?  Toggle help       Ctrl+C  Force quit"),
        Line::from(""),
        Line::from(Span::styled(
            " Press ? or Esc to close ",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let popup_area = centered_rect(70, 80, area);
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Help ")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(Text::from(help_text))
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, popup_area);
}

fn state_color(state: stkd_provider_api::MergeRequestState) -> Style {
    match state {
        stkd_provider_api::MergeRequestState::Open => Style::default().fg(Color::Green),
        stkd_provider_api::MergeRequestState::Merged => Style::default().fg(Color::Magenta),
        stkd_provider_api::MergeRequestState::Closed => Style::default().fg(Color::Red),
        stkd_provider_api::MergeRequestState::Draft => Style::default().fg(Color::Yellow),
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
