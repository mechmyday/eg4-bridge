ARG BUILD_VERSION
ARG BUILD_FROM

FROM mechmyday/eg4-bridge:${BUILD_VERSION} AS eg4-bridge

FROM $BUILD_FROM
ARG BUILD_ARCH

COPY --from=eg4-bridge / /

RUN apt-get update && \
    apt-get install -y --no-install-recommends curl ca-certificates && \
    case "$BUILD_ARCH" in \
      aarch64) yq_arch=arm64 ;; \
      amd64)   yq_arch=amd64 ;; \
      armv7)   yq_arch=arm ;; \
      *) echo "unsupported BUILD_ARCH: $BUILD_ARCH" >&2; exit 1 ;; \
    esac && \
    curl -fsSL "https://github.com/mikefarah/yq/releases/latest/download/yq_linux_${yq_arch}" -o /usr/local/bin/yq && \
    chmod +x /usr/local/bin/yq && \
    rm -rf /var/lib/apt/lists/*

COPY run.sh /
RUN chmod a+x /run.sh

CMD [ "/run.sh" ]
