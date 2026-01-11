use std::{io, time::{self, Duration, Instant}};

use color_eyre::owo_colors::OwoColorize;
use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind}, layout::{Constraint, Direction, Layout, Rect}, style::Stylize, text::{Line, Text, ToText}, widgets::{Block, Borders, Paragraph, Widget}};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = App { clock: Clockwatch { elapsed_time: Duration::ZERO, running: false, laps: vec![] }, exit: false, last_frame: Instant::now() };
    let app_result = app.run(&mut terminal);

    ratatui::restore();

    app_result?;

    Ok(())
}

#[derive(Debug)]
struct App {
    clock: Clockwatch, // clockwatch widget
    exit: bool, // bool for exit
    last_frame: Instant
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            let dt = self.last_frame.elapsed();
            self.last_frame = Instant::now();

            self.handle_events()?;
            self.update(dt);

            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    pub fn update(&mut self, dt: Duration) {
        self.clock.update(dt);
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&self.clock, frame.area());
        frame.render_widget(self, frame.area());
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        while event::poll(Duration::from_millis(0))? {
            if let event::Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_pressed_event(key_event)?;
                }
            }
        }
        Ok(())
    }

    pub fn handle_key_pressed_event(&mut self, key_event: KeyEvent) -> io::Result<()>{
        match key_event.code {
            KeyCode::Char('q') => {
                self.exit = true;
                Ok(())
            }
            KeyCode::Char(' ') => {
                self.clock.toggle_start_pause();
                Ok(())
            }
            KeyCode::Char('l') => {
                self.clock.lap();
                Ok(())
            }
            _ => {Ok(())}
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {

        let title = Line::from(" Clockwatch rust app ".bold()).centered();
        
        let instructions = Line::from(vec![
            " Pause/Start ".into(),
            "<Space>".blue().bold(),
            " Lap ".into(),
            "<l>".blue().bold(),
            " Exit ".into(),
            "<q>".blue().bold(),
        ]).centered();

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_bottom(instructions);
        
        Paragraph::default()
            .block(block)
            .render(area, buf);
    }
}

#[derive(Debug)]
struct Clockwatch {
    running: bool,
    elapsed_time: Duration, // accum time
    laps: Vec<Duration>, // laps in seconds 
}

impl Clockwatch {
    fn update(&mut self, dt: Duration) {
        if self.running {
            self.elapsed_time += dt;
        }
    }

    fn toggle_start_pause(&mut self) {
        self.running = !self.running;
    }

    fn lap(&mut self) {
        self.laps.push(self.elapsed_time);
    }

    fn duration_into_text(dt: Duration) -> String {
        let all_millis = dt.as_millis();
        let hours: u128 = all_millis / 1000 / 60 / 60;
        let minutes: u128 = all_millis / 1000 / 60 % 60;
        let secs: u128 = all_millis / 1000 % 60;
        let millis: u128 = all_millis % 1000;
        format!("{:02}:{:02}:{:02}:{:03}", hours, minutes, secs, millis)
    }
}

impl Widget for &Clockwatch {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {

        let clock_text = Text::from(vec![Line::from(Clockwatch::duration_into_text(self.elapsed_time))]);

        let mut laps_text = Text::from(vec![Line::from("Laps:")]);
        for lap in &self.laps.iter().rev().collect::<Vec<&Duration>>() {
            laps_text.push_line(Line::from(Clockwatch::duration_into_text(**lap)));
        }

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Length(1),
                Constraint::Min(0),
            ]).split(area);

        Paragraph::new(clock_text)
            .centered()
            .render(layout[1], buf);

        Paragraph::new(laps_text)
            .centered()
            .render(layout[2], buf);
    }
}
