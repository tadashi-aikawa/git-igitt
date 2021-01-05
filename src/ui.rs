use crate::app::{ActiveView, App};
use crate::widgets::commit_view::CommitView;
use crate::widgets::graph_view::GraphView;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::text::Text;
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use tui::Frame;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    if let ActiveView::Help(scroll) = app.active_view {
        draw_help(f, f.size(), scroll);
        return;
    }

    if app.is_fullscreen {
        match app.active_view {
            ActiveView::Graph => draw_graph(f, f.size(), app),
            ActiveView::Commit => draw_commit(f, f.size(), app),
            ActiveView::Files => draw_files(f, f.size(), app),
            ActiveView::Diff => draw_diff(f, f.size(), app),
            ActiveView::Help(_) => {}
        }
    } else {
        let base_split = if app.horizontal_split {
            Direction::Horizontal
        } else {
            Direction::Vertical
        };
        let sub_split = if app.horizontal_split {
            Direction::Vertical
        } else {
            Direction::Horizontal
        };

        let chunks = Layout::default()
            .direction(base_split)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        let right_chunks = Layout::default()
            .direction(sub_split)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);

        match app.active_view {
            ActiveView::Files | ActiveView::Diff => draw_diff(f, chunks[0], app),
            _ => draw_graph(f, chunks[0], app),
        }

        draw_commit(f, right_chunks[0], app);
        draw_files(f, right_chunks[1], app);
    }
}

fn draw_graph<B: Backend>(f: &mut Frame<B>, target: Rect, app: &mut App) {
    let mut block = Block::default()
        .borders(Borders::ALL)
        .title("Graph - H for help");
    if app.active_view == ActiveView::Graph {
        block = block.border_type(BorderType::Thick);
    }

    let graph = GraphView::default().block(block).highlight_symbol(">");

    f.render_stateful_widget(graph, target, &mut app.graph_state);
}

fn draw_commit<B: Backend>(f: &mut Frame<B>, target: Rect, app: &mut App) {
    let mut block = Block::default().borders(Borders::ALL).title("Commit");
    if app.active_view == ActiveView::Commit {
        block = block.border_type(BorderType::Thick);
    }

    let commit = CommitView::default().block(block).highlight_symbol(">");

    f.render_stateful_widget(commit, target, &mut app.commit_state);
}

fn draw_files<B: Backend>(f: &mut Frame<B>, target: Rect, app: &mut App) {
    let mut block = Block::default().borders(Borders::ALL).title("Files");
    if app.active_view == ActiveView::Files {
        block = block.border_type(BorderType::Thick);
    }
    if let Some(state) = &mut app.commit_state.content {
        let items: Vec<_> = state
            .diffs
            .items
            .iter()
            .map(|item| {
                ListItem::new(Text::styled(
                    format!("{} {}", item.1.to_string(), item.0),
                    Style::default().fg(item.1.to_color()),
                ))
            })
            .collect();
        let list = List::new(items).block(block).highlight_symbol("> ");
        f.render_stateful_widget(list, target, &mut state.diffs.state);
    } else {
        f.render_widget(block, target);
    }
}

fn draw_diff<B: Backend>(f: &mut Frame<B>, target: Rect, app: &mut App) {
    let mut block = Block::default().borders(Borders::ALL).title("Diff");
    if app.active_view == ActiveView::Diff {
        block = block.border_type(BorderType::Thick);
    }

    f.render_widget(block, target);
}

fn draw_help<B: Backend>(f: &mut Frame<B>, target: Rect, scroll: u16) {
    let block = Block::default().borders(Borders::ALL).title("Help");

    let paragraph = Paragraph::new(
        "Q                Quit\n\
         H                Show this help\n\
         Up/Down          Navigate commits\n\
         Shift + Up/Down  Navigate fast\n\
         Home/End         Navigate to first/last\n\
         Left/Right       Change panel\n\
         Tab              Fullscreen panel\n\
         Ecs              Return to default view\n\
         L                Toggle horizontal/vertical layout\n\
         R                Reload repository graph",
    )
    .block(block)
    .scroll((scroll, 0));

    f.render_widget(paragraph, target);
}
