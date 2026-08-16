use soroban_sdk::{Address, Env};

// ISSUE: [contracts] Implement admin role and upgrade authorization pattern
// CONTEXT: invoice-registry's TTL/rent strategy (item 6), policy-adapter's
//          AllowListRegistryAdapter and check_block_list (items 17, 18), and
//          any future upgrade entrypoint all assume an "admin-only" gate
//          exists without it being defined anywhere. Every one of those
//          ISSUE blocks references this pattern; none of them can be
//          implemented correctly until it exists in one place all of them
//          adopt, rather than each crate inventing its own admin concept.
// SCOPE:   assert_admin_only() — the shared test fixture every crate's
//          admin-gated entrypoint test suite calls to assert an
//          unauthorized caller is rejected and the admin succeeds; and, by
//          extension, the admin-role storage/require-auth pattern each
//          contract's production code must adopt to make this fixture
//          meaningful (a stored admin Address, a require_admin() check, and
//          a set_admin() rotation entrypoint that is itself admin-gated)
// ACCEPTANCE:
//   - a shared, reusable admin storage key and require-admin check exists
//     that every crate needing one (invoice-registry, policy-adapter, any
//     future upgrade entrypoint) can adopt without redefining its own
//     admin concept
//   - set_admin() rotates the admin and is itself admin-gated; the old
//     admin cannot call it after rotation, the new admin can
//   - this fixture drives a contract's real admin-gated entrypoint twice —
//     once as a non-admin (expect rejection) and once as the admin (expect
//     success) — generic enough to parametrize over any contract exposing
//     the pattern
//   - documents whether admin is a single key or a threshold (project doc
//     §6 leaves this open; this issue must resolve it, not defer it again)
// TESTS: non-admin rejected, admin succeeds, admin rotation, rotated admin
//        can still call, old admin cannot
// OUT OF SCOPE: wiring this into every individual entrypoint that should be
//               admin-gated — this issue delivers the pattern and its test
//               fixture; adopting it per-entrypoint is each of those
//               entrypoints' own issue (already scoped in their existing
//               ACCEPTANCE criteria) once this exists
// DIFFICULTY: medium
// LABELS: contracts, soroban, authorization
pub fn assert_admin_only(_env: &Env, _admin: &Address, _non_admin: &Address) {
    todo!()
}
