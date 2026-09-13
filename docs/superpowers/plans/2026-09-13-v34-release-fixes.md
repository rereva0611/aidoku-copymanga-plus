# CopyManga v34 Release Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Correct the three confirmed release defects and prepare the source for reproducible independent-repository publishing checks.

**Architecture:** Keep existing account and Markdown deep-link workarounds. Move the duplicate-operation decision into one shared favorite operation path, make input parsing explicit about URLs versus naked IDs, and preserve a successful free favorites response even when the paid endpoint fails. Tests remain pure where possible because the current test runner cannot model defaults deletion or public-site availability reliably.

**Tech Stack:** Rust 2024, aidoku-rs, aidoku-test, JSON manifests, aidoku CLI.

## Global Constraints

- Do not change Source ID, delete the uppercase-WWW workaround, replace the favorite API, or refactor unrelated upstream parsing.
- Do not log credentials, tokens, or cookies.
- Do not treat the old `.aix` as evidence for v34.
- Make the smallest change per defect and run the relevant test after each change.

---

### Task 1: Validate settings-page favorite input

**Files:**
- Modify: `src/tests.rs`
- Modify: `src/favorites.rs`

**Interfaces:**
- Produces: `parse_path_word(input: &str) -> Result<String>` accepting a recognized CopyManga comic URL or a constrained naked path word.

- [x] Add failing unit tests for result-banner text, malformed schemes, lookalike domains, valid configured hosts, query/fragment and a naked ID.
- [x] Run `cargo test parse_path_word_inputs -- --test-threads=1`; confirm the new malformed inputs fail under the old parser.
- [x] Parse a URL only after checking `http` or `https`, validate its host against the declared CopyManga host set, extract `/comic/{path_word}` or `/h5/details/comic/{path_word}`, and validate a naked path word with the evidence-backed character policy.
- [x] Re-run the focused test and confirm it passes.

### Task 2: Give both favorite entry points identical duplicate behavior

**Files:**
- Modify: `src/favorites.rs`
- Modify: `src/tests.rs`

**Interfaces:**
- Produces: one shared operation function used by both `deep_link_favorite` and `favorite_from_input`.

- [x] Extract only the pure action-selection rule necessary to unit-test duplicate add/remove behavior without network access.
- [x] Run the focused test and confirm it fails before implementation.
- [x] Make the shared core apply that decision; retain the existing real-time scan and preserve action failure feedback.
- [x] Re-run the focused test and all non-network tests.

### Task 3: Preserve a successful free-listing response

**Files:**
- Modify: `src/favorites.rs`
- Modify: `src/tests.rs`

**Interfaces:**
- Produces: `collect_page(page)` that does not convert a successful empty free response into an error solely because the paid response fails.

- [x] Extract the merge/fallback decision into a pure helper and add a failing test for empty-free plus failed-paid.
- [x] Run its focused test and confirm it fails under the old condition.
- [x] Implement the minimal successful-free fallback and retain existing `has_next_page` behavior for two successful responses.
- [x] Re-run focused and non-network tests.

### Task 4: Independent repository hygiene

**Files:**
- Create: `README.md`
- Create: `LICENSE-MIT`
- Create: `.gitignore`
- Modify: `Cargo.toml`

**Interfaces:**
- Produces: a source repository that documents private distribution, excludes generated artifacts and declares the dual license used by Aidoku Community.

- [x] Add a README that states it is an unofficial source, gives non-sensitive local build/test instructions, warns that public publishing is only after v34 validation, and explains that the community submission must target the existing upstream Source ID.
- [x] Preserve upstream attribution with the MIT license and declare `license = "MIT"` in package metadata. An upstream contribution remains subject to the Community repository's contribution terms.
- [x] Ignore `target/`, `*.aix`, and local editor/OS files; do not ignore Cargo.lock.
- [x] Check JSON formatting, `cargo fmt --check`, `cargo clippy -- -D warnings`, and tests. Report unavailable device tests rather than claiming them.

### Task 5: Release evidence

- [x] Run `cargo test -- --test-threads=1`.
- [x] Run `aidoku package` only after tests and linting are green; verify the newly produced v34 artifact with `aidoku verify`.
- [ ] Perform the specified Aidoku device matrix before any public upload: anonymous reading, login/relogin/logout, free/paid favorites, write interception page, deep-link routing with other CopyManga sources installed, and defaults deletion.
