# Sotto

**Confidential B2B settlement on Stellar.**

Invoice-to-settlement infrastructure built on Stellar Confidential Tokens. Two
businesses settle in USDC on a public ledger without publishing what they paid
each other — while their auditor, and only their auditor, sees everything.

---

## 1. The problem

Cross-border B2B payment is the thing Stellar is commercially best at. Anchors,
SEP-24 on/off ramps, five-second finality, sub-cent fees. The rails work.

Businesses still won't use them at scale, for a reason that has nothing to do
with the rails.

**A public ledger publishes your commercial relationships.**

If a manufacturer settles with its suppliers on Stellar today, anyone with a
block explorer can read:

- who its suppliers are, and in what order of importance
- what it pays per shipment, and therefore its input costs
- its total volume, and therefore its revenue within a rounding error
- when its balances dip, and therefore when it is cash-constrained
- when a supplier relationship starts or stops

A competitor gets a live feed of your margins. A supplier learns what you pay
their rival. An acquirer sees your working capital before diligence. In
traditional banking this information is confidential by default; on-chain it is
public by default. No treasurer signs off on that trade, no matter how good the
settlement speed is.

This is the institutional privacy paradox: the transparency that makes a public
chain trustworthy is the same property that makes it commercially unusable for
the businesses it is trying to serve.

The naive fix — hide everything — fails for the opposite reason. A business that
cannot show its auditor, its regulator, or its counterparty what happened has
traded one unusable system for another. Full anonymity is not a feature for a
company with filing obligations. It is a liability.

**What is actually needed is selective confidentiality:** amounts hidden from
the public, fully visible to the parties and to a designated auditor, with
disclosure provable on demand.

---

## 2. What Sotto is

Sotto is the settlement application layer on top of Stellar's Confidential
Tokens primitive.

Stellar shipped the cryptographic substrate: Confidential Tokens (OpenZeppelin
contract suite + Nethermind UltraHonk verifier) wrap any SEP-41 token so that
balances and transfer amounts are hidden while sender and recipient addresses
stay visible. That is precisely the right privacy shape for B2B — you know your
counterparty, the world doesn't know your numbers.

But a wrapper contract is not a settlement system. Between "I can move a hidden
amount" and "my accounts payable runs on this" sits everything a finance team
actually needs:

| Primitive gives you | Sotto adds |
|---|---|
| A confidential transfer | An invoice it settles against |
| A hidden balance | A reconciled ledger position |
| One payment | Multilateral netting across many |
| An auditor view key | An auditor workflow and export |
| A proof of one transaction | A period-close disclosure pack |

**Sotto is the layer that turns a confidential transfer into a settled invoice.**

### Scope, stated honestly

Sotto does **not** invent cryptography. It does not fork the Confidential Token
contract, write new circuits, or compete with OpenZeppelin or Nethermind. It
consumes their primitives as a dependency and builds the missing application on
top. Any circuit-level improvement upstream is a free upgrade for Sotto.

Sotto is also **not** a mixer, and not a privacy pool. Counterparty addresses
are public by design. Sotto hides amounts, not relationships. This is a
deliberate architectural choice, made at the start, because the choice made at
the start decides what the system can do later.

---

## 3. How it works

### 3.1 The settlement lifecycle

```
   Supplier                    Sotto                      Buyer
      │                          │                          │
      │  1. issue invoice ──────►│                          │
      │     (terms, amount,      │  2. notify ─────────────►│
      │      currency)           │                          │
      │                          │◄──── 3. approve ─────────│
      │                          │                          │
      │                          │  4. net open positions   │
      │                          │     across all invoices  │
      │                          │     between these two    │
      │                          │                          │
      │                          │◄──── 5. settle net ──────│
      │                          │      (confidential       │
      │                          │       transfer)          │
      │                          │                          │
      │◄─ 6. settlement proof ───│──── 6. settlement proof ►│
      │                          │                          │
      │                          │  7. auditor view key     │
      │                          │     sees all of it       │
      │                          ▼                          │
      │                     ┌─────────┐                     │
      └────────────────────►│ Auditor │◄────────────────────┘
                            └─────────┘
```

### 3.2 Architecture

Four layers, only the top two of which Sotto owns:

```
┌───────────────────────────────────────────────────────┐
│  sotto-console        Treasurer UI, auditor UI,       │  ← Sotto
│                       invoice inbox, disclosure view  │
├───────────────────────────────────────────────────────┤
│  sotto-core           Netting engine, proof service,  │  ← Sotto
│                       reconciliation, connectors      │
├───────────────────────────────────────────────────────┤
│  sotto-contracts      Invoice registry, settlement    │  ← Sotto
│                       orchestrator, policy adapter    │
├───────────────────────────────────────────────────────┤
│  Confidential Token   OpenZeppelin suite + Nethermind │  ← dependency
│  + UltraHonk verifier UltraHonk verifier              │
├───────────────────────────────────────────────────────┤
│  Stellar base ledger  SEP-41 / SAC, BN254, BLS12-381, │  ← protocol
│                       Poseidon (P25 X-Ray / P26)      │
└───────────────────────────────────────────────────────┘
```

