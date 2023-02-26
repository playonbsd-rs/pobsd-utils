use crate::parsing::Game;
use tui::widgets::ListState;

pub(crate) enum InputMode {
    Normal,
    Search,
}

pub(crate) struct AppState {
    pub(crate) mode: InputMode,
    pub(crate) games: Vec<Game>,
    pub(crate) list_state: ListState,
    pub(crate) search_text: String,
    pub(crate) search_list: Vec<Game>,
}

impl AppState {
    pub(crate) fn new() -> AppState {
        AppState {
            mode: InputMode::Normal,
            games: vec![],
            list_state: ListState::default(),
            search_text: String::new(),
            search_list: vec![],
        }
    }
    pub(crate) fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }
    pub(crate) fn search(&mut self) {
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
    pub(crate) fn move_up(&mut self) {
        let len_list = match self.mode {
            InputMode::Search => self.search_list.len(),
            _ => self.games.len(),
        };
        let selected = match self.list_state.selected() {
            Some(v) => {
                if len_list == 0 {
                    None
                } else if v == 0 {
                    Some(v)
                } else {
                    Some(v - 1)
                }
            }
            None => {
                if len_list == 0 {
                    None
                } else {
                    Some(0)
                }
            }
        };
        self.list_state.select(selected);
    }
    pub(crate) fn move_down(&mut self) {
        let len_list = match self.mode {
            InputMode::Search => self.search_list.len(),
            _ => self.games.len(),
        };
        let selected = match self.list_state.selected() {
            Some(v) => {
                if len_list == 0 {
                    None
                } else if v >= len_list - 1 {
                    Some(len_list - 1)
                } else {
                    Some(v + 1)
                }
            }
            None => {
                if len_list == 0 {
                    None
                } else {
                    Some(0)
                }
            }
        };
        self.list_state.select(selected);
    }
}
