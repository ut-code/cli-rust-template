use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::app::{App, Mode};

/// Render the UI
pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer/Help
        ])
        .split(frame.area());

    render_header(app, frame, chunks[0]);
    render_main(app, frame, chunks[1]);
    render_footer(app, frame, chunks[2]);

    // Render edit popup if in editing mode
    if app.mode == Mode::Editing {
        render_edit_popup(app, frame);
    }
}

/// Render the header
fn render_header(app: &App, frame: &mut Frame, area: Rect) {
    let mode_indicator = match app.mode {
        Mode::Normal => Span::styled(" NORMAL ", Style::default().fg(Color::Green)),
        Mode::Editing => Span::styled(" EDITING ", Style::default().fg(Color::Yellow)),
    };
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" CLI Template ", Style::default().fg(Color::Cyan).bold()),
        Span::raw("- TUI Mode "),
        mode_indicator,
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    )
    .alignment(Alignment::Center);

    frame.render_widget(header, area);
}

/// Render the main content
fn render_main(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_list(app, frame, chunks[0]);
    render_info(app, frame, chunks[1]);
}

/// Render the list panel
fn render_list(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.list_state_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if i == app.list_state_index {
                "▶ "
            } else {
                "  "
            };
            ListItem::new(Line::from(format!("{}{}", prefix, item))).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Items ")
            .title_style(Style::default().fg(Color::Green)),
    );

    frame.render_widget(list, area);
}

/// Render the info panel
fn render_info(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)])
        .split(area);

    // Counter
    let counter_color = if app.counter >= 0 {
        Color::Green
    } else {
        Color::Red
    };
    let counter = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Counter: {}", app.counter),
            Style::default().fg(counter_color).bold(),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Counter ")
            .title_style(Style::default().fg(Color::Magenta)),
    )
    .alignment(Alignment::Center);

    frame.render_widget(counter, chunks[0]);

    // Logs
    let logs: Vec<Line> = app
        .logs
        .iter()
        .rev()
        .take(10)
        .map(|log| {
            let color = if log.contains("[INFO]") {
                Color::Blue
            } else if log.contains("[WARN]") {
                Color::Yellow
            } else if log.contains("[ERROR]") {
                Color::Red
            } else {
                Color::Gray
            };
            Line::from(Span::styled(log.clone(), Style::default().fg(color)))
        })
        .collect();

    let logs_widget = Paragraph::new(logs).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Logs ")
            .title_style(Style::default().fg(Color::Cyan)),
    );

    frame.render_widget(logs_widget, chunks[1]);
}

/// Render the footer with help text
fn render_footer(app: &App, frame: &mut Frame, area: Rect) {
    let help = match app.mode {
        Mode::Normal => "j/↓: Down | k/↑: Up | Enter: Edit | d: Delete | a: Add | q: Quit",
        Mode::Editing => "Enter: Confirm | Esc: Cancel",
    };

    let footer = Paragraph::new(Line::from(help))
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).title(" Help "))
        .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}

/// Render the edit popup
fn render_edit_popup(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let popup_area = Rect {
        x: area.width / 4,
        y: area.height / 2 - 2,
        width: area.width / 2,
        height: 5,
    };

    frame.render_widget(Clear, popup_area);

    let input = Paragraph::new(format!("{}│", app.edit_buffer))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Edit Item (Enter: Save, Esc: Cancel) "),
        );

    frame.render_widget(input, popup_area);
}
