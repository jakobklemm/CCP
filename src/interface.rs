//! Interface

use crate::application::App;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Rect},
    prelude::{Alignment, Frame, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{
        block::title::Title, Block, BorderType, Borders, List, ListItem, ListState, Paragraph,
        Tabs, Wrap,
    },
};
use tui_textarea::TextArea;

pub fn render_header(f: &mut Frame, app: &mut App, area: Rect) {
    let bar = Layout::default()
        .constraints([Constraint::Percentage(96), Constraint::Percentage(4)])
        .direction(Direction::Horizontal)
        .split(area);

    let percision = {
        let width = f.size().width;
        if width > 250 {
            3
        } else if width > 200 {
            2
        } else if width > 150 {
            1
        } else {
            0
        }
    };

    let para = Paragraph::new(format!("{:.*}", percision, app.fps))
        .block(
            Block::default()
                .title(" FPS ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::new().fg(Color::Gray))
        .alignment(Alignment::Center);

    let sections = ["Dashboard", "Search", "Import", "Export", "Execute"]
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
                .title(Title::from(" V0.1.1 ").alignment(Alignment::Right)),
        )
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .style(Style::default().fg(Color::LightRed))
        .divider(" | ")
        .select(app.get_index());

    f.render_widget(tabs, bar[0]);
    f.render_widget(para, bar[1]);
}

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());

    render_header(f, app, layout[0]);

    match &mut app.interface {
        Interface::Dashboard { .. } => render_dashboard(f, app, layout[1]),
        Interface::Search { .. } => render_search(f, app, layout[1]),
        Interface::Tags {} => draw_counter(f, app, layout[1]),
        _ => {}
    }
}

fn render_dashboard(f: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .direction(Direction::Horizontal)
        .split(area);

    render_dashboard_left(f, app, layout[0]);
    render_dashboard_center(f, app, layout[1]);
    render_dashboard_right(f, app, layout[2]);
}

fn render_dashboard_left(f: &mut Frame, app: &mut App, area: Rect) {
    let bars = Layout::default()
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(40),
            Constraint::Percentage(35),
        ])
        .direction(Direction::Vertical)
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let p = Paragraph::new(format!("LEFT TOP")).block(block.clone());
    f.render_widget(p, bars[0]);

    let p = Paragraph::new(format!("LEFT BOTTOM")).block(block.clone());
    f.render_widget(p, bars[2]);
}

fn render_dashboard_center(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let style = Style::default().fg(Color::Yellow);

    let para = Paragraph::new("Some center text stuff \n This is text.")
        .block(block)
        .style(style)
        .alignment(Alignment::Center);

    f.render_widget(para, area);
}

fn render_dashboard_right(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = (1..15)
        .into_iter()
        .map(|x| {
            let t = format!(" - {:?} _ - _", x);
            ListItem::new(t).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(" - Tags -").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">");

    f.render_widget(list, area);
}

fn render_search(f: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    render_search_header(f, app, layout[0]);
    render_search_list(f, app, layout[1]);
}

fn render_search_header(f: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .constraints([Constraint::Percentage(5), Constraint::Min(1)])
        .direction(Direction::Horizontal)
        .split(area);

    let para = Paragraph::new(format!(" Search: "))
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    match &mut app.interface {
        Interface::Search { input, .. } => {
            input.set_cursor_line_style(Style::default());
            input.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

            f.render_widget(para, layout[0]);
            f.render_widget(input.widget(), layout[1]);
        }
        _ => {}
    };
}

use crate::application::Interface;

fn render_search_list(f: &mut Frame, app: &mut App, area: Rect) {
    let mut items = Vec::new();
    let mut state = ListState::default();
    match &mut app.interface {
        Interface::Search { input, list } => {
            items = list.items.clone();
            state = list.state.clone();
        }
        _ => {}
    }

    let updated: Vec<ListItem> = items
        .iter()
        .map(|x| {
            let txt = format!("{} - {} - {} - {:?}", x.key, x.id.0, x.title, x.tags);
            ListItem::new(txt).style(Style::default().fg(Color::Gray))
        })
        .collect();

    let list = List::new(updated)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Red));

    f.render_stateful_widget(list, area, &mut state)
}

fn draw_home(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Min(10)])
        .split(area);

    let p = Paragraph::new(format!(" Home Screen : {:?}", f.size()))
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
        app.home
            .first
            .set_style(Style::default().fg(Color::LightGreen));
        app.home
            .second
            .set_style(Style::default().fg(Color::LightRed));
    } else {
        app.home
            .first
            .set_style(Style::default().fg(Color::LightRed));
        app.home
            .second
            .set_style(Style::default().fg(Color::LightGreen));
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
