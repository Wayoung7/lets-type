use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph, Wrap,
    },
    Frame,
};

use crate::model::{AppState, Model};

pub fn view(model: &mut Model, f: &mut Frame) {
    if model.app_state.is_running() {
        main_view(model, f);
    } else if model.app_state == AppState::Info {
        information_view(model, f);
    }
}

fn main_view(model: &mut Model, f: &mut Frame) {
    let hms = format!(
        "{:0>2}:{:0>2}:{:0>2}",
        model.time_elapsed.as_secs() / 60 / 60,
        (model.time_elapsed.as_secs() / 60) % 60,
        model.time_elapsed.as_secs() % 60
    );
    let block = Block::new()
        .title("Let's type!".white())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(
            Title::from(hms.white())
                .position(Position::Bottom)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from("Info (Tab)".black().on_white())
                .position(Position::Top)
                .alignment(Alignment::Right),
        )
        .title(
            Title::from(format!("WPM: {}", model.wpm.to_string()).yellow())
                .position(Position::Bottom)
                .alignment(Alignment::Left),
        )
        .title(
            Title::from(format!("Accuracy: {:.1}%", 100. * model.accuracy))
                .position(Position::Bottom)
                .alignment(Alignment::Right),
        )
        .title_style(Style::default().fg(Color::LightGreen).italic());

    let binding = model.current_words.clone();
    let (should_typed, not_typed) = binding.split_at(model.current_typed_len());
    let mut line: Vec<Span> = Vec::new();

    for (ct, st) in model.current_typed.chars().zip(should_typed.chars()) {
        if ct == st {
            // Correctly typed char
            line.push(Span::styled(
                ct.to_string(),
                Style::default().fg(Color::LightGreen),
            ))
        } else {
            // Wrongly typed char
            line.push(Span::styled(
                st.to_string(),
                Style::default().bg(Color::LightRed),
            ))
        }
    }
    if let Some(next_char) = not_typed.chars().nth(0) {
        // Next char
        line.push(Span::styled(
            next_char.to_string(),
            Style::default()
                .underlined()
                .underline_color(Color::White)
                .fg(Color::White),
        ));
        // Not typed chars
        line.push(Span::styled(
            not_typed
                .strip_prefix(next_char)
                .expect("Generate view error"),
            Style::default().fg(Color::Gray).dim(),
        ));
    }

    f.render_widget(
        Paragraph::new(Line::default().spans::<Vec<Span>>(line))
            .block(block.clone())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left),
        f.size(),
    )
}

fn information_view(model: &mut Model, f: &mut Frame) {
    let block = Block::new()
        .title("Information".white())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title_style(Style::default().fg(Color::LightGreen).italic());
    let lines: Vec<Line> = vec![
        Line::styled(
            format!(
                "  Number of words per page:     {}",
                model.num_words_each_line
            ),
            Style::default(),
        ),
        Line::styled(
            format!(
                "  Allow backspace:              {}",
                model.enable_backspace.to_string()
            ),
            Style::default(),
        ),
        Line::styled(
            format!(
                "  Allow typing after mistake:   {}",
                model.allow_typing_after_mistake.to_string()
            ),
            Style::default(),
        ),
    ];
    f.render_widget(
        Paragraph::new::<Text>(Text::from(lines)).block(block),
        f.size(),
    );
}
