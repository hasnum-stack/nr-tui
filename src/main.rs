mod config;
mod tui;
mod utils;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    // symbols::border,
    widgets::{block::*, Widget, *},
};
use std::io::Result;

#[derive(Debug)]
pub struct App {
    exit: bool,
    row_index: usize,
    selected_index: Option<usize>,
    config_list: config::ConfigList::Dynamic,
}

impl App {
    fn default() -> Self {
        let config_list = config::get();
        App {
            exit: false,
            row_index: 0,
            selected_index: None,
            config_list,
        }
    }
    // ANCHOR: run
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    // ANCHOR_END: run

    // ANCHOR: render_frame
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }
    // ANCHOR_END: render_frame

    // ANCHOR: handle_events
    /// updates the application's state based on user input
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    // ANCHOR_END: handle_events

    // ANCHOR: handle_key_event
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                if self.row_index > 0 {
                    self.row_index -= 1;
                    return;
                }
                self.row_index = 2;
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                if self.row_index < 2 {
                    self.row_index += 1;
                    return;
                }
                self.row_index = 0;
            }
            _ => {}
        }
    }
    // ANCHOR_END: handle_key_event

    // ANCHOR: methods
    fn exit(&mut self) {
        self.exit = true;
    }

    // fn increment_counter(&mut self) {
    //     self.counter += 1;
    // }
    //
    // fn decrement_counter(&mut self) {
    //     self.counter -= 1;
    // }
    // ANCHOR_END: methods
}
// ANCHOR_END: methods
// ANCHOR: impl Widget
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // println!("rendering {}", self.counter);
        let title = utils::get_tittle() + self.row_index.to_string().as_str();
        let title = Title::from(title);
        // let instructions = Title::from(Line::from(vec![
        //     " Decrement ".into(),
        //     "<Left>".blue().bold(),
        //     " Increment ".into(),
        //     "<Right>".blue().bold(),
        //     " Quit ".into(),
        //     "<Q> ".blue().bold(),
        // ]));
        // let block = Block::default()
        //     .title(title.alignment(Alignment::Right))
        //     .title(
        //         instructions
        //             .alignment(Alignment::Center)
        //             .position(Position::Bottom),
        //     )
        //     .borders(Borders::ALL)
        //     .border_set(border::THICK);
        //
        // let counter_text = Text::from(vec![Line::from(vec![
        //     "Value: ".into(),
        //     self.counter.to_string().yellow(),
        // ])]);
        let items = self.config_list;
        let mut state = ListState::default().with_selected(Some(self.row_index));

        let list = List::new(items)
            .block(Block::default().title(title).borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        StatefulWidget::render(list, area, buf, &mut state);

        //
        // Paragraph::new(list)
        //     .centered()
        //     .block(block)
        //     .render(area, buf);
    }
}
// ANCHOR_END: impl Widget
fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
