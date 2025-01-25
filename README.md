# Kip - Your Personal Dashboard CLI

Kip is a modular command-line dashboard that aggregates and displays information from various sources like GitHub issues, calendars, and more. It consists of a daemon (kipd) that collects data in the background and a CLI (kip) to query this information. All data fetched by plugins is stored locally in a fast embedded database, ensuring instantaneous access to your information without network latency.

## Features

- 📅 Calendar integration with CalDAV servers (Nextcloud, etc.)
- 🐙 GitHub integration for issues and pull requests
- 🔔 Desktop notifications for new items
- 🔌 Pluggable architecture for easy extension
- 🚀 Fast and lightweight
- 💾 Zero-latency local data access

## Installation

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (rustc, cargo)
- [Just command runner](https://github.com/casey/just)

### Quick Install

```bash
# Clone the repository
git clone https://github.com/oknozor/kip.git
cd kip

# Install everything (binaries, config, and systemd service)
just install-all
```

### Manual Configuration

1. Edit your configuration file at `~/.config/kipd/config.toml`:

```toml
[plugins.github]
name = "github"
command = "kip-gh"
args = ["--token", "your-github-token", "pr-opened"]
interval = 300
timeout = 30
notify = true

[plugins.calendar]
name = "calendar"
command = "kip-agenda"
args = [
    "--url", "https://your-nextcloud-instance",
    "--username", "your-username",
    "--password", "your-password"
]
interval = 300
timeout = 30
notify = true
```

2. Start the service:
```bash
systemctl --user enable --now kipd
```

## Usage

Query your dashboard using the `kip` command:

```bash
# Get calendar events
kip get calendar

# Get GitHub issues
kip get github

# Get pull requests
kip get github_prs
```

## Writing Plugins

Kip's plugin system is language-agnostic - you can write plugins in any programming language! A plugin is simply an executable that outputs a specific JSON format to stdout.

### Plugin Output Format

Plugins must output a JSON array of `Item` objects with the following structure:

```json
[
  {
    "title": "Required: Item title",
    "url": "Required: Any URL",
    "custom": {
      "optional_field1": "value1",
      "optional_field2": "value2"
    }
  }
]
```

Required fields:
- `title`: String - The main title of the item
- `url`: String - A URL associated with the item

Additional fields can be added as needed in the `custom` object.

### Example Plugin in Python

Here's a simple weather plugin example:

```python
#!/usr/bin/env python3
import json
import requests

def get_weather():
    # Example using OpenWeatherMap API
    API_KEY = "your_api_key"
    city = "Paris"
    url = f"http://api.openweathermap.org/data/2.5/weather?q={city}&appid={API_KEY}&units=metric"

    response = requests.get(url)
    data = response.json()

    return [{
        "title": f"Weather in {city}: {data['weather'][0]['description']}",
        "url": f"https://openweathermap.org/city/{data['id']}",
        "custom": {
            "temperature": f"{data['main']['temp']}°C",
            "humidity": f"{data['main']['humidity']}%",
            "wind_speed": f"{data['wind']['speed']} m/s"
        }
    }]

if __name__ == "__main__":
    print(json.dumps(get_weather()))
```

### Configuring Your Plugin

Add your plugin to `~/.config/kipd/config.toml`:

```toml
[plugins.weather]
name = "weather"
command = "/path/to/weather.py"
args = []
interval = 1800  # Run every 30 minutes
timeout = 30
notify = true
```

### Plugin Development Tips

1. **Error Handling**: Your plugin should handle errors gracefully and return an empty array `[]` if data can't be fetched.
2. **Performance**: Keep the execution time reasonable as plugins are run periodically.
3. **Rate Limiting**: If your plugin calls external APIs, respect their rate limits.
4. **Testing**: You can test your plugin output directly in the terminal:
   ```bash
   ./your_plugin.py | jq
   ```

## Development

### Project Structure

- `kipd`: The main daemon that manages plugins and serves data
- `kip`: CLI tool to query the daemon
- `kip-agenda`: Calendar integration plugin
- `kip-gh`: GitHub integration plugin
- `kip-storage`: Shared storage functionality
- `kip-plugin`: Plugin trait and utilities

### Building

```bash
# Build all components
just build

# Run tests
just test

# Clean build artifacts
just clean
```

## Uninstalling

```bash
just uninstall
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License
