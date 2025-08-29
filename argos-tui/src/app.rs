use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Instant, Duration};
use crossterm::event::{self, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Row};
use ratatui::style::{Style, Modifier};
use crate::widgets::process_table::ProcessTable;
use crate::widgets::header::Header;
use crate::widgets::footer::Footer;
use argos_core::commands::list::list_processes;
use argos_core::process::model::ProcessRow;
use crate::stats::{ProcessStats, StatisticalMetrics};
use std::collections::{HashMap, VecDeque};

pub enum Mode {
    Normal,
    Search,
    Filter,
    ProcessDetail(u32),
}

pub struct App {
    pub should_quit: bool,
    pub process_table: ProcessTable<'static>,
    pub last_update: Instant,
    pub update_interval: Duration,
    pub process_count: usize,
    pub processes_data: Arc<Mutex<Option<(Vec<ProcessRow>, usize)>>>,
    pub all_processes: Vec<ProcessRow>,
    pub filtered_processes: Vec<ProcessRow>, // New field to track filtered processes
    pub mode: Mode,
    pub process_stats: HashMap<u32, ProcessStats>,
    pub selected_pid: Option<u32>,
    pub stats_update_interval: Duration,
    pub last_stats_update: Instant,
    pub search_query: String,
    pub filter_query: String,
    pub search_history: VecDeque<String>,
    pub filter_history: VecDeque<String>,
}

impl App {
    pub fn new() -> Self {
        let (header, initial_data, widths, count) = Self::load_process_data();
        let all_processes = list_processes().unwrap_or_else(|_| vec![]);
        
        let processes_data = Arc::new(Mutex::new(None));
        
        Self::start_background_updater(Arc::clone(&processes_data));
        
        Self {
            should_quit: false,
            process_table: ProcessTable::new(header, initial_data.clone(), widths),
            last_update: Instant::now(),
            update_interval: Duration::from_secs(2),
            process_count: count,
            processes_data,
            all_processes: all_processes.clone(),
            filtered_processes: all_processes, // Initialize with all processes
            mode: Mode::Normal,
            process_stats: HashMap::new(),
            selected_pid: None,
            last_stats_update: Instant::now(),
            stats_update_interval: Duration::from_secs(1),
            search_query: String::new(),
            filter_query: String::new(),
            search_history: VecDeque::with_capacity(10),
            filter_history: VecDeque::with_capacity(10),
        }
    }

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

