# Release Process

OLManager releases are maintainer-owned and source-first until signing/notarization and binary packaging policy is finalized.

## Branch flow

1. Community work merges into `development`.
2. Maintainer opens a release PR from `development` to `main`.
3. Release PR verifies versions, changelog, release notes, provenance, and required checks.
4. After merge to `main`, maintainer creates a version tag or runs release dispatch.
5. Release workflow creates source archive artifacts, platform bundles, and the auto-update manifest (`latest.json`).

## Release PR checklist

- [ ] `package.json` version is correct.
- [ ] `src-tauri/Cargo.toml` version is correct.
- [ ] `src-tauri/tauri.conf.json` version is correct.
- [ ] `CHANGELOG.md` has a dated release section.
- [ ] Release notes mention unsigned/signed artifact status.
- [ ] Data provenance changes are documented.
- [ ] [`docs/INHERITED_DOCS_AUDIT.md`](INHERITED_DOCS_AUDIT.md) is complete, or release notes explicitly disclose remaining unaudited inherited docs.
- [ ] Required PR checks `frontend-install` and `rust-check` pass.
- [ ] Manual experimental checks have been run and reviewed, or remaining failures are explicitly documented in release notes: `frontend-full-experimental` and `rust-full-experimental`.
- [ ] No production Tauri bundle build is required by PR CI.

## Version sync

The project version must stay aligned across:

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

The release workflow verifies these values before producing source artifacts.

## Tags

Use semantic version tags with a `v` prefix, for example:

```text
v0.2.1
v0.3.0
```

## Artifacts

The release workflow produces:

- Source archive and SHA-256 checksums.
- Platform binaries for Windows (`.exe`, `.msi`), Linux (`.AppImage`, `.deb`, `.rpm`), and macOS (`.dmg`, `.app.tar.gz`).
-Signature files (`.sig`) for each platform binary (if `TAURI_SIGNING_PRIVATE_KEY` is configured).
- Auto-update manifest (`latest.json`) with download URLs, signatures, and release notes.

## Auto-update

The app uses `tauri-plugin-updater` to check for and apply updates automatically. The update flow:

1. App checks the endpoint configured in `tauri.conf.json` (`plugins.updater.endpoints`).
2. Compares current version against `latest.json`.
3. If a newer version is available, downloads the platform-specific binary with signature verification.
4. Installs and restarts the app.

### Update manifest format

`latest.json` is generated automatically by the release workflow with this structure:

```json
{
  "version": "0.2.0",
  "notes": "## [0.2.0] - 2026-05-01\n...",
  "pub_date": "2026-05-01T12:00:00+00:00",
  "platforms": {
    "windows-x86_64": { "url": "...", "signature": "..." },
    "linux-x86_64": { "url": "...", "signature": "..." },
    "darwin-aarch64": { "url": "...", "signature": "..." }
  }
}
```

### Platform support for auto-update

| Platform | Auto-update works | Notes |
|----------|-------------------|-------|
| Linux (AppImage) | Yes | Fully supported |
| Windows (.exe) | Yes | SmartScreen warning if unsigned |
| macOS (.app.tar.gz) | Partial | Gatekeeper blocks unsigned apps; requires code signing + notarization for seamless updates |

## Hotfixes

Hotfixes may branch from `main` and target `main` only when the issue cannot wait for normal `development` promotion. After the hotfix release, back-merge `main` into `development` immediately.

## Signing and notarization placeholders

Potential future secrets include:

- `TAURI_SIGNING_PRIVATE_KEY` — Tauri updater signing key (minisign format). Generate with `npx @tauri-apps/cli signer generate`. The corresponding public key goes in `tauri.conf.json` `plugins.updater.pubkey`.
- Apple Developer ID certificate and notarization credentials.
- Windows signing certificate.
- Linux package signing key.
- GitHub release token permissions.

Do not add real secret names, credentials, or signing logic until maintainers decide the release policy.