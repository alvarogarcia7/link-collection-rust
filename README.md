# Link Collection (lc)

A Rust CLI application for managing and organizing link collections.

## Configuration

The application supports flexible configuration through multiple sources with hierarchical overrides.

### Configuration Sources (in order of precedence)

1. **Command-line arguments** (highest priority)
   - `--database-file <path>` - Path to the database file
   - `--log-level <level>` - Logging level
   - `--template-file <path>` - Path to template directory
   - `--hacker-news-url <url>` - HackerNews API URL
   - `--config-file <path>` - Custom configuration file path

2. **Environment variables**
   - `LC_CONFIG_FILE` - Path to configuration file
   - `LC_DATABASE_FILE` - Database file path
   - `LC_LOG_LEVEL` - Logging level
   - `LC_TEMPLATE_FILE` - Template directory path
   - `LC_HACKER_NEWS_URL` - HackerNews API URL

3. **.env.local file** (in current directory)
   - Project-local configuration overrides
   - Key=value format with `LC_*` prefixes
   - Example: `LC_LOG_LEVEL=debug`

4. **~/.config/lc/config.toml** (default user config)
   - User's home directory configuration
   - TOML format
   - Applied to all uses of the application

5. **Built-in defaults** (lowest priority)
   - `database_file`: `./data/database/links.rec`
   - `log_level`: `info`
   - `template_file`: `./data/template/`
   - `hacker_news_url`: `http://0.0.0.0:8181`

### Configuration File Format (TOML)

Create `~/.config/lc/config.toml`:

```toml
database_file = "./data/database/links.rec"
log_level = "info"
template_file = "./data/template/"
hacker_news_url = "https://hacker-news.firebaseio.com"
```

### Log Levels

Valid log levels (in order of verbosity):
- `trace` - Very detailed logging
- `debug` - Debug-level messages
- `info` - Informational messages (default)
- `warn` - Warning messages
- `error` - Error messages only
- `off` - No logging

### Environment Variables Example

Create `.env.local` in your project directory:

```
LC_LOG_LEVEL=debug
LC_DATABASE_FILE=/custom/path/links.rec
```

### Command-line Examples

Override just the log level:
```bash
lc --log-level debug list
```

Use a custom config file:
```bash
lc --config-file ~/custom-config.toml list
```

Set the database path via environment variable:
```bash
LC_DATABASE_FILE=/custom/links.rec lc list
```

## Usage

### List records
```bash
lc list
lc ls  # alias
```

### Add a new record
```bash
lc new-record
lc n  # alias
```

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## Sample Configuration Files

- `config.toml.sample` - Example TOML configuration file
- `.env.local.sample` - Example environment variables file

Copy and modify these files to customize your setup.
