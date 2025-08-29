// app.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchField {
    Pid,
    Name,
    Cpu,
    Memory,
    User,
    State,
    ReadDisk,
    WriteDisk,
    StartTime,
    ParentPid,
    VirtualMemory,
}

impl SearchField {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchField::Pid => "PID",
            SearchField::Name => "Nombre",
            SearchField::Cpu => "CPU%",
            SearchField::Memory => "Memoria",
            SearchField::User => "Usuario",
            SearchField::State => "Estado",
            SearchField::ReadDisk => "Lectura Disco",
            SearchField::WriteDisk => "Escritura Disco",
            SearchField::StartTime => "Inicio",
            SearchField::ParentPid => "PID Padre",
            SearchField::VirtualMemory => "Memoria Virtual",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            SearchField::Pid => 0,
            SearchField::Name => 1,
            SearchField::Cpu => 2,
            SearchField::Memory => 3,
            SearchField::User => 4,
            SearchField::State => 5,
            SearchField::ReadDisk => 6,
            SearchField::WriteDisk => 7,
            SearchField::StartTime => 8,
            SearchField::ParentPid => 9,
            SearchField::VirtualMemory => 10,
        }
    }

    pub fn all_fields() -> Vec<SearchField> {
        vec![
            SearchField::Pid,
            SearchField::Name,
            SearchField::Cpu,
            SearchField::Memory,
            SearchField::User,
            SearchField::State,
            SearchField::ReadDisk,
            SearchField::WriteDisk,
            SearchField::StartTime,
            SearchField::ParentPid,
            SearchField::VirtualMemory,
        ]
    }

    pub fn next(&self) -> SearchField {
        let fields = Self::all_fields();
        let current_index = fields.iter().position(|f| f == self).unwrap_or(0);
        fields[(current_index + 1) % fields.len()]
    }

    pub fn previous(&self) -> SearchField {
        let fields = Self::all_fields();
        let current_index = fields.iter().position(|f| f == self).unwrap_or(0);
        fields[(current_index + fields.len() - 1) % fields.len()]
    }
}