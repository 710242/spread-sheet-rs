mod ui;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::*;
use serde::{Deserialize, Serialize};
use std::sync;
use tui::{
    backend::Backend,
    widgets::{ListState, TableState},
    Terminal,
};
use ui::ui;

const DB_PATH: &str = "./data/";

pub enum WndsEvent {
    Input(KeyEvent),
    Tick,
}

enum CurWnd {
    Functions,
    Popup,
    Home,
}

enum HomePage {
    Welcome,
    Editing,
}

enum InputMode {
    Normal,
    Editing,
}

// #[derive(Serialize, Deserialize)]
pub struct App<'a> {
    cur_windows: CurWnd,
    table_state: TableState,
    table_items: Vec<Vec<&'a str>>,
    home_page: HomePage,
    header_function: Vec<&'a str>,
    header_index: usize,
    home_f_list_state: ListState,
    input: String,
    input_mode: InputMode,
    header_popup: bool,
    start_welcome: bool,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            cur_windows: CurWnd::Functions,
            table_state: TableState::default(),
            table_items: vec![vec!["r11", "r12", "r13"], vec!["r21", "r22", "r23"]],
            home_page: HomePage::Welcome,
            header_function: vec!["new", "open", "help"],
            header_index: 0,
            home_f_list_state: ListState::default(),
            input: String::new(),
            input_mode: InputMode::Normal,
            header_popup: false,
            start_welcome: true,
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    rx: sync::mpsc::Receiver<WndsEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    app.home_f_list_state.select(Some(0));
    loop {
        terminal.draw(|f| {
            ui(f, &mut app);
        })?;

        match rx.recv()? {
            WndsEvent::Input(kevent) => match kevent {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: _,
                } => {
                    return Ok(());
                }
                KeyEvent {
                    code: KeyCode::Char('E'),
                    modifiers: KeyModifiers::SHIFT,
                } => {
                    info!("Shift-e");
                }
                KeyEvent {
                    code: KeyCode::Char('e'),
                    modifiers: KeyModifiers::CONTROL,
                } => {
                    info!("Ctrl-e");
                }
                KeyEvent {
                    code: KeyCode::Char('e'),
                    modifiers: _,
                } => {
                    app.home_page = HomePage::Editing;
                    info!("Press-e");
                }
                KeyEvent {
                    code: KeyCode::Tab,
                    modifiers: _,
                } => match app.cur_windows {
                    CurWnd::Functions => {
                        app.header_index = (app.header_index + 1) % (app.header_function.len());
                    }
                    _ => {}
                },
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: _,
                } => {
                    app.cur_windows = match app.cur_windows {
                        CurWnd::Functions => {
                            app.header_popup = !app.header_popup;
                            CurWnd::Popup
                        }
                        CurWnd::Popup => {
                            app.header_popup = !app.header_popup;
                            CurWnd::Functions
                        }
                        CurWnd::Home => CurWnd::Home,
                    };
                }
                // KeyEvent {
                //     code: KeyCode::Up,
                //     modifiers: _,
                // } => {
                //     let i = match app.home_f_list_state.selected() {
                //         Some(i) => {
                //             if i < app.home_function.len() - 1 {
                //                 0
                //             } else {
                //                 i - 1
                //             }
                //         }
                //         None => 0,
                //     };
                //     app.home_f_list_state.select(Some(i));
                //     info!("Press Up");
                // }
                // KeyEvent {
                //     code: KeyCode::Down,
                //     modifiers: _,
                // } => {
                //     let i = match app.home_f_list_state.selected() {
                //         Some(i) => {
                //             if i >= app.home_function.len() - 1 {
                //                 0
                //             } else {
                //                 i + 1
                //             }
                //         }
                //         None => 0,
                //     };
                //     app.home_f_list_state.select(Some(i));
                //     info!("Press Down");
                // }
                _ => {}
            },
            WndsEvent::Tick => {}
        }
    }
}
