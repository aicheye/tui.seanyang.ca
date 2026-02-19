# ssh.seanyang.me

A personal portfolio and introduction, served over SSH as an interactive TUI.

```bash
ssh ssh.seanyang.me
```

## Deployment

This guide covers hosting the TUI on a home server while keeping normal SSH access to the machine.

The idea: move your real SSH daemon to a non-standard port, then give port 22 to the TUI container so that `ssh ssh.seanyang.me` connects directly to it.

### Prerequisites

- A machine with [Docker](https://docs.docker.com/get-docker/) installed
- An A record for `ssh.seanyang.me` pointing to the machine's public IP
- Port 22 forwarded to the machine on your router

### 1. Move Your SSH Daemon Off Port 22

Edit `/etc/ssh/sshd_config`:

```bash
Port 2200
```

Restart:

```bash
sudo systemctl restart sshd
```

> [!CAUTION]
> **Before closing your current session**, verify you can connect on the new port:
>
> ```bash
> ssh -p 2200 user@your-machine
> ```

To make this convenient, add an entry to `~/.ssh/config` on your local machine:

```bash
Host home
    HostName ssh.seanyang.me
    Port 2200
    User your-username
```

Now `ssh home` connects to your real shell.

### 2. Build the Docker Image

```bash
git clone https://github.com/aicheye/ssh.seanyang.me.git
cd ssh.seanyang.me
docker build -t ssh-seanyang-me .
```

For cross-platform builds (e.g. building for ARM on an x86 host):

```bash
docker buildx build --platform linux/arm64 -t ssh-seanyang-me .
```

### 3. Run the Container

```bash
docker run -d \
  --name ssh-seanyang-me \
  --restart unless-stopped \
  -p 22:2222 \
  -v ssh-host-key:/data \
  ssh-seanyang-me
```

This maps host port `22` → container port `2222`, so visitors running `ssh ssh.seanyang.me` hit the TUI.

### 4. Verify

From another machine:

```bash
# Should open the TUI
ssh ssh.seanyang.me

# Should open your real shell
ssh -p 2200 ssh.seanyang.me
```

---

## Docker Compose

```yaml
services:
  ssh:
    build: .
    ports:
      - "22:2222"
    volumes:
      - ssh-host-key:/data
    environment:
      - RUST_LOG=info
    restart: unless-stopped

volumes:
  ssh-host-key:
```

```bash
docker compose up -d
```

---

## Configuration

All configuration is done via environment variables:

| Variable | Default | Description |
|---|---|---|
| `SSH_ADDR` | `0.0.0.0:2222` | Address and port the SSH server binds to inside the container |
| `SSH_HOST_KEY` | `/data/host_key` | Path to the Ed25519 host key (auto-generated on first run) |
| `RUST_LOG` | `info` | Log level filter (`trace`, `debug`, `info`, `warn`, `error`) |

### Persisting the Host Key

The host key lives at `/data/host_key` inside the container. The volume mount (`-v ssh-host-key:/data`) ensures it survives container restarts.

> [!IMPORTANT]
> If the host key changes, returning visitors will see an SSH host key mismatch warning.

---

## Host Deployment (without Docker)

Download the latest binary from [GitHub Releases](https://github.com/aicheye/ssh.seanyang.me/releases):

```bash
curl -fsSL https://github.com/aicheye/ssh.seanyang.me/releases/download/v0.1.1/ssh-seanyang-me-linux-arm.zip \
  -o ssh-seanyang-me.zip
unzip ssh-seanyang-me.zip
chmod +x ssh-seanyang-me
```

### systemd Service

Create `/etc/systemd/system/ssh-seanyang-me.service`:

```ini
[Unit]
Description=ssh.seanyang.me SSH TUI server
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/ssh-seanyang-me
Environment=SSH_ADDR=0.0.0.0:22
Environment=SSH_HOST_KEY=/var/lib/ssh-seanyang-me/host_key
Environment=RUST_LOG=info
Restart=on-failure
RestartSec=5

NoNewPrivileges=true
ProtectSystem=strict
ReadWritePaths=/var/lib/ssh-seanyang-me

[Install]
WantedBy=multi-user.target
```

```bash
sudo mkdir -p /var/lib/ssh-seanyang-me
sudo cp ssh-seanyang-me /usr/local/bin/
sudo systemctl daemon-reload
sudo systemctl enable --now ssh-seanyang-me
```

---

## License

[MIT](LICENSE)
