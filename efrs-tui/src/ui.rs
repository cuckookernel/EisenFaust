use crate::app::App;
use crate::app::Screen;
use crate::app::WhatToShow;
use crate::exec_cell::ExecCell;
use tui::text::Spans;
use tui::widgets::Paragraph;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    layout::Constraint as C,
    style::{Color, Style},  // Modifier
    // symbols,
    text::{Span}, // , Spans
    // widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        // Axis, BarChart,
        Block, Borders, Cell,
        // Chart, Dataset, Gauge, LineGauge,
         List, ListItem,
        // Paragraph,
        Row,
        // Sparkline,
        Table
        // , Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks: Vec<Rect> = Layout::default()
        .constraints([C::Min(0), C::Length(1)].as_ref()).split(f.size());
    match app.active_screen {
        Screen::Commander => {
            draw_commander_screen(f, app, chunks[0])
        },
        Screen::LogMessages => {
            draw_log_msgs_screen(f, app, chunks[0])
        }
    }
}


fn draw_commander_screen<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)
{
    let chunks = Layout::default()
        .constraints([Constraint::Ratio(16, 100),
                  Constraint::Min(8)])
        .direction(Direction::Horizontal)
        .split(area);


    if app.to_show.contains(WhatToShow::CmdGroups) {
        draw_list(f, &app.cmd_groups.items,
                  app.cmd_groups.state.selected(),
                  &"Cmd Groups", chunks[0]);
    } else if app.to_show.contains(WhatToShow::CmdPicker) {
        draw_list(f, &app.cmd_picker.items,
                  app.cmd_picker.state.selected(),
                  &"Commands", chunks[0]);
    }

    draw_cells(f, &app.exec_cells, Some(0), chunks[1])
}


fn draw_log_msgs_screen<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(area);

    let height = (chunks[0].height - 1) as usize;
    let n_items = app.msgs.items.len();
    let first = 0.max(n_items - height.min(n_items)).min(n_items);
    let items: Vec<Row> = app.msgs.items[first..]
        .iter()
        .map(|c| {
            let cells = vec![
                Cell::from(Span::styled(c, Style::default().fg(Color::White)))

            ];
            Row::new(cells)
        })
        .collect();
    let table = Table::new(items)
        .block(Block::default().title("Messages").borders(Borders::ALL))
        .widths(&[
            Constraint::Ratio(1, 1),
        ]);
    f.render_widget(table, chunks[0]);
}


fn draw_list<B: Backend>(f: &mut Frame<B>, a_list: &Vec<String>, selected_idx: Option<usize>,
                         title: &dyn AsRef<str>, area: Rect)

{
    let selected_idx1 = selected_idx.unwrap_or(usize::MAX);

    let items: Vec<ListItem> = a_list
        .iter().enumerate()
        .map(|(i, c)| {

            let f_char = if i == selected_idx1 { '▹' } else { ' ' };
            ListItem::new(format!("{}{}", f_char, c))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default()
        .title(title.as_ref())
        .borders(Borders::ALL));

    f.render_widget(list, area);
}

fn draw_cells<B: Backend>(f: &mut Frame<B>, cells: &Vec<ExecCell>,
                          _sel_idx: Option<usize>, area: Rect) {
    if cells.len() == 0 { return }

    let cell = &cells[cells.len() -1];
    let text = vec![Spans::from(vec![Span::raw(cell.cmd_tmpl.clone())])];

    // https://docs.rs/tui/latest/tui/widgets/struct.Paragraph.html
    let par = Paragraph::new(text);

    f.render_widget(par, area)
}