use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    layout::{Layout, Constraint, Direction}
};

use crate::detail_event::DetailEvent; 

pub struct App {
    pub events: Vec<DetailEvent>,
    pub scroll_state: ListState
}

impl App {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            scroll_state: ListState::default()
        }
    }

    pub fn scroll_down(&mut self) {
        let i = match self.scroll_state.selected() {
            Some(i) => if i >= self.events.len() -1 {0} else {i + 1},
            None => 0    
        };
        self.scroll_state.select(Some(i));
    }

    pub fn scroll_up(&mut self) {
        let i = match self.scroll_state.selected() {
            Some(i) => if i == 0 { self.events.len() -1 } else {i - 1},
            None => 0    
        };
        self.scroll_state.select(Some(i));
    }
}

pub fn ui(frame: &mut Frame, app: &mut App) {

    let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ]).split(frame.size());


    let items: Vec<ListItem> = app
        .events
        .iter()
        .map(|event| {
            let color = event.event_type.get_color();
            let style = Style::default().bg(color);  
            ListItem::new(format!("{:?}", event)).style(style)
        })
        .collect();

    let event_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Netwatch Events"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_stateful_widget(event_list, chunks[0], &mut app.scroll_state);

    let stats_block = Paragraph::new("Hello")
        .block(Block::default().borders(Borders::ALL).title("Stats"));

    frame.render_widget(stats_block, chunks[1]);    
}