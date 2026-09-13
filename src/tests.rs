#![cfg(test)]

use crate::Copymanga;
use aidoku::{
	Home as _, Listing, ListingProvider as _, Source,
	alloc::{String, Vec, format},
	imports::defaults::{DefaultValue, defaults_set},
};

fn setup() -> Copymanga {
	defaults_set(
		"url",
		DefaultValue::String(String::from("https://www.copy3000.com")),
	);
	Copymanga::new()
}

/// 回归：匿名搜索在改动后仍正常。
#[aidoku_test::aidoku_test]
fn search_regression() {
	let source = setup();
	let result = source
		.get_search_manga_list(Some(String::from("戀愛")), 1, Vec::new())
		.expect("search should succeed");
	assert!(
		!result.entries.is_empty(),
		"search should return entries, got {}",
		result.entries.len()
	);
	assert!(!result.entries[0].key.is_empty());
}

/// 回归：分类浏览（filters 路径）仍正常。
#[aidoku_test::aidoku_test]
fn filters_regression() {
	let source = setup();
	let result = source
		.get_search_manga_list(None, 1, Vec::new())
		.expect("filters listing should succeed");
	assert!(!result.entries.is_empty(), "filters should return entries");
}

/// 收藏列表在未登录时必须返回可理解的错误，而不是崩溃或空页。
#[aidoku_test::aidoku_test]
fn favorites_listing_requires_login() {
	let source = setup();
	let err = source
		.get_manga_list(
			Listing {
				id: String::from("f:fav"),
				..Default::default()
			},
			1,
		)
		.expect_err("should fail when not logged in");
	let message = format!("{err:?}");
	assert!(message.contains("登錄"), "unexpected error: {message}");
}

/// 未登录时 Home 应为空布局而非报错。
#[aidoku_test::aidoku_test]
fn home_without_login_is_empty() {
	let source = setup();
	let home = source.get_home().expect("home should not error");
	assert!(
		home.components.is_empty(),
		"anonymous home should have no favorite components"
	);
}

/// 未用到的 filter id 应报错（确认路由没被破坏）。
#[allow(unused_variables)]
#[aidoku_test::aidoku_test]
fn unknown_listing_errors() {
	let source = setup();
	assert!(
		source
			.get_manga_list(
				Listing {
					id: String::from("f:unknown"),
					..Default::default()
				},
				1,
			)
			.is_err()
	);
}

/// 设置页和详情页应使用同一规则，避免重复的收藏写操作。
#[aidoku_test::aidoku_test]
fn favorite_operation_needed_inputs() {
	assert!(!crate::favorites::favorite_operation_needed(true, true));
	assert!(!crate::favorites::favorite_operation_needed(false, false));
	assert!(crate::favorites::favorite_operation_needed(true, false));
	assert!(crate::favorites::favorite_operation_needed(false, true));
}

/// 付费书架暂时失败不应覆盖已经成功获取的免费空列表。
#[aidoku_test::aidoku_test]
fn merge_collect_pages_keeps_empty_free_result() {
	let free = aidoku::MangaPageResult {
		entries: Vec::new(),
		has_next_page: false,
	};
	let result =
		crate::favorites::merge_collect_pages(free, Err(aidoku::error!("paid unavailable")))
			.expect("successful free page should be preserved");
	assert!(result.entries.is_empty());
	assert!(!result.has_next_page);
}

/// 纯函数：收藏状态 map 的写入/一次性消息消费/登出清除。
#[aidoku_test::aidoku_test]
fn fav_state_map_roundtrip() {
	// 写入状态 + 消息
	crate::favorites::set_collected_state("aaa111", true);
	crate::favorites::set_fav_msg("aaa111", "✅ 已加入書架");
	assert_eq!(crate::favorites::get_collected_state("aaa111"), Some(true));
	// 消息一次性：读取后即消失
	assert_eq!(
		crate::favorites::take_fav_msg("aaa111").as_deref(),
		Some("✅ 已加入書架")
	);
	assert_eq!(crate::favorites::take_fav_msg("aaa111"), None);
	// 状态仍在（消息消费不影响按钮状态）
	assert_eq!(crate::favorites::get_collected_state("aaa111"), Some(true));
	// 未知的漫画 = None（触发实时扫描）
	assert_eq!(crate::favorites::get_collected_state("unknown999"), None);
	// 登出清除
	crate::favorites::clear_all_state();
	assert_eq!(crate::favorites::get_collected_state("aaa111"), None);
}

/// 纯函数：简介按钮 deep link 解析（兼容带 // 与不带 // 的 URL 形态）。
#[aidoku_test::aidoku_test]
fn parse_fav_deep_link_inputs() {
	use crate::parse_fav_deep_link;
	// App 实际传入形态（无 //）
	assert_eq!(
		parse_fav_deep_link("https:www.copy3000.com/__fav/add/jtyhjissmdpj"),
		Some((String::from("jtyhjissmdpj"), true))
	);
	assert_eq!(
		parse_fav_deep_link("https:www.copy3000.com/__fav/remove/abc123"),
		Some((String::from("abc123"), false))
	);
	// 完整形态
	assert_eq!(
		parse_fav_deep_link("https://www.copy3000.com/__fav/add/xyz000"),
		Some((String::from("xyz000"), true))
	);
	// 动作段缺失漫画 ID 时必须失败（v27 的 bug 场景）
	assert_eq!(
		parse_fav_deep_link("https://www.copy3000.com/__fav/add/"),
		None
	);
	// 未知动作
	assert_eq!(
		parse_fav_deep_link("https://www.copy3000.com/__fav/toggle/abc"),
		None
	);
	// 非收藏路径
	assert_eq!(
		parse_fav_deep_link("https://www.copy3000.com/comic/abc"),
		None
	);
}
