default:
    @just --list

# Build all packages in release mode
build:
    cargo build --release

# Install the binaries to ~/.local/bin
install: build
    cargo install --path crates/kipd --force
    cargo install --path crates/kip --force
    cargo install --path crates/kip-gh --force
    cargo install --path crates/kip-agenda --force

# Create configuration directory and copy example config
setup-config:
    #!/usr/bin/env bash
    mkdir -p ~/.local/share/kipd
    mkdir -p ~/.config/kipd

    if [ ! -f ~/.config/kipd/config.toml ]; then
        cp config.example.toml ~/.config/kipd/config.toml
        echo "Created config.toml from template. Please edit ~/.config/kipd/config.toml with your credentials"
    fi

# Install systemd user unit
setup-systemd:
    #!/usr/bin/env bash
    mkdir -p ~/.config/systemd/user
    cp kipd.service ~/.config/systemd/user/
    systemctl --user daemon-reload
    echo "Systemd user unit installed. Enable with: systemctl --user enable --now kipd"

# Complete installation
install-all: build install setup-config setup-systemd
    #!/usr/bin/env bash
    echo "Installation complete!"
    echo "Next steps:"
    echo "1. Edit ~/.config/kipd/config.toml with your credentials"
    echo "2. Start the service: systemctl --user enable --now kipd"
    echo "3. Check status: systemctl --user status kipd"
    echo "4. View logs: journalctl --user -u kipd -f"

# Check service status
status:
    systemctl --user status kipd

# View service logs
logs:
    journalctl --user -u kipd -f

# Stop the service
stop:
    systemctl --user stop kipd

# Start the service
start:
    systemctl --user start kipd

# Restart the service
restart:
    systemctl --user restart kipd

# Uninstall kipd
uninstall:
    #!/usr/bin/env bash
    systemctl --user stop kipd || true
    systemctl --user disable kipd || true
    rm ~/.config/systemd/user/kipd.service || true
    cargo uninstall kipd || true
    cargo uninstall kip || true
    systemctl --user daemon-reload
    echo "Uninstall complete. Configuration at ~/.config/kipd remains."

# Clean all build artifacts
clean:
    cargo clean

# Run tests
test:
    cargo test --all
