# 拷貝漫畫 Plus

适用于 [Aidoku](https://aidoku.app/) 的 CopyManga 增强图源。在保留搜索和阅读功能的基础上，增加了账号收藏功能。

## 主要功能

- 无需登录即可搜索和阅读漫画
- 登录 CopyManga 账号
- 查看免费和付费收藏
- 在漫画详情页加入或取消收藏
- 登录状态过期后自动恢复
- 支持地区、状态和排序筛选

## 安装

发布后，请从本仓库的 Releases 页面下载最新 `package.aix`，并导入 Aidoku。

## 使用

阅读漫画不需要登录。使用收藏功能时，请先在图源设置中登录，然后在漫画详情页加入或取消收藏。“我的收藏”可以从首页或 Explore 页面打开。

## 说明

这是非官方项目，与 Aidoku、CopyManga 均无关联。Plus 版拥有独立的 Source ID，可以与社区原版分别安装。

<details>
<summary>开发与验证</summary>

开发环境需要 Rust、`wasm32-unknown-unknown` 编译目标和 Aidoku CLI。

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test -- --test-threads=1
aidoku package
aidoku verify package.aix
```

若未来向 Aidoku Community 贡献代码，应基于社区现有的 CopyManga 图源并保留其原有 Source ID。

</details>

## 许可证

本项目使用 [MIT 许可证](LICENSE-MIT)。
