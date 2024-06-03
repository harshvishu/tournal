pub mod cli;
pub mod db;
pub mod errors;
pub mod models;
pub mod schema;
pub mod tui;

use color_eyre::eyre::Context;
use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::{Color, Modifier, Style},
    symbols::border,
    widgets::{
        block::*, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table,
        Tabs, Widget,
    },
    Frame,
};
use std::{
    io::{self, stdout},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
    vec,
};

use cli::pet::{Event, MenuItem};
use models::User;

#[derive(Default, Debug)]
struct App {
    active_menu_item: MenuItem,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame));
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());

        //match self.active_menu_item {
        //    MenuItem::Home => frame.render_widget(render_home(), chunks[1]),
        //    MenuItem::Users => {
        //        let user_chunks = Layout::default()
        //            .direction(Direction::Horizontal)
        //            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        //            .split(chunks[1]);

        //        let (left, right) = render_users(&users_list_state);
        //        frame.render_stateful_widget(left, user_chunks[0], &mut users_list_state);
        //        frame.render_widget(right, user_chunks[1]);
        //    }
        //}
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(key) if key.kind == KeyEventKind::Press => {
                self.handle_key_events(key)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit(),
            //KeyCode::Char('h') => active_menu_item = MenuItem::Home,
            //KeyCode::Char('u') => active_menu_item = MenuItem::Users,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true
    }

    fn render_home<'a>() -> Paragraph<'a> {
        Paragraph::new(vec![
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Welcome")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("to")]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::styled(
                "pet-CLI",
                Style::default().fg(Color::LightBlue),
            )]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(
                "Press 'u' to access users, 'a' to add users and 'd' to delete selected user.",
            )]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        )
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area);

        let (header, mid, footer) = (chunks[0], chunks[1], chunks[2]);

        let menu_titles = ["Home", "Users", "Add", "Delete", "Quit"];

        let copyright = Paragraph::new("Copyright @ harsh 2024 - All rights reserved")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::all())
                    .style(Style::default().fg(Color::White))
                    .title("Copyright")
                    .border_type(BorderType::Plain),
            );

        copyright.render(footer, buf);

        let menu: Vec<_> = menu_titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Line::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();

        let tabs = Tabs::new(menu)
            .select(self.active_menu_item.into())
            .block(Block::default().title("Menu").borders(Borders::all()))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .divider(Span::raw("|"));

        tabs.render(header, buf);

        match self.active_menu_item {
            MenuItem::Home => App::render_home().render(chunks[1], buf),
            MenuItem::Users => {
                let user_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                    .split(mid);

                //let (left, right) = render_users(&users_list_state);
                //frame.render_stateful_widget(left, user_chunks[0], &mut users_list_state);
                //frame.render_widget(right, user_chunks[1]);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen);

    enable_raw_mode().expect("Can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    let mut users_list_state = ListState::default();
    users_list_state.select(Some(0));

    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let time_out = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(time_out).expect("Poll works") {
                if let crossterm::event::Event::Key(key) = event::read().expect("Can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = ["Home", "Users", "Add", "Delete", "Quit"];
    let mut active_menu_item = MenuItem::Home;

    loop {
        let _ = terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let copyright = Paragraph::new("Copyright @ harsh 2024 - All rights reserved")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::all())
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            frame.render_widget(copyright, chunks[2]);

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::all()))
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .divider(Span::raw("|"));

            frame.render_widget(tabs, chunks[0]);

            match active_menu_item {
                MenuItem::Home => frame.render_widget(render_home(), chunks[1]),
                MenuItem::Users => {
                    let user_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);

                    let (left, right) = render_users(&users_list_state);
                    frame.render_stateful_widget(left, user_chunks[0], &mut users_list_state);
                    frame.render_widget(right, user_chunks[1]);
                }
            }
        }); // End of draw rect closure

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                event::Event::Key(key) => match key.code {
                    KeyCode::Char('q') => {
                        io::stdout().execute(LeaveAlternateScreen)?;
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                    KeyCode::Char('u') => active_menu_item = MenuItem::Users,
                    _ => {}
                },
                _ => {}
            }
        }

        //match rx.recv()? {
        //    Event::Input(event) => match event.code {
        //        KeyCode::Char('q') => {
        //            io::stdout().execute(LeaveAlternateScreen)?;
        //            disable_raw_mode()?;
        //            terminal.show_cursor()?;
        //            break;
        //        }
        //        KeyCode::Char('h') => active_menu_item = MenuItem::Home,
        //        KeyCode::Char('u') => active_menu_item = MenuItem::Users,
        //        _ => {}
        //    },
        //    Event::Tick => {}
        //}
    } // end of render loop

    Ok(())
}

fn render_home<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "pet-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(
            "Press 'u' to access users, 'a' to add users and 'd' to delete selected user.",
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    )
}

fn read_db() -> Result<Vec<User>, db::Error> {
    let mut conn = db::establish_connection();
    let new_user = db::get_users(&mut conn)?;
    Ok(new_user)

    //let db_content = fs::read_to_string("DB_PATH")?;
    //let parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
    //Ok(parsed)
}

fn render_users<'a>(users_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let users = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Users")
        .border_type(BorderType::Plain);

    let user_list = read_db().expect("Can fetch pet list");
    let items: Vec<_> = user_list
        .iter()
        .map(|user| {
            ListItem::new(Spans::from(vec![Span::styled(
                user.username.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_user = user_list
        .get(
            users_list_state
                .selected()
                .expect("There is always a selected pet"),
        )
        .expect("User list can not be empty");

    let list = List::new(items).block(users).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let user_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_user.user_id.unwrap_or(0).to_string())),
        Cell::from(Span::raw(selected_user.username.clone())),
        Cell::from(Span::raw(selected_user.email.clone())),
        Cell::from(Span::raw(
            selected_user.created_at.clone().unwrap_or(String::from("")),
        )),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Email",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ]);

    (list, user_detail)
}
*/
