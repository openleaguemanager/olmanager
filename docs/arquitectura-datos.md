# Arquitectura de Datos — OLManager

## Estructura de archivos

```
src-tauri/data/
├── leagues.json                 
├── competitions/                ← Reglas de calendario de cada competicion
│   ├── lec/manifest.json
│   ├── cblol/manifest.json
│   └── ... (12 competiciones)
├── teams/                       ← Equipos organizados por region
│   ├── emea_teams.json          
│   ├── kr_teams.json            
│   ├── na_teams.json            
│   └── ... (15 archivos)
├── players/                     ← Jugadores organizados por region
│   ├── emea_players.json
│   ├── kr_players.json
│   └── ... (15 archivos)
├── staffs/
│   └── free_agents.json         ← Staff libre (el staff por equipo se genera proceduralmente)
├── draft/
│   └── champions.json           ← Catalogo global de champions (unico archivo vivo del draft)
├── erls/                        ← Seeds de academias (texto)
│   ├── les.txt, lfl.txt, Prime League.txt
└── leagues.json                 ← Metadata de ligas para el frontend
```

## Flujo al iniciar partida

```
Frontend: league_selection -> team_selection -> start_new_game()
```
