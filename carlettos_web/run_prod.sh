export TRUNK_BUILD_RELEASE=true
export TRUNK_SERVE_PORT=443
export TRUNK_SERVE_ADDRESS="0.0.0.0"
export TRUNK_SERVE_TLS_KEY_PATH="/etc/letsencrypt/live/scirustic.cl/privkey.pem"
export TRUNK_SERVE_TLS_CERT_PATH="/etc/letsencrypt/live/scirustic.cl/fullchain.pem"
export API_IP="https://scirustic.cl:8080"

trunk serve --release