# Claude Switcher

A command-line tool for managing multiple Claude API provider configurations. Easily switch between different Claude API endpoints and models with a simple interactive interface.

## Overview

Claude Switcher allows you to maintain multiple Claude API configurations and switch between them seamlessly. It's particularly useful for developers who work with different Claude API providers (like Claude Code, DeepSeek, Minimax, etc.) and need to switch configurations quickly.

## Features

- **Interactive Provider Selection**: Choose from available providers with a clean CLI interface
- **Current Settings Display**: Shows your current Claude settings before switching
- **Safe File Operations**: Asks for confirmation before overwriting existing settings
- **Automatic Directory Creation**: Creates necessary directories if they don't exist
- **Error Handling**: Graceful handling of missing files and malformed JSON

## Installation

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd claude_switcher

# Build the project
cargo build --release

# The binary will be available at target/release/claude_switcher
```

### Installation Methods

#### Method 1: Copy Binary to PATH

```bash
# Copy to a directory in your PATH
cp target/release/claude_switcher /usr/local/bin/
# or
cp target/release/claude_switcher ~/.local/bin/claude_switcher
# Or on Windows, copy to a directory in your PATH
```

#### Method 2: Cargo Install (Development)

```bash
# Install directly from local source
cargo install --path .
```

## Usage

### Setup

1. Create the switcher configuration directory:
   ```bash
   mkdir ~/.claude_switcher
   ```

2. Add provider configuration files in the format `settings_<provider_name>.json`:
   ```bash
   # Example: Create a DeepSeek provider configuration
   echo '{
     "env": {
       "ANTHROPIC_BASE_URL": "https://api.deepseek.com/anthropic",
       "ANTHROPIC_MODEL": "deepseek-chat"
     }
   }' > ~/.claude_switcher/settings_deepseek.json

   # Example: Create a Minimax provider configuration
   echo '{
     "env": {
       "ANTHROPIC_BASE_URL": "https://api.minimax.chat/v1",
       "ANTHROPIC_MODEL": "abab5.5-chat"
     }
   }' > ~/.claude_switcher/settings_minimax.json
   ```

### Running the Switcher

```bash
claude_switcher
// or

claude_switcher --provider minimax -y --dir .san_claude
```

The tool will:
1. Display your current Claude settings
2. Show available providers from your `~/.claude_switcher/` directory
3. Prompt you to select a provider
4. Ask for confirmation before overwriting existing settings
5. Copy the selected provider configuration to `~/.claude/settings.json`

### Example Session

```bash
$ claude_switcher
Current Claude Settings:
- ANTHROPIC_BASE_URL: https://api.deepseek.com/anthropic
- ANTHROPIC_MODEL: deepseek-chat

Please choose the provider
❯ deepseek
  minimax_api
  minimax_plan

The .claude/settings file already exists. Are you sure you want to overwrite it? [Y/n] Y
Switch successful! Current provider: minimax_api
```

## Configuration File Format

Provider configuration files should be JSON files with the following structure:

```json
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://api.example.com/anthropic",
    "ANTHROPIC_MODEL": "model-name"
  }
}
```

## Development

### Project Structure

```
claude_switcher/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml          # Rust dependencies and package info
├── Cargo.lock          # Dependency lock file
└── README.md           # This file
```

### Building for Development

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run directly
cargo run
```

### Dependencies

The project uses the following Rust crates:

- **dialoguer**: Interactive command-line prompts
- **anyhow**: Error handling
- **serde_json**: JSON parsing and serialization
- **dirs**: Cross-platform directory handling

### Code Overview

- **`main()`**: Entry point that orchestrates the switching process
- **`discover_providers()`**: Scans the switcher directory for provider configurations
- **`display_current_claude_settings()`**: Reads and displays current Claude settings

### Testing

To test the application:

1. Create test provider configurations in `~/.claude_switcher/`
2. Run the application and verify it correctly:
   - Discovers available providers
   - Displays current settings
   - Successfully switches configurations

## Troubleshooting

### Common Issues

1. **"Claude switcher directory not found"**:
   - Create the directory: `mkdir ~/.claude_switcher`

2. **"No provider settings found"**:
   - Add provider configuration files in the correct format
   - Ensure files are named `settings_<provider_name>.json`

3. **JSON parsing errors**:
   - Verify your JSON files are valid
   - Check for trailing commas and proper syntax

4. **Permission errors**:
   - Ensure you have write permissions to `~/.claude/` directory

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

MIT

## Acknowledgments

- Built with Rust for performance and reliability
- Uses excellent Rust crates from the ecosystem
- Inspired by the need for flexible Claude API configuration management