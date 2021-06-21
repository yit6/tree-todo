mod tree;

use crossterm::{
    event::{self,Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode}
};
use std::sync::mpsc;
use std::io;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph},
    Terminal,
};


enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    enable_raw_mode();

    let mut tree = tree::TreeNode::new("root".to_string());
    tree.add_child(tree::TreeNode::new("child".to_string()));
    tree.add_child(tree::TreeNode::new("child".to_string()));
    tree.add_child(tree::TreeNode::new("child".to_string()));
    tree.add_child(tree::TreeNode::new("child".to_string()));
    tree.children[0].add_child(tree::TreeNode::new("chsdfasfasfasfda".to_string()));
    tree.children[1].add_child(tree::TreeNode::new("chsdfasfasfasfda".to_string()));
    tree.children[1].add_child(tree::TreeNode::new("chsdfasfasfasfda".to_string()));
    tree.children[1].children[1].add_child(tree::TreeNode::new("chsdfasfasfasfda".to_string()));
    tree.children[2].add_child(tree::TreeNode::new("chsdfasfasfasfda".to_string()));

    let mut selected_index = 0;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref()
                )
                .split(size);

            let mut selected = ListState::default();
            selected.select(Some(selected_index));

            let body = List::new(tree.to_list_items_without_self())
                .highlight_style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                );

            let footer = Paragraph::new("instructions to use, test with tui library, blah blah blah")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                );
            rect.render_stateful_widget(body, chunks[0], &mut selected);
            rect.render_widget(footer, chunks[1]);
        });

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                },
                KeyCode::Char('j') | KeyCode::Down => {
                    if selected_index < tree.to_list_items_without_self().len()-1 {
                        selected_index += 1;
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Char(' ') => {
                    let mut flattened = tree.to_done_list();
                    flattened[selected_index] = &true;
                }
                _ => {},
            },
            _ => {},
        }
    }
    Ok(())
}
