use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, event::Event::Key, execute};
use std::path::Path;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};
use tui::{Frame, Terminal};

use crate::parser::{Game, Parser, ParserResult};

enum InputMode {
    Normal,
    Search,
}

struct AppState {
    mode: InputMode,
    games: Vec<Game>,
    list_state: ListState,
    search_text: String,
    search_list: Vec<Game>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            mode: InputMode::Normal,
            games: vec![],
            list_state: ListState::default(),
            search_text: String::new(),
            search_list: vec![],
        }
    }
    pub fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }
    pub fn search(&mut self) {
        self.search_list = self
            .games
            .clone()
            .into_iter()
            .filter(|item| {
                item.name
                    .to_lowercase()
                    .contains(&self.search_text.to_lowercase())
            })
            .collect();
    }
    pub fn move_up(&mut self) {
        let selected = match self.list_state.selected() {
            Some(v) => {
                if v == 0 {
                    Some(v)
                } else {
                    Some(v - 1)
                }
            }
            None => Some(0),
        };
        self.list_state.select(selected);
    }
    pub fn move_down(&mut self) {
        let len_list = match self.mode {
            InputMode::Search => self.search_list.len(),
            _ => self.games.len(),
        };
        let selected = match self.list_state.selected() {
            Some(v) => {
                if v >= len_list - 1 {
                    Some(len_list - 1)
                } else {
                    Some(v + 1)
                }
            }
            None => Some(0),
        };
        self.list_state.select(selected);
    }
}

pub fn browse(db: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let parser = Parser::default();
    let games = match parser.load_from_file(&db)? {
        ParserResult::WithError(games, _) => games,
        ParserResult::WithoutError(games) => games,
    };
    let mut app_state = AppState::new();
    app_state.games = games;
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app_state);

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;

    if let Err(e) = result {
        println!("{}", e.to_string());
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: &mut AppState,
) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| ui(f, state))?;
        if let Key(key) = event::read()? {
            match state.mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('s') => {
                        state.change_mode(InputMode::Search);
                        state.list_state.select(None);
                    }
                    KeyCode::Up | KeyCode::Char('k') => state.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => state.move_down(),
                    _ => {}
                },
                InputMode::Search => match key.code {
                    KeyCode::Esc => {
                        state.change_mode(InputMode::Normal);
                        state.search_text.clear();
                        state.list_state.select(None);
                    }
                    KeyCode::Char(c) => {
                        state.search_text.push(c);
                        state.search();
                    }
                    KeyCode::Backspace => {
                        state.search_text.pop();
                        state.search();
                    }
                    KeyCode::Up => state.move_up(),
                    KeyCode::Down => state.move_down(),
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &mut AppState) {
    let parent_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let list_section_block = Block::default()
        .title("Games")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(list_section_block, parent_chunk[0]);
    list_section(f, state, parent_chunk[0]);

    let detail_section_block = Block::default()
        .title("Details")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(detail_section_block, parent_chunk[1]);
    detail_section(f, state, parent_chunk[1]);
}

fn detail_section<B: Backend>(f: &mut Frame<B>, state: &mut AppState, area: Rect) {
    let game_to_show = state.games[1].to_owned();
    let game_to_show = ListItem::new(Span::from(game_to_show.name));

    let new_selection_chunk = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(4),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area);

    let desc = Paragraph::new("test");
    f.render_widget(desc, new_selection_chunk[3]);
}

fn list_section<B: Backend>(f: &mut Frame<B>, state: &mut AppState, area: Rect) {
    let list_to_show = if state.search_text.is_empty() {
        state.games.to_owned()
    } else {
        state.search_list.to_owned()
    };

    let items: Vec<ListItem> = list_to_show
        .into_iter()
        .map(|item| ListItem::new(Span::from(item.name)))
        .collect();

    let list_chunk = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    let search_input = Paragraph::new(state.search_text.to_owned())
        .block(
            Block::default()
                .title("Search")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(match state.mode {
            InputMode::Search => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });
    f.render_widget(search_input, list_chunk[0]);

    let list = List::new(items)
        .block(Block::default())
        //.highlight_symbol("->")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list, list_chunk[1], &mut state.list_state)
}
