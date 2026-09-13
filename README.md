# 拷貝漫畫 Plus

这是一个非官方的 [Aidoku](https://aidoku.app/) CopyManga 图源，提供账户登录和网站收藏功能。项目与 Aidoku、CopyManga 均无关联。

## 项目状态

这是独立维护的发布线，Source ID 为 `zh.copymanga.plus`。在账户收藏功能完成验证期间，它与社区原版图源分开发布。

本地的 `package.aix` 只是构建产物，不应直接作为正式发布依据。请从当前源码重新打包并验证后再分发。

## 开发与验证

需要安装 Rust、`wasm32-unknown-unknown` 编译目标和 Aidoku CLI。

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test -- --test-threads=1
aidoku package
aidoku verify package.aix
```

公开发布前，应在 Aidoku 设备上验证匿名阅读、登录/登出/重新登录、收藏读取与写入、token 过期续期和详情页 deep link。

## 向 Aidoku Community 贡献

若未来向 Aidoku Community 提交改动，应基于社区现有的 CopyManga 图源，并保留其原有 Source ID；不要将独立的 `zh.copymanga.plus` 作为平行重复图源提交。请使用聚焦的 Pull Request、Conventional Commit 标题，并附上真机测试结果。

## 许可证

本项目使用 [MIT 许可证](LICENSE-MIT)。原 Aidoku Community 图源采用双许可证；未来向社区提交的代码仍受社区仓库贡献条款约束。
