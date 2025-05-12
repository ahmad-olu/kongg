use std::io;

use crossterm::event::{KeyCode, KeyModifiers};
use kongg_shared::{
    helpers::{crud::get_file, surreal_init::init},
    models::file::FileResponse,
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        Block, Borders, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState,
    },
};

const ITEM_HEIGHT: usize = 4;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let db = init().await.unwrap();
    let files = get_file(&db).await;

    let mut app = App {
        exit: false,
        scroll_state: ScrollbarState::new((files.len() - 1) * ITEM_HEIGHT),
        items: files,
        state: TableState::default().with_selected(0),
        search_text: String::new(),
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

pub struct App {
    state: TableState,
    exit: bool,
    items: Vec<FileResponse>,
    scroll_state: ScrollbarState,
    search_text: String,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
            terminal.draw(|frame| self.draw(frame))?;
            // self.handle_events()?;
        }

        Ok(())
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    fn draw(&mut self, frame: &mut Frame) {
        //frame.render_widget(self, frame.area());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(5),
                Constraint::Percentage(80),
                Constraint::Percentage(5),
            ])
            .split(frame.area());

        self.render_filter_field(frame, layout[0]);
        self.render_input_search_field(frame, layout[1]);
        self.render_table(frame, layout[2]);
        self.render_scrollbar(frame, layout[2]);
        self.render_title(frame, layout[3]);
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        let ke_kind = key_event.kind;
        let ke_code = key_event.code;
        let ke_mod = key_event.modifiers;

        // if let KeyCode::Char(c) = ke_code {}

        match (ke_kind, ke_mod, ke_code) {
            (_k, KeyModifiers::NONE, KeyCode::Char(char)) => {
                self.search_text.push(char);
            }
            (_k, KeyModifiers::NONE, KeyCode::Backspace) => {
                self.search_text.pop();
            }
            (_k, KeyModifiers::CONTROL, KeyCode::Char('q')) => self.exit = true,
            (_k, KeyModifiers::CONTROL, KeyCode::Char('j') | KeyCode::Down) => {
                self.next_row();
            }
            (_k, KeyModifiers::CONTROL, KeyCode::Char('k') | KeyCode::Up) => {
                self.previous_row();
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        // let mut table_state = TableState::default();

        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let item = data;
            let num = (i + 1).to_string();
            let cells: Vec<String> = vec![
                num,
                item.event_type.to_string(),
                item.path.clone(),
                item.timestamp.to_string(),
            ];
            Row::new(cells)
        });

        let widths = [
            Constraint::Max(3),
            Constraint::Fill(1),
            Constraint::Percentage(60),
            Constraint::Fill(1),
        ];
        let widget = Table::new(rows, widths)
            .style(Style::new().light_yellow())
            .header(
                Row::new(vec!["S/N", "Event Type", "path", "time & date"])
                    .height(1)
                    .style(Style::default().fg(Color::Magenta))
                    .bg(Color::White),
            );
        frame.render_stateful_widget(widget, area, &mut self.state);
    }
    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let widget = Line::from("Kongg on the Terminal")
            .bold()
            .centered()
            .yellow();

        frame.render_widget(widget, area);
    }
    fn render_input_search_field(&self, frame: &mut Frame, area: Rect) {
        // let widget = Line::from("Kongg on the Terminal")
        //     .bold()
        //     .centered()
        //     .yellow();

        let area = frame.area();

        let block = Block::default().title("INput").borders(Borders::all());
        let paragraph = Paragraph::new(self.search_text.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(block);

        frame.render_widget(paragraph, area);
    }
    fn render_filter_field(&self, frame: &mut Frame, area: Rect) {
        let widget = Line::from("Filter Stuff").bold().centered().yellow();

        frame.render_widget(widget, area);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }
}

// impl Widget for &App {
//     fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
//     where
//         Self: Sized,
//     {
//         let layout = Layout::default()
//             .direction(Direction::Vertical)
//             .constraints(vec![
//                 Constraint::Percentage(10),
//                 Constraint::Percentage(5),
//                 Constraint::Percentage(80),
//                 Constraint::Percentage(5),
//             ])
//             .split(area);

//         Line::from("Kongg on the Terminal")
//             .bold()
//             .centered()
//             .yellow()
//             .render(layout[0], buf);

//         Line::from("Kongg on the Terminal")
//             .bold()
//             .centered()
//             .yellow()
//             .render(layout[1], buf);

//         table_widget(self, layout[2], buf);

//         Line::from("Kongg on the Terminal")
//             .bold()
//             .centered()
//             .yellow()
//             .render(layout[3], buf);
//         //.render(area, buf);
//         //  Paragraph::new("-").centered().render(area, buf);
//     }
// }

// fn table_widget(app: &App, area: Rect, buf: &mut Buffer) {
//     let mut table_state = TableState::default();

//     let rows = [
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type2", "path2", "time & date2"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type1", "path1", "time & date1"]),
//         Row::new(vec!["Event Type4", "path4", "time & date4"]),
//     ];
//     let widths = [
//         Constraint::Fill(1),
//         Constraint::Percentage(60),
//         Constraint::Fill(1),
//     ];
//     Table::new(rows, widths)
//         .style(Style::new().light_yellow())
//         .header(Row::new(vec!["Event Type", "path", "time & date"]))
//         // .block(Block::new().title("Table").bold())
//         .render(area, buf);
// }
