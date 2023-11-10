//! Interface

use crate::application::App;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Rect, Direction},
    prelude::{Alignment, Frame, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Wrap, block::{title::Title}},
};

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Percentage(20)])
        .split(f.size());

    let sections = ["Home", "Counter"]
        .iter()
        .cloned()
        .map(Line::from)
        .collect();

    let tabs = Tabs::new(sections)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Title::from(" Central Content Processor ").alignment(Alignment::Center))
                .title(Title::from(" TAB SECRET ").alignment(Alignment::Left))
                .title(Title::from(" V0.1.1 ").alignment(Alignment::Right))
        )
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .style(Style::default().fg(Color::LightRed))
        .divider(" - ")
        .select(app.get_index());

    f.render_widget(tabs, layout[0]);

    match app.get_index() {
        0 => draw_home(f, app, layout[1]),
        1 => draw_counter(f, app, layout[1]),
        _ => todo!(),
    }
}

fn draw_home(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Min(10)])
        .split(area);

    let p = Paragraph::new(format!(" Home Screen "))
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(" STUFF ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    let inner = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .direction(Direction::Horizontal)
        .split(chunks[1]);

    if app.home.active % 2 == 0 {
        app.home.first.set_style(Style::default().fg(Color::LightGreen));
        app.home.second.set_style(Style::default().fg(Color::LightRed));
    } else {
        app.home.first.set_style(Style::default().fg(Color::LightRed));
        app.home.second.set_style(Style::default().fg(Color::LightGreen));
    }

    f.render_widget(p, chunks[0]);
    f.render_widget(app.home.first.widget(), inner[0]);
    f.render_widget(app.home.second.widget(), inner[1]);
}

fn draw_counter(f: &mut Frame, app: &mut App, area: Rect) {
    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
            app.get_counter()
        ))
        .block(
            Block::default()
                .title(" TAB Central Content Processor ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        area,
    );
}
