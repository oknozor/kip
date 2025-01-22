default:
    @just --list

# Build all packages in release mode
build:
    cargo build --release

# Install the binaries to ~/.local/bin
install: build
    cargo install --path crates/homedd --force
    cargo install --path crates/homed-cli --force

# Create configuration directory and copy example config
setup-config:
    #!/usr/bin/env bash
    mkdir -p ~/.local/share/homedd
    mkdir -p ~/.config/homedd

    if [ ! -f ~/.config/homedd/config.toml ]; then
        cp config.example.toml ~/.config/homedd/config.toml
        echo "Created config.toml from template. Please edit ~/.config/homedd/config.toml with your credentials"
    fi

# Install systemd user unit
setup-systemd:
    #!/usr/bin/env bash
    mkdir -p ~/.config/systemd/user
    cp homedd.service ~/.config/systemd/user/
    systemctl --user daemon-reload
    echo "Systemd user unit installed. Enable with: systemctl --user enable --now homedd"

# Complete installation
install-all: build install setup-config setup-systemd
    #!/usr/bin/env bash
    echo "Installation complete!"
    echo "Next steps:"
    echo "1. Edit ~/.config/homedd/config.toml with your credentials"
    echo "2. Start the service: systemctl --user enable --now homedd"
    echo "3. Check status: systemctl --user status homedd"
    echo "4. View logs: journalctl --user -u homedd -f"

# Check service status
status:
    systemctl --user status homedd

# View service logs
logs:
    journalctl --user -u homedd -f

# Stop the service
stop:
    systemctl --user stop homedd

# Start the service
start:
    systemctl --user start homedd

# Restart the service
restart:
    systemctl --user restart homedd

# Uninstall homedd
uninstall:
    #!/usr/bin/env bash
    systemctl --user stop homedd || true
    systemctl --user disable homedd || true
    rm ~/.config/systemd/user/homedd.service || true
    cargo uninstall homedd || true
    cargo uninstall homed-cli || true
    systemctl --user daemon-reload
    echo "Uninstall complete. Configuration at ~/.config/homedd remains."

# Clean all build artifacts
clean:
    cargo clean

# Run tests
test:
    cargo test --all
