mod lessons;
mod session;
mod stats;
mod storage;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph},
    Terminal,
};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut selected = 0;
    let options = [
        ("1", "Practicar", Color::Rgb(100, 255, 150)),
        ("2", "Ver progreso", Color::Rgb(255, 150, 100)),
        ("3", "Salir", Color::Rgb(255, 100, 150)),
    ];

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(8),
                    Constraint::Min(6),
                    Constraint::Length(3),
                ])
                .split(size);

            let logo = Paragraph::new(vec![
                Line::from("                                              "),
                Line::from("████████╗██╗   ██╗██████╗ ██╗███████╗████████╗"),
                Line::from("╚══██╔══╝╚██╗ ██╔╝██╔══██╗██║██╔════╝╚══██╔══╝"),
                Line::from("   ██║    ╚████╔╝ ██████╔╝██║███████╗   ██║   "),
                Line::from("   ██║     ╚██╔╝  ██╔═══╝ ██║╚════██║   ██║   "),
                Line::from("   ██║      ██║   ██║     ██║███████║   ██║   "),
                Line::from("   ╚═╝      ╚═╝   ╚═╝     ╚═╝╚══════╝   ╚═╝   "),
                Line::from("         C O N S O L E   E D I T I O N        "),
                Line::from("                                              "),
            ])
            .style(
                Style::default()
                    .fg(Color::Rgb(70, 210, 255))
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

            let menu_block = Block::default().borders(Borders::NONE);

            let menu_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Length(5),
                    Constraint::Min(0),
                ])
                .split(menu_block.inner(main_chunks[1]));

            for (i, (num, text, color)) in options.iter().enumerate() {
                let is_selected = i == selected;

                let item_block = Block::default()
                    .borders(Borders::NONE)
                    .padding(Padding::vertical(2))
                    .style(if is_selected {
                        Style::default()
                            .bg(Color::Rgb(80, 80, 100))
                            .fg(Color::White)
                    } else {
                        Style::default().bg(Color::Rgb(20, 20, 30)).fg(*color)
                    });

                let item = Paragraph::new(format!("[{}] {}", num, text))
                    .alignment(Alignment::Center)
                    .block(item_block);

                f.render_widget(item, menu_chunks[i + 1]);
            }
            let footer_block = Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray));

            let footer = Paragraph::new("↑↓ Seleccionar opción | ENTER Confirmar | ESC Salir")
                .style(
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center)
                .block(footer_block);

            f.render_widget(logo, main_chunks[0]);
            f.render_widget(menu_block, main_chunks[1]);
            f.render_widget(footer, main_chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => session::start_practice(terminal)?,
                KeyCode::Char('2') => show_popup(terminal, " 🚧 En desarrollo", "Próximamente...")?,
                KeyCode::Char('3') => break,
                KeyCode::Up => selected = selected.saturating_sub(1),
                KeyCode::Down => selected = selected.saturating_add(1).min(options.len() - 1),
                KeyCode::Enter => match selected {
                    0 => session::start_practice(terminal)?,
                    1 => show_popup(terminal, " 🚧 En desarrollo", "Próximamente...")?,
                    2 => break,
                    _ => (),
                },
                KeyCode::Esc => break,
                _ => (),
            }
        }
    }
    Ok(())
}

fn show_popup<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    title: &str,
    message: &str,
) -> io::Result<()> {
    terminal.draw(|f| {
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Rgb(30, 30, 40)));

        let paragraph = Paragraph::new(message)
            .block(block)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        let area = centered_rect(50, 20, f.area());
        f.render_widget(paragraph, area);
    })?;
    event::read()?;
    Ok(())
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

