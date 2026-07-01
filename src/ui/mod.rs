use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{ActivePanel, App, AppMode};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Min(1)])
        .split(main_chunks[0]);

    draw_folders(frame, app, body_chunks[0]);
    draw_notes(frame, app, body_chunks[1]);
    draw_status_bar(frame, app, main_chunks[1]);
}

fn draw_folders(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = app
        .folders
        .iter()
        .enumerate()
        .map(|(index, folder)| {
            let marker = if index == app.selected_folder {
                "> "
            } else {
                "  "
            };

            let style = if index == app.selected_folder {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            ListItem::new(Line::from(vec![
                Span::styled(marker, style),
                Span::styled(folder, style),
            ]))
        })
        .collect();

    let border_style = if app.active_panel == ActivePanel::Folders {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let list = List::new(items).block(
        Block::default()
            .title(" Folders ")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    frame.render_widget(list, area);
}

fn draw_notes(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = app
        .notes
        .iter()
        .enumerate()
        .map(|(index, note)| {
            let marker = if index == app.selected_note {
                "> "
            } else {
                "  "
            };

            let style = if index == app.selected_note {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            ListItem::new(Line::from(vec![
                Span::styled(marker, style),
                Span::styled(note, style),
            ]))
        })
        .collect();

    let border_style = if app.active_panel == ActivePanel::Notes {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let list = List::new(items).block(
        Block::default()
            .title(" Notes ")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    frame.render_widget(list, area);
}

fn draw_status_bar(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let text = match app.mode {
        AppMode::Normal => {
            format!(
                "q quit | h/l switch | j/k move | enter open | n new | / search | {}",
                app.status_message
            )
        }
        AppMode::CreatingNote => {
            format!("New note: {}", app.new_note_name)
        }
        AppMode::Searching => {
            format!("Search: {}", app.search_query)
        }
    };

    let help = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    frame.render_widget(help, area);
}
