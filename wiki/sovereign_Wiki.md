# SigmaOS-sovereign Wiki

## Overview
Sovereign init (PID 1) and service supervisor suite extracted from SigmaOS-system (v52-v76 design variants). Part of the SigmaOS multi-repository component fleet; implementation
was extracted from the SigmaOS-system monolith and hardened where required
(`OnceLock<Mutex<T>>` singletons, serialized global-state tests).

## Modules
- `abi`
- `sovereign_init_and_supervisor_advancements_v52`
- `sovereign_init_and_supervisor_advancements_v53`
- `sovereign_init_and_supervisor_advancements_v60`
- `sovereign_init_and_supervisor_advancements_v61`
- `sovereign_init_and_supervisor_advancements_v62`
- `sovereign_init_and_supervisor_advancements_v63`
- `sovereign_init_and_supervisor_advancements_v63_pr`
- `sovereign_init_and_supervisor_advancements_v67`
- `sovereign_init_and_supervisor_advancements_v68_pr`
- `sovereign_init_and_supervisor_advancements_v69_pr`
- `sovereign_init_and_supervisor_advancements_v71_pr`
- `sovereign_init_and_supervisor_advancements_v75_pr`
- `sovereign_init_and_supervisor_advancements_v76_pr`
- `sovereign_init_and_supervisor_inspirations_pr`
- `sovereign_init_supervisor`

## Principles
- Single Responsibility / one component per repo
- Fail-fast initialization and defensive programming
- Race-free, deadlock-free singleton access
- Deterministic, isolated tests (`cargo test`)
- Stable API via `COMPONENT.toml` contract (`provides`/`depends_on`)

## Usage
```bash
cargo test          # run component tests
cargo build --release
```

## Integration
Registered in the connector registry (`COMPONENTS.toml`) and reachable via
`scripts/sigma_hub.sh` fleet operations (`list`, `status`, `test`, `sync`).
