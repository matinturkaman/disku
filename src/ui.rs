use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Span;

use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Row, Table, TableState};
use xerxes::Config;

pub fn render(frame: &mut Frame, config: &Config, file_list_state: &mut TableState) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(2)]).split(frame.area());

    let size_block_title = format!(
        "Total size: {}",
        bytes_to_readable_format(config.total_size),
    );
    let size_block = Block::default().title(size_block_title);
    frame.render_widget(size_block, layout[0]);

    draw_files(frame, config, layout[1], file_list_state);
}

fn draw_files(frame: &mut Frame, config: &Config, area: Rect, table_state: &mut TableState) {
    let header = Row::new(["Created", "Format", "Size", "Name", "Path"]);

    let mut rows = vec![];

    let mut sorted_files = Vec::from_iter(config.files.values());
    sorted_files.sort_by(|a, b| b.size.cmp(&a.size));

    for item in sorted_files {
        rows.push(Row::new(vec![
            Span::styled(
                format!("{:<15}", item.created),
                Style::default().fg(Color::Gray),
            ),
            Span::styled(
                format!("{:<15}", item.format),
                Style::default().fg(Color::Magenta),
            ),
            Span::styled(
                format!("{:<15}", bytes_to_readable_format(item.size)),
                Style::default().fg(Color::Blue),
            ),
            Span::styled(
                format!("{:<35}", truncate_long_string(&item.file_name)),
                Style::default().fg(if item.is_hidden {
                    Color::Rgb(147, 151, 153)
                } else {
                    Color::LightGreen
                }),
            ),
            Span::styled(
                format!("{:<35}", item.path),
                Style::default().fg(if item.is_hidden {
                    Color::Rgb(147, 151, 153)
                } else {
                    Color::Yellow
                }),
            ),
        ]));
    }

    let block = Block::bordered().border_style(Style::default().dark_gray());
    let widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Fill(1),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(Style::new().bg(Color::DarkGray))
        .block(block);

    frame.render_stateful_widget(table, area, table_state);
}

fn bytes_to_readable_format(bytes: u64) -> String {
    if bytes >= 1073741824 {
        format!("{:.2} GB", bytes as f64 / 1073741824.0)
    } else if bytes >= 1048576 {
        format!("{:.2} MB", bytes as f64 / 1048576.0)
    } else if bytes >= 1024 {
        format!("{:.2} kB", bytes as f64 / 1024.0)
    } else {
        format!("{} bytes", bytes)
    }
}

fn truncate_long_string(s: &String) -> String {
    if s.len() > 30 {
        format!("{:.30}...", s)
    } else {
        format!("{}", s)
    }
}
