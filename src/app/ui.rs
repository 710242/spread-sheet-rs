use crate::app::{App, HomePage};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Table, Tabs,
        Widget,
    },
    Frame,
};
use tui_logger::TuiLoggerWidget;
use tui_markup::parse;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(f.size());

    let main_page_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);

    // componets
    let function_list = app.header_function.clone();

    // render part

    // header
    match app.home_page {
        HomePage::Editing => {
            let f_list: Vec<_> = function_list
                .iter()
                .map(|funcs| Spans::from(vec![Span::raw(funcs.clone())]))
                .collect();
            let functions = Tabs::new(f_list)
                .block(
                    Block::default()
                        .title("functions")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Plain),
                )
                .select(app.header_index)
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Black)
                        .bg(Color::White),
                );
            f.render_widget(functions, chunks[0]);

            if app.header_popup {
                let title = app.header_function[app.header_index];
                let block = Block::default().title(title).borders(Borders::ALL);
                let area = centered_rect(60, 20, f.size());
                f.render_widget(Clear, area); //this clears out the background
                f.render_widget(block, area);
            }
        }
        HomePage::Welcome => {
            let title = Paragraph::new("Spread-Sheet-inRust")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );
            f.render_widget(title, chunks[0]);
        }
    }

    // main section
    match app.home_page {
        HomePage::Editing => {
            let block = Paragraph::new("test").alignment(Alignment::Center).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default())
                    .border_type(BorderType::Plain)
                    .title("Home Page"),
            );
            // let f_list: Vec<_> = function_list
            //     .iter()
            //     .map(|funcs| ListItem::new(Spans::from(vec![Span::raw(funcs.clone())])))
            //     .collect();
            // let functions = List::new(f_list)
            //     .block(
            //         Block::default()
            //             .title("functions")
            //             .borders(Borders::ALL)
            //             .border_type(BorderType::Plain),
            //     )
            //     .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            //     .highlight_symbol(">>");
            // f.render_stateful_widget(functions, main_page_chunks[0], &mut app.home_f_list_state);
            // f.render_widget(block, main_page_chunks[1]);
            f.render_widget(block, chunks[1]);
        }
        HomePage::Welcome => {
            let wel_message = parse("Welcome to <blue- Spread-Sheet-inRust>\n This is a small project for implementing spread sheet in terminal ui\n\nPlease press e to start(Without capital)\n\nauthor: Jelly Fish");

            let block = Paragraph::new(wel_message.unwrap())
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default())
                        .border_type(BorderType::Plain)
                        .title("Home Page"),
                );
            f.render_widget(block, chunks[1]);
        }
    }

    // tui logger
    let logger = draw_logs();
    f.render_widget(logger, chunks[2]);
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
