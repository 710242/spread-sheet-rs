use crate::ui::ui;
use crossterm::event::KeyCode;
use std::sync;
use tui::{backend::Backend, widgets::TableState, Terminal};

enum WndsEvent<I> {
    Input(I),
    Tick,
}

enum HomePage {
    CreateSheet,
    SelectSheet,
    FunctionRow,
    Help,
}

enum EditingPage {
    None,
    SelectRow,
    StartInput,
}

enum InputMode {
    Normal,
    Editing,
}

pub struct App<'a> {
    table_state: TableState,
    table_items: Vec<Vec<&'a str>>,
    home_page: HomePage,
    editing_page: EditingPage,
    input: String,
    input_mode: InputMode,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            table_state: TableState::default(),
            table_items: vec![vec!["r11", "r12", "r13"], vec!["r21", "r22", "r23"]],
            home_page: HomePage::Help,
            editing_page: EditingPage::None,
            input: String::new(),
            input_mode: InputMode::Normal,
        }
    }
}

pub fn run_app<B: Backend, R>(
    terminal: &mut Terminal<B>,
    mut app: App,
    rx: sync::mpsc::Receiver<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| {
            ui(f);
        })?;

        match rx.recv()? {
            WndsEvent::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                _ => {}
            },
            WndsEvent::Tick => {}
        }
    }
}
