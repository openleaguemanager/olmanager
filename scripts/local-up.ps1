#!/usr/bin/env pwsh
# ── Local Docker dev ──────────────────────────────────────────
# Build and run the project locally using Docker.
#
# Prerequisites:
#   1. Docker Desktop installed and running
#   2. .env.local file with your Supabase credentials
#
# Usage:
#   .\scripts\local-up.ps1          # build + start
#   .\scripts\local-up.ps1 -build   # force rebuild
#   .\scripts\local-up.ps1 -logs    # attach logs
#   .\scripts\local-up.ps1 -down    # stop

param(
    [switch]$build,
    [switch]$logs,
    [switch]$down
)

$root = Split-Path -Parent (Split-Path -Parent $PSCommandPath)
$envFile = "$root\.env.local"

if (-not (Test-Path $envFile)) {
    Write-Host "❌ .env.local not found. Create it from .env.local.example:" -ForegroundColor Red
    Write-Host "   copy .env.local.example .env.local" -ForegroundColor Yellow
    exit 1
}

if ($down) {
    Write-Host "⏹️  Stopping containers..." -ForegroundColor Cyan
    docker compose --env-file $envFile -f "$root\docker-compose.yml" down
    exit 0
}

$cmd = "docker compose --env-file $envFile -f `"$root\docker-compose.yml`""
if ($build) {
    $cmd += " up --build"
} else {
    $cmd += " up"
}
if ($logs) {
    $cmd += " -d"
}

Write-Host "🚀 Starting OLManager locally..." -ForegroundColor Cyan
Write-Host "   Backend → http://localhost:3001" -ForegroundColor Green
Write-Host "   Frontend → http://localhost:80" -ForegroundColor Green

Invoke-Expression $cmd

if ($logs) {
    Write-Host "📋 Attaching logs (Ctrl+C to stop)..." -ForegroundColor Cyan
    docker compose --env-file $envFile -f "$root\docker-compose.yml" logs -f
}
