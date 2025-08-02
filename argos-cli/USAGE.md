# Argos CLI - Guía de Uso

## Comandos Básicos

### Monitorear un proceso específico

```bash
# Mostrar información básica
argos monitor --pid 1234

# Mostrar en formato JSON
argos monitor --pid 1234 --format json

# Guardar en base de datos
argos monitor --pid 1234 --save

# Combinar opciones
argos monitor --pid 1234 --format json --save
```

### Muestreo de procesos

```bash
# Muestreo básico (10 iteraciones, 500ms)
argos sample --pid 1234

# Muestreo personalizado
argos sample --pid 1234 --iterations 20 --interval-ms 1000

# Guardar resultados en archivo
argos sample --pid 1234 --output samples.csv --format csv

# Muestreo en JSON
argos sample --pid 1234 --format json --output results.json
```

### Listar procesos (Futuro)

```bash
# Listar todos los procesos
argos list

# Filtrar por nombre
argos list --name "chrome"

# Filtrar por usuario
argos list --user "admin"

# Ordenar por uso de CPU
argos list --sort-by cpu

# Formato JSON
argos list --format json
```

### Historial de procesos (Futuro)

```bash
# Ver historial completo
argos history

# Historial de un proceso específico
argos history --pid 1234

# Limitar resultados
argos history --limit 25
```

### Configuración

```bash
# Ver configuración actual
argos config show

# Establecer formato por defecto
argos config set default_format json

# Habilitar auto-guardado
argos config set auto_save true

# Configurar intervalo por defecto
argos config set default_interval_ms 1000

# Resetear configuración
argos config reset
```

## Formatos de Salida

### Text (Por defecto)

Salida formateada y legible para humanos con emojis y tablas.

### JSON

Salida estructurada ideal para scripting y APIs:

```json
{
  "name": "chrome.exe",
  "pid": 1234,
  "state": "Running",
  "cpu_usage": 15.5,
  "memory_mb": 256.8,
  "start_time": "2025-07-31T10:30:00",
  "parent_pid": 456,
  "timestamp": "2025-07-31T15:45:30Z"
}
```

### CSV

Formato ideal para análisis en Excel o scripts:

```csv
name,pid,state,cpu_usage,memory_mb,start_time,parent_pid
chrome.exe,1234,Running,15.50,256.80,2025-07-31T10:30:00,456
```

## Ejemplos de Integración

### Automatización con PowerShell

```powershell
# Monitorear proceso y guardar como JSON
$result = argos monitor --pid 1234 --format json | ConvertFrom-Json
Write-Host "Proceso $($result.name) usa $($result.cpu_usage)% CPU"

# Muestreo y análisis
argos sample --pid 1234 --format csv --output "sample_$(Get-Date -Format 'yyyyMMdd_HHmm').csv"
```

### Integración con scripts

```bash
#!/bin/bash
# Monitorear proceso crítico
PID=$(pgrep myapp)
if [ ! -z "$PID" ]; then
    argos monitor --pid $PID --save
    argos sample --pid $PID --iterations 5 --output "/logs/myapp_$(date +%Y%m%d).csv"
fi
```

## Configuración Avanzada

El archivo de configuración se guarda en:

- Windows: `%APPDATA%\argos\config.toml`
- Linux/macOS: `~/.config/argos/config.toml`

Ejemplo de configuración:

```toml
default_format = "json"
default_interval_ms = 1000
default_iterations = 20
auto_save = true
database_url = "argos.db"
log_level = "info"
```
