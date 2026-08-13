# tui.seanyang.ca

A personal portfolio and introduction, served over SSH as an interactive TUI. Heavily inspired by <https://www.terminal.shop/>

```bash
ssh seanyang.ca
ssh tui.seanyang.ca
```

The previous domain still answers during the migration: `ssh seanyang.me` and
`ssh tui.seanyang.me` reach the same server.

<img width="920" height="664" alt="image" src="https://github.com/user-attachments/assets/3315f01f-383a-4104-99ca-676e722884d7" />

## Deployment

This guide covers hosting the TUI on a home server while keeping normal SSH access to the machine.

The idea: move your real SSH daemon to a non-standard port, then give port 22 to the TUI container so that `ssh tui.seanyang.ca` connects directly to it.

### Prerequisites

- A machine with [Docker](https://docs.docker.com/get-docker/) installed
- An A record for `tui.seanyang.ca` pointing to the machine's public IP (plus
  one for `tui.seanyang.me` for as long as the old domain is kept alive)
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
    HostName tui.seanyang.ca
    Port 2200
    User your-username
```

Now `ssh home` connects to your real shell.

### 2. Start the Container

```bash
git clone https://github.com/aicheye/tui.seanyang.me.git
cd tui.seanyang.me
docker compose up -d --build
```

This builds the image and starts the container, mapping host port `22` → container port `2222`, so visitors running `ssh tui.seanyang.ca` hit the TUI.

### 3. Verify

From another machine:

```bash
# Should open the TUI
ssh tui.seanyang.ca

# Should open your real shell
ssh -p 2200 tui.seanyang.ca
```

---

## Configuration

All configuration is done via environment variables:

| Variable | Default | Description |
|---|---|---|
| `SSH_ADDR` | `0.0.0.0:2222` | Address and port the SSH server binds to inside the container |
| `SSH_HOST_KEY` | `/data/host_key` | Path to the Ed25519 host key (auto-generated on first run) |
| `SITE_DATA_URL` | `https://cdn.jsdelivr.net/gh/aicheye/seanyang.me@main/public/data` | Base URL the content (jobs, projects, quotes, socials, adjectives) is fetched from |
| `RUST_LOG` | `info` | Log level filter (`trace`, `debug`, `info`, `warn`, `error`) |

### Content

The page content is pulled at runtime over HTTP from `SITE_DATA_URL`. The
canonical JSON lives in the website repo under `public/data/` (the single source
of truth shared with the web frontend); it's served via jsDelivr by default,
since `seanyang.ca` itself sits behind Vercel's bot challenge which blocks
non-browser clients. Content is refreshed on each SSH connection, rate-limited to
one pull per minute. A bundled snapshot keeps the TUI fully functional if the
source is unreachable.

### Persisting the Host Key

The host key lives at `/data/host_key` inside the container. The volume mount (`-v ssh-host-key:/data`) ensures it survives container restarts.

> [!IMPORTANT]
> If the host key changes, returning visitors will see an SSH host key mismatch warning.

---

## Host Deployment (without Docker)

Download the latest binary from [GitHub Releases](https://github.com/aicheye/tui.seanyang.me/releases):

```bash
curl -fsSL https://github.com/aicheye/tui.seanyang.me/releases/download/v0.1.7/tui-seanyang-me-linux-x64.zip \
  -o tui-seanyang-me.zip
unzip tui-seanyang-me.zip
chmod +x tui-seanyang-me
```

### systemd Service

Create `/etc/systemd/system/tui-seanyang-me.service`:

```ini
[Unit]
Description=tui.seanyang.ca SSH TUI server
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/tui-seanyang-me
Environment=SSH_ADDR=0.0.0.0:22
Environment=SSH_HOST_KEY=/var/lib/tui-seanyang-me/host_key
Environment=RUST_LOG=info
Restart=on-failure
RestartSec=5

NoNewPrivileges=true
ProtectSystem=strict
ReadWritePaths=/var/lib/tui-seanyang-me

[Install]
WantedBy=multi-user.target
```

```bash
sudo mkdir -p /var/lib/tui-seanyang-me
sudo cp tui-seanyang-me /usr/local/bin/
sudo systemctl daemon-reload
sudo systemctl enable --now tui-seanyang-me
```

---

## License

[MIT](LICENSE)