    fn processes_to_rows(processes: &[ProcessRow]) -> Vec<Row<'static>> {
        processes.iter().map(Self::process_to_row).collect()
    }

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
    
    pub fn search_processes(&mut self, query: &str) {
        let (filtered_rows, filtered_processes) = if query.is_empty() {
            (
                Self::processes_to_rows(&self.all_processes),
                self.all_processes.clone(),
            )
        } else {
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
            
            (
                Self::processes_to_rows(&filtered),
                filtered,
            )
        };

        self.process_table.data_rows = filtered_rows;
        self.filtered_processes = filtered_processes;
        self.process_count = self.process_table.data_rows.len();
        
        if !query.is_empty() && !self.search_history.contains(&query.to_string()) {
            if self.search_history.len() >= 10 {
                self.search_history.pop_back();
            }
            self.search_history.push_front(query.to_string());
        }
    }

    pub fn filter_processes(&mut self, field: usize, query: &str) {
        let (filtered_rows, filtered_processes) = if query.is_empty() {
            (
                Self::processes_to_rows(&self.all_processes),
                self.all_processes.clone(),
            )
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
            
            (
                Self::processes_to_rows(&filtered),
                filtered,
            )
        };

        self.process_table.data_rows = filtered_rows;
        self.filtered_processes = filtered_processes;
        self.process_count = self.process_table.data_rows.len();
        
        let history_entry = format!("{}:{}", field, query);
        if !self.filter_history.contains(&history_entry) {
            if self.filter_history.len() >= 10 {
                self.filter_history.pop_back();
            }
            self.filter_history.push_front(history_entry);
        }
    }

    pub fn clear_search(&mut self) {
        let all_rows = Self::processes_to_rows(&self.all_processes);
        self.process_table.data_rows = all_rows;
        self.filtered_processes = self.all_processes.clone();
        self.process_count = self.all_processes.len();
        self.search_query.clear();
        self.filter_query.clear();
        self.mode = Mode::Normal;
    }

    pub fn refresh_processes(&mut self) {
        let mut data = self.processes_data.lock().unwrap();
        if let Some((new_process_data, new_count)) = data.take() {
            self.all_processes = new_process_data;
            
            let (filtered_rows, filtered_processes) = if !self.search_query.is_empty() {
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
                
                (
                    Self::processes_to_rows(&filtered),
                    filtered,
                )
            } else if !self.filter_query.is_empty() {
                let query = self.filter_query.clone();
                let query_lower = query.to_lowercase();
                let filtered: Vec<ProcessRow> = self.all_processes
                    .iter()
                    .filter(|process| {
                        process.name.to_lowercase().contains(&query_lower)
                    })
                    .cloned()
                    .collect();
                
                (
                    Self::processes_to_rows(&filtered),
                    filtered,
                )
            } else {
                (
                    Self::processes_to_rows(&self.all_processes),
                    self.all_processes.clone(),
                )
            };

            self.process_table.data_rows = filtered_rows;
            self.filtered_processes = filtered_processes;
            self.process_count = self.process_table.data_rows.len();
            
            self.last_update = Instant::now();
            
            // Reselect the previously selected PID if it exists
            if let Some(prev_pid) = self.selected_pid {
                let new_selection = self.filtered_processes.iter().position(|process| process.pid == prev_pid);
                self.process_table.state.select(new_selection);
            } else if !self.process_table.data_rows.is_empty() {
                self.process_table.state.select(Some(0));
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

        self.update_process_stats();

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
            Mode::ProcessDetail(pid) => {
                if let Some(process) = self.get_process_by_pid(pid) {
                    format!("Detalles del Proceso - PID: {} | Nombre: {}", pid, process.name)
                } else {
                    format!("Detalles del Proceso - PID: {} | Nombre: Desconocido", pid)
                }
            }
        };

        let header = Header::new(&header_text);
        f.render_widget(header.render(), chunks[0]);

        match self.mode {
            Mode::ProcessDetail(pid) => {
                self.draw_process_details(f, chunks[1], pid);
            }
            _ => {
                self.process_table.render(f, chunks[1]);
            }
        }

        let footer_text = match self.mode {
            Mode::Normal => format!(
                "q:Salir | r:Actualizar | /:Buscar | f:Filtrar | c:Limpiar | ↑↓:Navegar | Última: {}s",
                self.last_update.elapsed().as_secs()
            ),
            Mode::Search => "Enter:Aceptar | Esc:Cancelar | Escribe para buscar".to_string(),
            Mode::Filter => "Enter:Aceptar | Esc:Cancelar | Escribe para filtrar".to_string(),
            Mode::ProcessDetail(_) => "Esc/q:Volver | r:Actualizar estadísticas".to_string(),
        };

        let footer = Footer::new(&footer_text);
        f.render_widget(footer.render(), chunks[2]);

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
            Mode::ProcessDetail(pid) => self.handle_detail_mode(key, pid),
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
            KeyCode::Enter => {
                if let Some(selected) = self.process_table.state.selected() {
                    if let Some(process) = self.filtered_processes.get(selected) {
                        self.show_process_details(process.pid);
                    }
                }
            },
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
                let query = self.search_query.clone();
                if !self.search_query.is_empty() {
                    self.search_processes(&query);
                } else {
                    self.clear_search();
                }
            },
            KeyCode::Char(c) => {
                self.search_query.push(c);
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
                self.filter_processes(1, &query);
                self.mode = Mode::Normal;
            },
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.filter_query.clear();
            },
            KeyCode::Backspace => {
                self.filter_query.pop();
                let query = self.filter_query.clone();
                if !self.filter_query.is_empty() {
                    self.filter_processes(1, &query);
                } else {
                    self.clear_search();
                }
            },
            KeyCode::Char(c) => {
                self.filter_query.push(c);
                let query = self.filter_query.clone();
                self.filter_processes(1, &query);
            },
            _ => {}
        }
    }

    fn handle_detail_mode(&mut self, key: event::KeyEvent, pid: u32) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.mode = Mode::Normal;
            },
            KeyCode::Char('r') => {
                let process = self.get_process_by_pid(pid).cloned();
                if let Some(stats) = self.process_stats.get_mut(&pid) {
                    if let Some(process) = process {
                        stats.add_sample(
                            process.cpu_usage,
                            process.memory_mb,
                            process.read_disk_usage,
                            process.write_disk_usage,
                        );
                    }
                }
            },
            _ => {}
        }
    }

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

        let filtered_len = filtered.len();
        self.process_table.data_rows = Self::processes_to_rows(&filtered);
        self.filtered_processes = filtered;
        self.process_count = filtered_len;
    }

    pub fn search_by_pid(&mut self, pid: u32) {
        self.search_by_field(0, &pid.to_string());
    }

    pub fn search_by_user(&mut self, user: &str) {
        self.search_by_field(4, user);
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn update_process_stats(&mut self) {
        if Instant::now().duration_since(self.last_stats_update) < self.stats_update_interval {
            return;
        }

        let to_remove: Vec<u32> = self.process_stats
            .keys()
            .filter(|pid| self.get_process_by_pid(**pid).is_none())
            .copied()
            .collect();

        for pid in to_remove {
            self.process_stats.remove(&pid);
        }

        let process_data: Vec<(u32, Option<ProcessRow>)> = self.process_stats
            .keys()
            .map(|pid| (*pid, self.get_process_by_pid(*pid).cloned()))
            .collect();

        for (pid, maybe_process) in process_data {
            if let Some(process) = maybe_process {
                if let Some(stats) = self.process_stats.get_mut(&pid) {
                    stats.add_sample(
                        process.cpu_usage,
                        process.memory_mb,
                        process.read_disk_usage,
                        process.write_disk_usage,
                    );
                }
            }
        }

        self.last_stats_update = Instant::now();
    }

    pub fn get_process_by_pid(&self, pid: u32) -> Option<&ProcessRow> {
        self.all_processes.iter().find(|p| p.pid == pid)
    }

    pub fn show_process_details(&mut self, pid: u32) {
        self.selected_pid = Some(pid);
        self.mode = Mode::ProcessDetail(pid);
        
        if !self.process_stats.contains_key(&pid) {
            if let Some(process) = self.get_process_by_pid(pid) {
                let mut stats = ProcessStats::new(pid, process.name.clone(), 60);
                stats.add_sample(
                    process.cpu_usage,
                    process.memory_mb,
                    process.read_disk_usage,
                    process.write_disk_usage,
                );
                self.process_stats.insert(pid, stats);
            }
        }
    }

    pub fn draw_process_details(&self, f: &mut Frame, area: Rect, pid: u32) {
        if let Some(stats) = self.process_stats.get(&pid) {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(9),  // CPU Metrics
                    Constraint::Length(9),  // Memory Metrics
                    Constraint::Length(9),  // Disk Metrics
                    Constraint::Length(10), // Statistical Analysis
                ])
                .split(area);
            self.draw_cpu_metrics(f, chunks[0], stats);
            self.draw_memory_metrics(f, chunks[1], stats);
            self.draw_disk_metrics(f, chunks[2], stats);
            self.draw_statistical_analysis(f, chunks[3], stats);
        }
    }

   fn draw_cpu_metrics(&self, f: &mut Frame, area: Rect, stats: &ProcessStats) {
    let metrics = stats.get_cpu_metrics();
    let trend_emoji = if metrics.trend > 0.1 { "📈↗️" } else if metrics.trend < -0.1 { "📉↘️" } else { "➡️" };
    let text = format!(
        "CPU:           {:>6.1}% {}\n\
         Media:        {:>6.1}%\n\
         Mediana:      {:>6.1}%\n\
         σ:            {:>6.1}%\n\
         Mín:          {:>6.1}%\n\
         Máx:          {:>6.1}%\n\
         Percentil 95: {:>6.1}%\n\
         Tendencia:    {:>6.3}/min\n\
         Predicción:   {:>6.1}%",
        stats.cpu_history.back().unwrap_or(&0.0),
        trend_emoji,
        metrics.mean,
        metrics.median,
        metrics.std_dev,
        metrics.min,
        metrics.max,
        metrics.percentile_95.unwrap_or(0.0),
        metrics.trend * 60.0,
        metrics.forecast
    );

    let block = Block::default()
        .title("Estadísticas de CPU")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_memory_metrics(&self, f: &mut Frame, area: Rect, stats: &ProcessStats) {
    let metrics = stats.get_memory_metrics();
    let trend_emoji = if metrics.trend > 0.1 { "📈↗️" } else if metrics.trend < -0.1 { "📉↘️" } else { "➡️" };
    let text = format!(
        "Memoria:       {:>6.1} MB {}\n\
         Media:        {:>6.1} MB\n\
         Mediana:      {:>6.1} MB\n\
         σ:            {:>6.1} MB\n\
         Mín:          {:>6.1} MB\n\
         Máx:          {:>6.1} MB\n\
         Percentil 95: {:>6.1} MB\n\
         Tendencia:    {:>6.3} MB/min\n\
         Predicción:   {:>6.1} MB",
        stats.memory_history.back().unwrap_or(&0.0),
        trend_emoji,
        metrics.mean,
        metrics.median,
        metrics.std_dev,
        metrics.min,
        metrics.max,
        metrics.percentile_95.unwrap_or(0.0),
        metrics.trend * 60.0,
        metrics.forecast
    );

    let block = Block::default()
        .title("Estadísticas de Memoria")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green));

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_disk_metrics(&self, f: &mut Frame, area: Rect, stats: &ProcessStats) {
    let read_metrics = stats.get_disk_read_metrics();
    let write_metrics = stats.get_disk_write_metrics();
    let read_trend_emoji = if read_metrics.trend > 0.1 { "↗️" } else if read_metrics.trend < -0.1 { "↘️" } else { "➡️" };
    let write_trend_emoji = if write_metrics.trend > 0.1 { "↗️" } else if write_metrics.trend < -0.1 { "↘️" } else { "➡️" };
    let text = format!(
        "Lectura:      {:>6.1} MB/s {}\n\
         Escritura:    {:>6.1} MB/s {}\n\
         σ Lectura:    {:>6.1}\n\
         σ Escritura:  {:>6.1}\n\
         Tendencia L:  {:>6.3} MB/s/min\n\
         Tendencia E:  {:>6.3} MB/s/min\n\
         P95 L:        {:>6.1}\n\
         P95 E:        {:>6.1}",
        stats.disk_read_history.back().unwrap_or(&0.0),
        read_trend_emoji,
        stats.disk_write_history.back().unwrap_or(&0.0),
        write_trend_emoji,
        read_metrics.std_dev,
        write_metrics.std_dev,
        read_metrics.trend * 60.0,
        write_metrics.trend * 60.0,
        read_metrics.percentile_95.unwrap_or(0.0),
        write_metrics.percentile_95.unwrap_or(0.0)
    );

    let block = Block::default()
        .title("Estadísticas de Disco")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue));

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_statistical_analysis(&self, f: &mut Frame, area: Rect, stats: &ProcessStats) {
    let cpu_metrics = stats.get_cpu_metrics();
    let memory_metrics = stats.get_memory_metrics();
    let read_metrics = stats.get_disk_read_metrics();
    let write_metrics = stats.get_disk_write_metrics();
    let cpu_stability = (1.0 - (cpu_metrics.std_dev / cpu_metrics.mean.max(1.0))) * 100.0;
    let memory_stability = (1.0 - (memory_metrics.std_dev / memory_metrics.mean.max(1.0))) * 100.0;
    let text = format!(
        "Estab. CPU:   {:>6.1}%\n\
         Estab. Mem:   {:>6.1}%\n\
         Coef. Var. CPU: {:>6.3}\n\
         Coef. Var. Mem: {:>6.3}\n\
         Sesgo CPU:    {:>6.3}\n\
         Sesgo Mem:    {:>6.3}\n\
         Coef. Var. L: {:>6.3}\n\
         Coef. Var. E: {:>6.3}\n\
         Ventana Temp: {:>6.1}s\n\
         Muestras:     {:>6}",
        cpu_stability,
        memory_stability,
        cpu_metrics.std_dev / cpu_metrics.mean.max(1.0),
        memory_metrics.std_dev / memory_metrics.mean.max(1.0),
        cpu_metrics.skewness.unwrap_or(0.0),
        memory_metrics.skewness.unwrap_or(0.0),
        read_metrics.std_dev / read_metrics.mean.max(1.0),
        write_metrics.std_dev / write_metrics.mean.max(1.0),
        stats.get_time_window().map(|d| d.as_secs_f32()).unwrap_or(0.0),
        stats.cpu_history.len()
    );

    let block = Block::default()
        .title("Análisis Estadístico Detallado")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}
}