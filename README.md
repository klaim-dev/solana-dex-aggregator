# Solana DEX Aggregator

Real-time DEX aggregation engine for Solana — finds optimal swap routes across Orca and Raydium with live analytics.

> ⚠️ **Work in progress.** Currently building the off-chain API foundation. Full aggregation engine coming soon.

## Architecture

```
                    ┌─────────────┐
  Solana RPC ──────▶│   Ingest    │
  (WebSocket)       └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │   Decode    │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │   Route     │
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
       ┌──────▼──┐  ┌──────▼──┐  ┌──────▼──┐
       │  Store  │  │  Cache  │  │   API   │
       │ (Postgres)│ │ (Redis) │  │ (Axum)  │
       └─────────┘  └─────────┘  └────┬────┘
                                      │
                               ┌──────▼──────┐
                               │  Dashboard  │
                               └─────────────┘
```

## Current Status

| Component | Stage | Status |
|-----------|-------|--------|
| API (Axum + SQLx) | Stage 1-2 | 🚧 In progress |
| Engine Core | Stage 3 | 🔜 |
| Ingestion (Orca + Raydium) | Stage 4 | 🔜 |
| Routing Engine | Stage 4 | 🔜 |
| Dashboard | Stage 4 | 🔜 |

## Quick Start

```bash
# Start dependencies
docker-compose up -d postgres

# Run API
cd crates/api
cp .env.example .env
cargo run
```

## Tech Stack

- **Language:** Rust
- **API:** Axum
- **Database:** PostgreSQL (SQLx)
- **Cache:** Redis
- **On-chain:** Anchor (Solana)
- **Frontend:** Next.js (planned)

## Project Structure

```
solana-dex-aggregator/
├── crates/
│   └── api/              ← current focus
├── programs/             ← Stage 4 (on-chain, stretch)
├── dashboard/            ← Stage 4
├── docker-compose.yml
└── ARCH.md
```
