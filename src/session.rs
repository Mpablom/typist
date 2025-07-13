use crate::{lessons, stats::TypingStats};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

pub fn start_practice<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let lesson = lessons::get_random_lesson();
    let mut input = String::new();
    let mut mistakes = 0;
    let mut started = false;
    let mut start_time = Instant::now();

    loop {
        let input_chars: Vec<char> = input.chars().collect();
        let input_len = input_chars.len();

        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(60),
                    Constraint::Percentage(30),
                    Constraint::Min(1),
                ])
                .split(size);

            let mut spans = Vec::new();
            for (i, c) in lesson.chars().enumerate() {
                let style = match i.cmp(&input_len) {
                    std::cmp::Ordering::Less => {
                        let typed_char = input_chars[i];
                        if typed_char == c {
                            Style::default()
                                .fg(Color::Rgb(100, 255, 100))
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                                .fg(Color::Rgb(255, 100, 100))
                                .add_modifier(Modifier::CROSSED_OUT)
                        }
                    }
                    std::cmp::Ordering::Equal => Style::default()
                        .fg(Color::Rgb(255, 255, 100))
                        .bg(Color::Rgb(50, 50, 70))
                        .add_modifier(Modifier::UNDERLINED),
                    std::cmp::Ordering::Greater => Style::default().fg(Color::Rgb(200, 200, 200)),
                };

                spans.push(Span::styled(c.to_string(), style));
            }

            let target_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(70, 210, 255)))
                .title(" 🚀 ESCRIBE ESTE TEXTO ")
                .title_alignment(Alignment::Center)
                .style(Style::default().bg(Color::Rgb(20, 20, 30)));

            let target_paragraph = Paragraph::new(Text::from(Line::from(spans)))
                .block(target_block)
                .wrap(Wrap { trim: true });

            let input_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(100, 255, 150)))
                .title(" ✍️ TU ESCRITURA ")
                .title_alignment(Alignment::Center)
                .style(Style::default().bg(Color::Rgb(25, 25, 35)));

            let input_paragraph = Paragraph::new(input.as_str())
                .block(input_block)
                .style(Style::default().fg(Color::White))
                .scroll((input.lines().count().saturating_sub(3) as u16, 0));

            f.render_widget(target_paragraph, chunks[0]);
            f.render_widget(input_paragraph, chunks[1]);

            if input_len < lesson.len() {
                let current_char = lesson.chars().nth(input_len).unwrap();
                let indicator = Paragraph::new(format!("Próximo carácter: '{}'", current_char))
                    .style(Style::default().fg(Color::Rgb(150, 150, 255)))
                    .alignment(Alignment::Center);
                f.render_widget(indicator, chunks[2]);
            }
        })?;

        if !started {
            started = true;
            start_time = Instant::now();
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        if input.len() < lesson.len() {
                            input.push(c);
                            let pos = input.chars().count() - 1;
                            if lesson.chars().nth(pos).unwrap() != c {
                                mistakes += 1;
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        if input.len() == lesson.len() {
            let duration = start_time.elapsed().as_secs_f64();
            let wpm = (input.len() as f64 / 5.0) / (duration / 60.0);
            let stats = TypingStats {
                total_chars: input.len(),
                mistakes,
                wpm,
            };

            terminal.draw(|f| {
                let size = f.area();
                let main_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(8), Constraint::Length(4)])
                    .split(size);

                let result_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(255, 150, 100)))
                    .title(" 🎉 RESULTADOS ")
                    .title_alignment(Alignment::Center)
                    .style(Style::default().bg(Color::Rgb(20, 20, 30)));

                let inner_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ])
                    .split(result_block.inner(main_chunks[0]));

                f.render_widget(result_block, main_chunks[0]);

                let metric_style = Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD);

                let chars = Paragraph::new(format!("⌨️  Caracteres: {}", stats.total_chars))
                    .style(metric_style)
                    .alignment(Alignment::Center);
                f.render_widget(chars, inner_chunks[0]);

                let mistakes = Paragraph::new(format!("❌  Errores: {}", stats.mistakes))
                    .style(metric_style)
                    .alignment(Alignment::Center);
                f.render_widget(mistakes, inner_chunks[1]);

                let accuracy = Paragraph::new(format!("🎯  Precisión: {:.2}%", stats.accuracy()))
                    .style(metric_style)
                    .alignment(Alignment::Center);
                f.render_widget(accuracy, inner_chunks[2]);

                let wpm = Paragraph::new(format!("🚀  Velocidad (WPM): {:.2}", stats.wpm))
                    .style(metric_style)
                    .alignment(Alignment::Center);
                f.render_widget(wpm, inner_chunks[3]);

                let footer = Paragraph::new("Presiona ESC para volver al menú")
                    .style(Style::default().fg(Color::Rgb(255, 255, 100)))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .border_style(Style::default().fg(Color::Rgb(70, 70, 90))),
                    );
                f.render_widget(footer, main_chunks[1]);
            })?;

            loop {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc {
                        return Ok(());
                    }
                }
            }
        }
    }

    Ok(())
}
