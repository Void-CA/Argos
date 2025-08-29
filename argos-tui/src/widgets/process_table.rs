use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Row, Table, TableState},
    style::{Style, Color, Modifier},
    layout::Constraint,
};

// process_table.rs
pub struct ProcessTable<'a> {
    pub header: Row<'a>,
    pub data_rows: Vec<Row<'a>>,
    pub widths: Vec<Constraint>,
    pub state: TableState,
}

impl<'a> ProcessTable<'a> {
    pub fn new(header: Row<'a>, data_rows: Vec<Row<'a>>, widths: Vec<Constraint>) -> Self {
        Self {
            header,
            data_rows,
            widths,
            state: TableState::default(),
        }
    }

    pub fn init_selection(&mut self) {
        if self.state.selected().is_none() && !self.data_rows.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn select_up(&mut self) {
        let selected = self.state.selected().unwrap_or(0);
        if selected > 0 {
            self.state.select(Some(selected - 1));
        } else if !self.data_rows.is_empty() {
            self.state.select(Some(self.data_rows.len() - 1));
        }
    }

    pub fn select_down(&mut self) {
        let selected = self.state.selected().unwrap_or(0);
        if selected < self.data_rows.len() - 1 {
            self.state.select(Some(selected + 1));
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        self.init_selection();

        // SOLUCIÓN: Usar solo data_rows para Table::new()
        // y header separado con .header()
        let table = Table::new(self.data_rows.clone(), self.widths.clone())
            .block(Block::default().title("Procesos").borders(Borders::ALL))
            .header(self.header.clone())  // Este es el único header
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ")
            .column_spacing(1);

        f.render_stateful_widget(table, area, &mut self.state);
    }
}