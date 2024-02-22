use ratatui::{
    layout::{Alignment, Offset},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph, Wrap,
    },
    Frame,
};

use crate::model::Model;

pub fn view(model: &mut Model, f: &mut Frame) {
    if model.app_state.is_running() {
        let block = Block::new()
            .title("Let's type!".white())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(
                Title::from(format!("Accuracy: {:.1}%", 100. * model.accuracy))
                    .position(Position::Bottom)
                    .alignment(Alignment::Right),
            )
            .title_style(Style::default().fg(Color::LightGreen).italic());
        // f.render_widget(Paragraph::new(format!("{:?}", model.app_state)), f.size());
        // f.render_widget(
        //     Paragraph::new(format!("{}", model.current_words)),
        //     f.size().offset(Offset { x: 0, y: 4 }),
        // );
        let binding = model.current_words.clone();
        let (should_typed, untyped) = binding.split_at(model.current_typed_len());
        // f.render_widget(
        //     Paragraph::new(typed.bg(Color::Green))
        //         .block(block.clone())
        //         .wrap(Wrap { trim: true }),
        //     f.size(),
        // );
        // f.render_widget(
        //     Paragraph::new(untyped.fg(Color::Gray))
        //         .block(block)
        //         .wrap(Wrap { trim: true }),
        //     f.size(),
        // );
        // let (mut typed_correct, mut typed_wrong) = (String::new(), String::new());
        let mut line: Vec<Span> = Vec::new();

        for (ct, st) in model.current_typed.chars().zip(should_typed.chars()) {
            if ct == st {
                line.push(Span::styled(
                    ct.to_string(),
                    Style::default().fg(Color::LightGreen),
                ))
            } else {
                line.push(Span::styled(
                    st.to_string(),
                    Style::default().bg(Color::LightRed),
                ))
            }
        }
        if let Some(next_char) = untyped.chars().nth(0) {
            line.push(Span::styled(
                next_char.to_string(),
                Style::default()
                    .underlined()
                    .underline_color(Color::White)
                    .fg(Color::White),
            ));
            line.push(Span::styled(
                untyped.strip_prefix(next_char).unwrap(),
                Style::default().fg(Color::Gray),
            ));
        }
        // (model.current_typed.chars().zip(other), model.current_words.chars()).

        f.render_widget(
            Paragraph::new(Line::default().spans::<Vec<Span>>(line))
                .block(block.clone())
                .wrap(Wrap { trim: true })
                .alignment(Alignment::Left),
            f.size(),
        )
    }
    // f.render_widget(
    //     Paragraph::new(format!(
    //         "{:?} {} {}, {} {}",
    //         model.app_state,
    //         model.current_words_len(),
    //         model.current_typed_len(),
    //         model.num_correct,
    //         model.num_mistake,
    //     )),
    //     f.size().offset(Offset { x: 2, y: 6 }),
    // );
}
