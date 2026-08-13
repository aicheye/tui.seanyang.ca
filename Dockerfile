FROM ubuntu:22.04

ARG VERSION=latest
ARG TARGETARCH

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl unzip && \
    rm -rf /var/lib/apt/lists/*

# Fetch latest-release metadata. BuildKit revalidates remote URLs on every
# build, so this layer (and everything below it) is invalidated whenever a new
# release is published -- making `--build` actually pick up new releases instead
# of reusing the cached download layer.
ADD https://api.github.com/repos/aicheye/tui.seanyang.me/releases/latest /tmp/release.json

# Map Docker TARGETARCH to release asset names, resolve latest tag if needed
RUN case "${TARGETARCH}" in \
    amd64) ASSET="tui-seanyang-ca-linux-x64" ;; \
    arm64) ASSET="tui-seanyang-ca-linux-arm64" ;; \
    *)     echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac && \
    TAG="${VERSION}" && \
    if [ "${TAG}" = "latest" ]; then \
    TAG=$(grep '"tag_name"' /tmp/release.json | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/'); \
    echo "Resolved latest -> ${TAG}"; \
    fi && \
    BASE="https://github.com/aicheye/tui.seanyang.me/releases/download/${TAG}" && \
    curl -fsSL "${BASE}/${ASSET}.zip" -o "/tmp/${ASSET}.zip" && \
    curl -fsSL "${BASE}/sha256sums.txt" -o /tmp/sha256sums.txt && \
    cd /tmp && grep "^[a-f0-9]*  ${ASSET}.zip$" sha256sums.txt | sha256sum --check && \
    unzip /tmp/${ASSET}.zip -d /usr/local/bin && \
    chmod +x /usr/local/bin/tui-seanyang-ca && \
    rm /tmp/${ASSET}.zip /tmp/sha256sums.txt /tmp/release.json

RUN useradd -r -s /sbin/nologin appuser && \
    mkdir -p /data && \
    chown appuser:appuser /data

ENV SSH_ADDR=0.0.0.0:2222
ENV RUST_LOG=info

EXPOSE 2222

VOLUME ["/data"]

USER appuser

ENTRYPOINT ["tui-seanyang-ca"]
