FROM debian:bookworm-slim

ARG VERSION=v0.1.1
ARG TARGETARCH

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl unzip && \
    rm -rf /var/lib/apt/lists/*

# Map Docker TARGETARCH to release asset names
RUN case "${TARGETARCH}" in \
      amd64) ASSET="ssh-seanyang-me-linux-x64" ;; \
      arm64) ASSET="ssh-seanyang-me-linux-arm" ;; \
      *)     echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac && \
    curl -fsSL "https://github.com/aicheye/ssh.seanyang.me/releases/download/${VERSION}/${ASSET}.zip" \
         -o /tmp/release.zip && \
    unzip /tmp/release.zip -d /usr/local/bin && \
    chmod +x /usr/local/bin/ssh-seanyang-me && \
    rm /tmp/release.zip

RUN mkdir -p /data

ENV SSH_ADDR=0.0.0.0:2222
ENV SSH_HOST_KEY=/data/host_key
ENV RUST_LOG=info

EXPOSE 2222

VOLUME ["/data"]

ENTRYPOINT ["ssh-seanyang-me"]
