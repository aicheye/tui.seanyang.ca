FROM ubuntu:22.04

ARG VERSION=latest
ARG TARGETARCH

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl unzip && \
    rm -rf /var/lib/apt/lists/*

# Map Docker TARGETARCH to release asset names, resolve latest tag if needed
RUN case "${TARGETARCH}" in \
    amd64) ASSET="tui-seanyang-me-linux-x64" ;; \
    *)     echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac && \
    TAG="${VERSION}" && \
    if [ "${TAG}" = "latest" ]; then \
    TAG=$(curl -fsSL "https://api.github.com/repos/aicheye/tui.seanyang.me/releases/latest" | \
    grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/'); \
    echo "Resolved latest -> ${TAG}"; \
    fi && \
    BASE="https://github.com/aicheye/tui.seanyang.me/releases/download/${TAG}" && \
    curl -fsSL "${BASE}/${ASSET}.zip" -o "/tmp/${ASSET}.zip" && \
    curl -fsSL "${BASE}/sha256sums.txt" -o /tmp/sha256sums.txt && \
    cd /tmp && grep "^[a-f0-9]*  ${ASSET}.zip$" sha256sums.txt | sha256sum --check && \
    unzip /tmp/${ASSET}.zip -d /usr/local/bin && \
    chmod +x /usr/local/bin/tui-seanyang-me && \
    rm /tmp/${ASSET}.zip /tmp/sha256sums.txt

RUN useradd -r -s /sbin/nologin appuser && \
    mkdir -p /data && \
    chown appuser:appuser /data

ENV SSH_ADDR=0.0.0.0:2222
ENV RUST_LOG=info

EXPOSE 2222

VOLUME ["/data"]

USER appuser

ENTRYPOINT ["tui-seanyang-me"]
