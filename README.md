> [!WARNING]
> ## Deprecated and Archived
>
> This standalone prototype has been superseded by the active monorepo.
> The maintained DAG Layer 1 implementation now lives in
> [extropy-engine/packages/dag-substrate](https://github.com/00ranman/extropy-engine/tree/main/packages/dag-substrate).
>
> This repository is archived (read-only) and kept for history. Do not build on it.
> See [ECOSYSTEM_MAP.md](https://github.com/00ranman/extropy-engine/blob/main/ECOSYSTEM_MAP.md) for the full mapping.

# ⚠️ DEPRECATED — Use [xp-dag-mesh](https://github.com/00ranman/xp-dag-mesh)

> **This repository has been superseded by [xp-dag-mesh](https://github.com/00ranman/xp-dag-mesh).**
> All active development, issues, and contributions have moved there.

## Why?

`xp-net` was the original prototype for the physics-anchored DAG Layer 1 protocol. The project has since been restructured and expanded as `xp-dag-mesh`, which includes:

- Entropy-weighted consensus engine
- Desktop client (Tauri + TypeScript)
- Ecosystem integration with Extropy Engine
- Monorepo architecture for all Extropy platform services

## Migration

All code from `xp-net` has been incorporated into `xp-dag-mesh`. If you have local clones, update your remote:

```bash
git remote set-url origin https://github.com/00ranman/xp-dag-mesh.git
```

This repository is archived for historical reference only.
