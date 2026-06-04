#!/bin/bash
set -euo pipefail

DOMAIN="olmanager.nicorueda.dev"
APP_DIR="/opt/olmanager"

echo "=== OLManager Hetzner Setup ==="
echo "Domain: $DOMAIN"
echo "App dir: $APP_DIR"
echo ""

# 1. Create app directory
sudo mkdir -p "$APP_DIR"
cd "$APP_DIR"

# 2. Create .env file
if [ ! -f .env ]; then
    echo "Creating .env file..."
    cat > .env << 'EOF'
DATABASE_URL=postgres://user:pass@host:5432/postgres
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_JWKS_URL=https://your-project.supabase.co/auth/v1/.well-known/jwks.json
SUPABASE_SERVICE_ROLE_KEY=your_service_role_key
OLM_ALLOW_IMPORT=false
RUST_LOG=info
EOF
    echo ">>> IMPORTANTE: editá .env con tus credenciales reales <<<"
    echo ">>> Ejecutá: nano $APP_DIR/.env <<<"
else
    echo ".env ya existe"
fi

# 3. Install Docker if missing
if ! command -v docker &> /dev/null; then
    echo "Instalando Docker..."
    curl -fsSL https://get.docker.com | sudo bash
    sudo usermod -aG docker "$USER"
fi

# 4. Install Docker Compose plugin if missing
if ! docker compose version &> /dev/null; then
    echo "Instalando Docker Compose plugin..."
    sudo apt-get update
    sudo apt-get install -y docker-compose-plugin
fi

# 5. Download production compose file
echo "Descargando docker-compose.prod.yml..."
sudo curl -fsSL -o docker-compose.yml \
    "https://raw.githubusercontent.com/NicoRuedaA/OLManager/main/docker-compose.prod.yml"

# 6. Setup nginx + SSL via Certbot
if ! command -v certbot &> /dev/null; then
    echo "Instalando nginx + certbot..."
    sudo apt-get update
    sudo apt-get install -y nginx certbot python3-certbot-nginx
fi

# Create nginx config
sudo tee /etc/nginx/sites-available/olmanager > /dev/null << NGINX
server {
    listen 80;
    server_name $DOMAIN;

    location /api/ {
        proxy_pass http://127.0.0.1:3001;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        client_max_body_size 500M;
    }

    location /health {
        proxy_pass http://127.0.0.1:3001;
        proxy_set_header Host \$host;
    }

    location / {
        proxy_pass http://127.0.0.1:80;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
    }
}
NGINX

sudo ln -sf /etc/nginx/sites-available/olmanager /etc/nginx/sites-enabled/
sudo rm -f /etc/nginx/sites-enabled/default
sudo nginx -t && sudo systemctl reload nginx

# 7. Get SSL certificate
echo "Obteniendo certificado SSL para $DOMAIN..."
sudo certbot --nginx -d "$DOMAIN" --non-interactive --agree-tos --email admin@nicorueda.dev || \
    echo ">>> Certbot falló. Ejecutá manualmente: sudo certbot --nginx -d $DOMAIN <<<"

echo ""
echo "=== Setup completado ==="
echo ""
echo "Pasos siguientes:"
echo "1. Editá las credenciales: nano $APP_DIR/.env"
echo "2. Configurá DNS: apuntá $DOMAIN a la IP del server"
echo "3. Ejecutá: cd $APP_DIR && docker compose pull && docker compose up -d"
echo ""
echo "Después del primer deploy desde GitHub Actions, los containers se actualizan solos."
