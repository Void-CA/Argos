// app.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Instant, Duration};
use crossterm::event::{self, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Row};
use ratatui::style::{Style, Modifier};
use crate::widgets::process_table::ProcessTable;
use crate::widgets::header::Header;
use crate::widgets::footer::Footer;
use argos_core::commands::list::list_processes;
use argos_core::process::model::ProcessRow; // Asegúrate de importar ProcessRow
use crate::search::SearchField; // Importar el trait Searchable
// app.rs
use std::collections::VecDeque;

pub enum Mode {
    Normal,
    Search,
    Filter,
}

pub struct App {
    pub should_quit: bool,
    pub process_table: ProcessTable<'static>,
    pub last_update: Instant,
    pub update_interval: Duration,
    pub process_count: usize,
    pub processes_data: Arc<Mutex<Option<(Vec<ProcessRow>, usize)>>>, // Cambiar a ProcessRow
    pub all_processes: Vec<ProcessRow>, // Almacenar los datos originales
    pub mode: Mode,
    pub search_query: String,
    pub filter_query: String,
    pub search_history: VecDeque<String>,
    pub filter_history: VecDeque<String>,
}

impl App {
    pub fn new() -> Self {
        let (header, initial_data, widths, count) = Self::load_process_data();
        let all_processes = list_processes().unwrap_or_else(|_| vec![]); // Obtener datos originales
        
        let processes_data = Arc::new(Mutex::new(None));
        
        Self::start_background_updater(Arc::clone(&processes_data));
        
        Self {
            should_quit: false,
            process_table: ProcessTable::new(header, initial_data.clone(), widths),
            last_update: Instant::now(),
            update_interval: Duration::from_secs(2),
            process_count: count,
            processes_data,
            all_processes, // Guardar datos originales
            mode: Mode::Normal,
            search_query: String::new(),
            filter_query: String::new(),
            search_history: VecDeque::with_capacity(10),
            filter_history: VecDeque::with_capacity(10),
        }
    }