### 3.3 What each layer does

**`sotto-contracts` — Soroban, Rust**

On-chain state that must be trustless and auditable.

- **Invoice registry.** Commitments to invoice terms. The existence and status
  of an invoice is on-chain; the amount is not. Prevents either party from
  later disputing that an invoice existed or what it referenced.
- **Settlement orchestrator.** Binds a confidential transfer to the specific
  invoice(s) it discharges, so a payment cannot be silently re-attributed.
- **Netting attestation.** Records that N invoices were netted to one transfer,
  with a commitment to the netting set, so the compression is verifiable
  without revealing the components.
- **Policy adapter.** Plugs into the Confidential Token compliance policy engine
  so allow-list / block-list identity registries govern who may settle.

**`sotto-core` — backend service**

Everything that must not be on-chain, because it is either private, expensive,
or off-chain by nature.

- **Netting engine.** Bilateral first, multilateral later. Ten invoices between
  two parties become one transfer — cheaper, faster, and a smaller information
  surface.
- **Proof service.** Generates the Noir witnesses and UltraHonk proofs for
  confidential transfers. Runs in the client's trust domain; Sotto never holds
  spending keys.
- **Reconciliation.** Matches settled transfers back to invoices and produces
  the ledger entries a finance team can actually import.
- **Disclosure service.** Builds selective-disclosure packs — prove to a
  specific counterparty or auditor that a specific settlement occurred, without
  exposing anything else.
- **Connectors.** Invoice ingest from the systems businesses already use.

**`sotto-console` — frontend**

Three roles, three views, mirroring the Confidential Token demo's own role model:

- **Treasurer.** Invoice inbox, approvals, netting preview, settlement
  execution, position view.
- **Counterparty.** What was invoiced, what was approved, what settled.
- **Auditor.** View-key access to amounts and balances, period-close export,
  disclosure verification.

### 3.4 Trust model

| Party | Can see |
|---|---|
| Public / competitor | That two addresses transacted. Not amounts. Not balances. |
| Buyer & supplier | Their own invoices, amounts, and settlement history. |
| Designated auditor | All amounts and balances for assets in the wrapper. |
| Sotto (the service) | Nothing it isn't given. No custody, no spending keys. |
| Asset issuer | Retains SAC-level freeze, cascading into the wrapper. |

The last row matters and is deliberate. An enterprise treasurer wants the
issuer freeze to exist. It is a feature of this design, not a weakness.

---

## 4. Why Stellar, and why now

- **The primitive just landed.** Confidential Tokens are live on testnet as a
  Developer Preview, with contract and verifier audits underway. SDF is openly
  asking for design partners.
- **The cryptography is native and cheap.** BN254, BLS12-381, and
  Poseidon/Poseidon2 are protocol-level host functions after X-Ray (P25) and
  Yardstick (P26). A pairing check that costs tens of millions of Wasm
  instructions elsewhere is a single host call here.
- **The customers are already here.** Stellar's ecosystem is anchors, regulated
  payment rails, and B2B settlement companies across LatAm, Africa, and
  Southeast Asia. Sotto's users are the network's existing users.
- **SDF's stated privacy strategy is exactly this.** Open and transparent by
  default, privacy configurable and compliance-minded from the start, with
  view keys and disclosure mechanisms supporting oversight. Sotto is an
  implementation of that thesis, not an argument against it.
- **Nobody has built it.** The wrapper exists. The settlement application does
  not.

---

## 5. Roadmap

**Phase 1 — Foundation**
Repo scaffolding, contract interfaces, Confidential Token integration spike on
testnet, invoice registry contract, single bilateral settlement end to end.

**Phase 2 — Netting**
Bilateral netting engine, netting attestation contract, reconciliation output,
treasurer console.

**Phase 3 — Compliance surface**
Auditor role and view-key workflow, selective-disclosure packs, policy adapter
wired to identity registries, period-close export.

**Phase 4 — Adoption**
Connectors for real invoice sources, multilateral netting, design-partner
pilot, mainnet readiness pending Confidential Token GA.

---

## 6. Open questions

Stated plainly, because a project doc that pretends it has no unknowns is not
credible.

- Confidential Tokens are testnet-only and not mainnet-approved. Phase 4 depends
  on a timeline Sotto does not control.
- Proof generation cost and latency for realistic batch sizes is unmeasured.
  Phase 1 must produce numbers before Phase 2 designs around them.
- Multilateral netting across three or more parties without revealing the
  bilateral components is an open design problem, not a solved one.
- Soroban state archival: the invoice registry needs a TTL and rent strategy
  that survives an inactive quarter.
- Whether the auditor role should be a single key or a threshold is unresolved.

---

## 7. Contributing

Sotto is open source and built in public across three repositories. Issues are
scoped to be completable independently, with clear acceptance criteria and test
requirements. See `CONTRIBUTING.md` in each repo.
