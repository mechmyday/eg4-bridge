#!/usr/bin/with-contenv bashio

bashio::log.info "Creating eg4-bridge config from options..."

yq -oy /data/options.json > /etc/config.yaml

bashio::log "Done."

bashio::log.info "Starting eg4-bridge..."

/usr/local/bin/eg4-bridge -c /etc/config.yaml