    // Método para convertir ProcessRow a Row
    fn process_to_row(process: &ProcessRow) -> Row<'static> {
        Row::new(vec![
            process.pid.to_string(),
            process.name.clone(),
            format!("{:.2}", process.cpu_usage),
            format!("{:.2} MB", process.memory_mb),
            process.user.clone(),
            process.state.clone(),
            format!("{:.2}", process.read_disk_usage),
            format!("{:.2}", process.write_disk_usage),
            process.start_time_human.clone(),
            process.parent_pid.map_or("-".to_string(), |pp| pp.to_string()),
            format!("{:.2} MB", process.virtual_memory_mb),
        ])
    }

    // Método para convertir vector de ProcessRow a vector de Row
    fn processes_to_rows(processes: &[ProcessRow]) -> Vec<Row<'static>> {
        processes.iter().map(Self::process_to_row).collect()
    }


    // Actualizar el background updater
    fn start_background_updater(processes_data: Arc<Mutex<Option<(Vec<ProcessRow>, usize)>>>) {
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(2));
                
                if let Ok(processes) = list_processes() {
                    let count = processes.len();
                    let mut data = processes_data.lock().unwrap();
                    *data = Some((processes, count));
                }
            }
        });
    }

    // Actualizar load_process_data
    fn load_process_data() -> (Row<'static>, Vec<Row<'static>>, Vec<Constraint>, usize) {
        let header = Row::new(vec![
            "PID",
            "Nombre",
            "CPU%",
            "Memoria",
            "Usuario",
            "Estado",
            "Lectura Disco",
            "Escritura Disco",
            "Inicio",
            "PID Padre",
            "Memoria Virtual",
        ])
        .style(Style::default().add_modifier(Modifier::BOLD));

        let processes = list_processes().unwrap_or_else(|_| vec![]);
        let count = processes.len();

        let data_rows = Self::processes_to_rows(&processes);

        let widths = vec![
            Constraint::Length(7),
            Constraint::Length(20),
            Constraint::Length(7),
            Constraint::Length(12),
            Constraint::Length(15),
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(18),
        ];

        (header, data_rows, widths, count)
    }
    
    // Método para buscar procesos (modificado)
    pub fn search_processes(&mut self, query: &str) {
        let filtered_rows = if query.is_empty() {
            // Si la búsqueda está vacía, mostrar todos los procesos
            Self::processes_to_rows(&self.all_processes)
        } else {
            let query_lower = query.to_lowercase();
            let filtered: Vec<ProcessRow> = self.all_processes
                .iter()
                .filter(|process| {
                    // Buscar en todos los campos del proceso
                    process.pid.to_string().to_lowercase().contains(&query_lower) ||
                    process.name.to_lowercase().contains(&query_lower) ||
                    process.user.to_lowercase().contains(&query_lower) ||
                    process.state.to_lowercase().contains(&query_lower) ||
                    process.start_time_human.to_lowercase().contains(&query_lower)
                })
                .cloned()
                .collect();
            
            Self::processes_to_rows(&filtered)
        };

        // Ahora actualizamos self después de terminar el borrow
        self.process_table.data_rows = filtered_rows;
        self.process_count = self.process_table.data_rows.len();
        
        // Guardar en historial
        if !query.is_empty() && !self.search_history.contains(&query.to_string()) {
            if self.search_history.len() >= 10 {
                self.search_history.pop_back();
            }
            self.search_history.push_front(query.to_string());
        }
    }

    // Método para filtrar por campo específico (modificado)
    pub fn filter_processes(&mut self, field: usize, query: &str) {
        let filtered_rows = if query.is_empty() {
            Self::processes_to_rows(&self.all_processes)
        } else {
            let query_lower = query.to_lowercase();
            let filtered: Vec<ProcessRow> = self.all_processes
                .iter()
                .filter(|process| {
                    match field {
                        0 => process.pid.to_string().to_lowercase().contains(&query_lower),
                        1 => process.name.to_lowercase().contains(&query_lower),
                        2 => format!("{:.2}", process.cpu_usage).to_lowercase().contains(&query_lower),
                        3 => format!("{:.2}", process.memory_mb).to_lowercase().contains(&query_lower),
                        4 => process.user.to_lowercase().contains(&query_lower),
                        5 => process.state.to_lowercase().contains(&query_lower),
                        6 => format!("{:.2}", process.read_disk_usage).to_lowercase().contains(&query_lower),
                        7 => format!("{:.2}", process.write_disk_usage).to_lowercase().contains(&query_lower),
                        8 => process.start_time_human.to_lowercase().contains(&query_lower),
                        9 => process.parent_pid.map_or(false, |pp| pp.to_string().to_lowercase().contains(&query_lower)),
                        10 => format!("{:.2}", process.virtual_memory_mb).to_lowercase().contains(&query_lower),
                        _ => false,
                    }
                })
                .cloned()
                .collect();
            
            Self::processes_to_rows(&filtered)
        };

        // Actualizar después de terminar el borrow
        self.process_table.data_rows = filtered_rows;
        self.process_count = self.process_table.data_rows.len();
        
        // Guardar en historial de filtros
        let history_entry = format!("{}:{}", field, query);
        if !self.filter_history.contains(&history_entry) {
            if self.filter_history.len() >= 10 {
                self.filter_history.pop_back();
            }
            self.filter_history.push_front(history_entry);
        }
    }

    // Método para limpiar búsqueda/filtro
    pub fn clear_search(&mut self) {
        let all_rows = Self::processes_to_rows(&self.all_processes);
        self.process_table.data_rows = all_rows;
        self.process_count = self.all_processes.len();
        self.search_query.clear();
        self.filter_query.clear();
        self.mode = Mode::Normal;
    }

    // Actualizar el método refresh_processes
    pub fn refresh_processes(&mut self) {
        let mut data = self.processes_data.lock().unwrap();
        if let Some((new_process_data, new_count)) = data.take() {
            // Guardar todos los procesos (datos originales)
            self.all_processes = new_process_data;
            
            // Aplicar búsqueda/filtro actual si existe
            let filtered_rows = if !self.search_query.is_empty() {
                // Crear una copia temporal para evitar el borrow
                let query = self.search_query.clone();
                let query_lower = query.to_lowercase();
                let filtered: Vec<ProcessRow> = self.all_processes
                    .iter()
                    .filter(|process| {
                        process.pid.to_string().to_lowercase().contains(&query_lower) ||
                        process.name.to_lowercase().contains(&query_lower) ||
                        process.user.to_lowercase().contains(&query_lower) ||
                        process.state.to_lowercase().contains(&query_lower) ||
                        process.start_time_human.to_lowercase().contains(&query_lower)
                    })
                    .cloned()
                    .collect();
                
                Self::processes_to_rows(&filtered)
            } else if !self.filter_query.is_empty() {
                // Crear una copia temporal para evitar el borrow
                let query = self.filter_query.clone();
                let query_lower = query.to_lowercase();
                let filtered: Vec<ProcessRow> = self.all_processes
                    .iter()
                    .filter(|process| {
                        process.name.to_lowercase().contains(&query_lower)
                    })
                    .cloned()
                    .collect();
                
                Self::processes_to_rows(&filtered)
            } else {
                Self::processes_to_rows(&self.all_processes)
            };

            self.process_table.data_rows = filtered_rows;
            self.process_count = self.process_table.data_rows.len();
            
            self.last_update = Instant::now();
            
            // Mantener la selección
            let current_selection = self.process_table.state.selected();
            if let Some(selected) = current_selection {
                if selected < self.process_table.data_rows.len() {
                    self.process_table.state.select(Some(selected));
                } else if !self.process_table.data_rows.is_empty() {
                    self.process_table.state.select(Some(self.process_table.data_rows.len() - 1));
                }
            }
        }
    }

    pub fn should_update(&self) -> bool {
        Instant::now().duration_since(self.last_update) >= self.update_interval
    }

    
    pub fn draw(&mut self, f: &mut Frame) {
        if self.should_update() {
            self.refresh_processes();
        }

        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3)
            ])
            .split(size);

        // Header con información de búsqueda
        let header_text = match self.mode {
            Mode::Normal => format!(
                "Argos TUI - {} procesos{} | Actualizando cada {}s",
                self.process_count,
                if !self.search_query.is_empty() {
                    format!(" (buscando: '{}')", self.search_query)
                } else if !self.filter_query.is_empty() {
                    format!(" (filtrado: '{}')", self.filter_query)
                } else {
                    String::new()
                },
                self.update_interval.as_secs()
            ),
            Mode::Search => format!("Buscar: {}", self.search_query),
            Mode::Filter => format!("Filtrar por nombre: {}", self.filter_query),
        };

        let header = Header::new(&header_text);
        f.render_widget(header.render(), chunks[0]);

        // Process table
        self.process_table.render(f, chunks[1]);

        // Footer con ayuda contextual
        let footer_text = match self.mode {
            Mode::Normal => format!(
                "q:Salir | r:Actualizar | /:Buscar | f:Filtrar | c:Limpiar | ↑↓:Navegar | Última: {}s",
                self.last_update.elapsed().as_secs()
            ),
            Mode::Search => "Enter:Aceptar | Esc:Cancelar | Escribe para buscar".to_string(),
            Mode::Filter => "Enter:Aceptar | Esc:Cancelar | Escribe para filtrar".to_string(),
        };

        let footer = Footer::new(&footer_text);
        f.render_widget(footer.render(), chunks[2]);

        // Si estamos en modo búsqueda, mostrar un cursor
        if matches!(self.mode, Mode::Search | Mode::Filter) {
            let search_area = Rect {
                x: chunks[0].x + 8 + self.search_query.len() as u16,
                y: chunks[0].y + 1,
                width: 1,
                height: 1,
            };
            f.render_widget(
                Paragraph::new("|")
                    .style(Style::default().fg(Color::Yellow)),
                search_area
            );
        }
    }
    
    pub fn handle_key(&mut self, key: event::KeyEvent) {
        match self.mode {
            Mode::Normal => self.handle_normal_mode(key),
            Mode::Search => self.handle_search_mode(key),
            Mode::Filter => self.handle_filter_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('r') => self.refresh_processes(),
            KeyCode::Char('/') => {
                self.mode = Mode::Search;
                self.search_query.clear();
            },
            KeyCode::Char('f') => {
                self.mode = Mode::Filter;
                self.filter_query.clear();
            },
            KeyCode::Char('c') => self.clear_search(),
            KeyCode::Char('+') => {
                self.update_interval += Duration::from_secs(1);
            },
            KeyCode::Char('-') => {
                if self.update_interval > Duration::from_secs(1) {
                    self.update_interval -= Duration::from_secs(1);
                }
            },
            KeyCode::Down => self.process_table.select_down(),
            KeyCode::Up => self.process_table.select_up(),
            _ => {}
        }
    }

    fn handle_search_mode(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                let query = self.search_query.clone();
                self.search_processes(&query);
                self.mode = Mode::Normal;
            },
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.search_query.clear();
            },
            KeyCode::Backspace => {
                self.search_query.pop();
                // Búsqueda en tiempo real mientras se escribe
                let query = self.search_query.clone();
                if !self.search_query.is_empty() {
                    self.search_processes(&query);
                } else {
                    self.clear_search();
                }
            },
            KeyCode::Char(c) => {
                self.search_query.push(c);
                // Búsqueda en tiempo real
                let query = self.search_query.clone();
                self.search_processes(&query);
            },
            _ => {}
        }
    }

    fn handle_filter_mode(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                let query = self.filter_query.clone();
                self.filter_processes(1, &query); // Filtrar por nombre por defecto
                self.mode = Mode::Normal;
            },
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.filter_query.clear();
            },
            KeyCode::Backspace => {
                self.filter_query.pop();
                // Filtrado en tiempo real
                let query = self.filter_query.clone();
                if !self.filter_query.is_empty() {
                    self.filter_processes(1, &query);
                } else {
                    self.clear_search();
                }
            },
            KeyCode::Char(c) => {
                self.filter_query.push(c);
                // Filtrado en tiempo real
                let query = self.filter_query.clone();
                self.filter_processes(1, &query);
            },
            _ => {}
        }
    }

    // Búsqueda por campo específico
    pub fn search_by_field(&mut self, field: usize, query: &str) {
        let query_lower = query.to_lowercase();
        
        let filtered: Vec<ProcessRow> = self.all_processes
            .iter()
            .filter(|process| {
                match field {
                    0 => process.pid.to_string().to_lowercase().contains(&query_lower),
                    1 => process.name.to_lowercase().contains(&query_lower),
                    2 => format!("{:.2}", process.cpu_usage).to_lowercase().contains(&query_lower),
                    3 => format!("{:.2}", process.memory_mb).to_lowercase().contains(&query_lower),
                    4 => process.user.to_lowercase().contains(&query_lower),
                    5 => process.state.to_lowercase().contains(&query_lower),
                    6 => format!("{:.2}", process.read_disk_usage).to_lowercase().contains(&query_lower),
                    7 => format!("{:.2}", process.write_disk_usage).to_lowercase().contains(&query_lower),
                    8 => process.start_time_human.to_lowercase().contains(&query_lower),
                    9 => process.parent_pid.map_or(false, |pp| pp.to_string().to_lowercase().contains(&query_lower)),
                    10 => format!("{:.2}", process.virtual_memory_mb).to_lowercase().contains(&query_lower),
                    _ => false,
                }
            })
            .cloned()
            .collect();

        // Convertir los ProcessInfo filtrados a Rows
        self.process_table.data_rows = Self::processes_to_rows(&filtered);
        self.process_count = filtered.len();
    }

    // Búsqueda por PID
    pub fn search_by_pid(&mut self, pid: u32) {
        self.search_by_field(0, &pid.to_string());
    }

    // Búsqueda por usuario
    pub fn search_by_user(&mut self, user: &str) { 
        self.search_by_field(4, user); // Campo 4 es usuario
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}