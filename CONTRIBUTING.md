# Contributing to sotto-contracts

Thanks for considering a contribution. This document covers everything you need
to go from cloning the repo to a merged PR.

If something here is unclear or wrong, that's a bug in this document — open an
issue and say so.

---

## What this repo is

Soroban smart contracts for **Sotto**, confidential B2B settlement on Stellar.
Two businesses settle in USDC without publishing what they paid each other,
while a designated auditor retains full visibility.

For the full picture, read [`SOTTO_PROJECT_DOC.md`](./SOTTO_PROJECT_DOC.md).
You don't need to read it to fix a single issue, but it explains why the pieces
exist.

### You do not need to know zero-knowledge cryptography

This is worth stating plainly, because the word "confidential" scares people off.

Sotto writes no cryptography. It consumes Stellar's Confidential Tokens as a
dependency. The test harness ships a **mock Confidential Token client**, so
almost every issue in this repo — the invoice registry, the settlement
orchestrator, netting attestation, the policy adapter — can be built and tested
against a stub without touching a proof system.

The handful of issues that do touch the proof layer are labelled `hard` and link
to the upstream OpenZeppelin and Nethermind resources.

---

## Setup

**Prerequisites**

| Tool | Version |
|---|---|
| Rust | stable (see `rust-toolchain.toml`) |
| Target | `wasm32-unknown-unknown` |
| Stellar CLI | latest |

```bash
# Rust, if you don't have it
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# the wasm target — required, the build fails without it
rustup target add wasm32-unknown-unknown

# Stellar CLI
cargo install --locked stellar-cli
```

**Clone and verify**

```bash
git clone https://github.com/sotto-labs/sotto-contracts.git
cd sotto-contracts
cargo build --target wasm32-unknown-unknown --release
cargo test
```

If those two commands pass, you're ready. The first build compiles the entire
`soroban-sdk` tree and takes several minutes — that's normal, not a hang.

---

## Finding something to work on

Issues are published in batches as scaffolding lands, so everything open is
genuinely ready to start. Nothing in the tracker is a placeholder.

**Labels**

| Label | Meaning |
|---|---|
| `easy` | Self-contained. Good first issue. No cross-module knowledge needed. |
| `medium` | Touches one module and its tests. Some context required. |
| `hard` | Design judgement, cross-module, or proof-layer work. |
| `contracts` | This repo. |

If you're new here, start with an `easy` issue. They're real work — event
emission, query endpoints, error-code documentation — not busywork, and
completing one teaches you the conventions before you take on netting logic.

**Claiming an issue**

Work is assigned through [GrantFox](https://grantfox.xyz) before it starts, so
contributors aren't racing each other on the same ticket. Apply there rather
than opening an unsolicited PR.

New contributors take **one issue at a time** until their first merge.

---

## Anatomy of an issue

Every issue is derived from a structured block in the source. You'll see these
above the stubs:

```rust
// ISSUE: [contracts] Implement invoice commitment storage
// CONTEXT: Why this exists and how it fits the settlement flow.
// SCOPE:   The exact function and its responsibility.
// ACCEPTANCE:
//   - objectively checkable condition
//   - objectively checkable condition
// TESTS: named cases, including at least one failure path
// OUT OF SCOPE: what you must not touch
// DIFFICULTY: medium
// LABELS: contracts, soroban, storage
```

Two fields matter more than the rest:

- **ACCEPTANCE** defines done. Every bullet must be true before you open a PR.
- **OUT OF SCOPE** is binding. Don't reformat neighbouring code, bump
  dependencies, or refactor adjacent modules inside a feature PR. If you spot a
  real problem outside your scope, open a separate issue — that's a genuinely
  useful contribution.

**If an issue is ambiguous, say so.** Underspecified issues are a maintainer
error, not yours. Ask on the issue and it gets rewritten.

---

## Working on it

**Branches**

```
feat/<issue-number>-short-description
fix/<issue-number>-short-description
test/<issue-number>-short-description
docs/<issue-number>-short-description
```

**Commits** follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(registry): implement invoice commitment storage
fix(settlement): reject replayed settlement ids
test(netting): cover unauthorized attestation caller
```

Keep them scoped. Prefer targeted `git add <path>` over `git add .` — it's the
simplest way to avoid sweeping unrelated changes into a PR.

**Tests are required, not encouraged.** Your issue names its test cases. A PR
without them is incomplete by the issue's own definition. Include at least one
failure path — unauthorized caller, duplicate ID, invalid input.

---

## Before you open a PR

Run the gates locally. CI runs exactly these, so anything failing here fails
there:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo build --target wasm32-unknown-unknown --release
cargo test
```

Notes:

- `clippy` runs with `-D warnings`. Warnings are errors.
- If you need an `allow`, scope it to the specific item and comment why. Never
  blanket-allow at crate level.
- Formatting is not negotiable — `cargo fmt` before every commit.

**A PR with red CI does not get review.** This isn't unfriendliness; it keeps
review attention on work that's actually ready.

---

## Opening the PR

**Link the issue.** Include `Closes #<number>` in the description. GrantFox
tracks the link between your PR and the assigned issue — without it, your
contribution may not register.

**In the description, include:**

- what changed, in a sentence or two
- how you verified it (which tests, what you ran)
- anything you decided that the issue didn't specify
- anything you found but deliberately left alone

**What gets a PR closed:**

- CI failing with no attempt to fix it
- work outside the issue's stated scope
- no tests where the issue named test cases
- cosmetic-only changes unconnected to an assigned issue — typo fixes,
  whitespace, comment reflows. Genuine documentation improvements are welcome
  as their own scoped issues.

---

## Review

You'll get a response promptly, and it'll be specific. If something's missing,
the comment will say what and where rather than gesturing at it.

If your submission is close, expect guidance to merge rather than a close and
re-open. Iteration is normal — the first PR to a codebase you've just met is
rarely perfect, and that's fine.

Once you've merged something, you're welcome to take harder issues in the same
module. Someone who implements the invoice status machine is the natural person
for settlement binding, and building depth in one area beats scattering across
unrelated tickets.

---

## Repo conventions

**Error codes.** Soroban assigns each error variant an integer. Each contract
owns a numbered range, documented at the top of its `errors.rs`. Don't reuse a
code across contracts; extend your contract's range.

**Storage and TTL.** Soroban entries expire. Any write to persistent storage
must bump the TTL. This is a common source of bugs — if your issue touches
storage, its acceptance criteria will say so explicitly.

**No custody, ever.** Sotto never holds spending keys. Any code path implying
custody is a design error. If an issue seems to require one, stop and ask.

**No secrets in the repo.** No `.env`, no keys, no seeds — not even testnet
ones.

---

## Code of conduct

Be decent to people. Assume good faith, disagree about code rather than about
each other, and remember that someone asking a basic question is someone who
decided to spend their evening on this project.

See [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md).

---

## Questions

Open an issue with the `question` label, or ask on the issue you're working on.
There are no stupid questions about a codebase built on a Developer Preview
primitive that shipped weeks ago.
