# CopyManga 收藏 Source

An unofficial [Aidoku](https://aidoku.app/) Source for CopyManga, with optional account-backed favorites. It is not affiliated with Aidoku or CopyManga.

## Status

This repository is an independent development and release line. The current Source ID is `zh.copymanga.v27`; it is intentionally separate from the community source while the account-favorites work is validated.

Do not treat a local `package.aix` as a release artifact. Build a package from the checked-out source and validate it before distribution.

## Development

Requirements: Rust with the `wasm32-unknown-unknown` target and the Aidoku CLI.

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test -- --test-threads=1
aidoku package
aidoku verify package.aix
```

Before public release, test anonymous reading, login/logout/relogin, favorite reads and writes, expired-token recovery, and detail-page deep links on an Aidoku device.

## Community contribution

An Aidoku Community pull request must target the existing upstream CopyManga source and keep its upstream Source ID. It must not submit this experimental `zh.copymanga.v27` Source as a parallel duplicate. Follow the community contribution guide, use a focused PR and Conventional Commit title, and include device-test evidence.

## License

Licensed under [MIT](LICENSE-MIT). The original Aidoku Community source is dual-licensed; an eventual upstream contribution is governed by that repository's contribution terms.
