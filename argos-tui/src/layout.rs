use ratatui::{prelude::*};
use crate::widgets::{header::Header, footer::Footer, process_table::ProcessTable};

pub fn draw_layout(f: &mut Frame, process_table: &mut ProcessTable) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),    // Header
            Constraint::Min(10),      // Tabla de procesos (mínimo 10 líneas)
            Constraint::Length(3)     // Footer
        ])
        .split(size);

    // Render header
    let header = Header::new("Argos TUI - Monitoreo de procesos");
    f.render_widget(header.render(), chunks[0]);

    // Render process table - pasamos el área y el frame
    process_table.render(f, chunks[1]);

    // Render footer
    let footer = Footer::new("Presiona 'q' para salir, ↑/↓ para navegar");
    f.render_widget(footer.render(), chunks[2]);
}