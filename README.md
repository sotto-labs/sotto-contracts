# sotto-contracts

Soroban smart contracts for **Sotto** — confidential B2B settlement on Stellar.

Two businesses settle in USDC on a public ledger without publishing what they
paid each other, while a designated auditor sees everything. Built on Stellar
Confidential Tokens.

📄 **[Full project specification →](./SOTTO_PROJECT_DOC.md)**

## What's here

| Contract | Role |
|---|---|
| Invoice registry | Commitments to invoice terms and status |
| Settlement orchestrator | Binds confidential transfers to invoices |
| Netting attestation | Verifiable compression of many invoices into one transfer |
| Policy adapter | Hooks into the Confidential Token compliance policy engine |

## Status

Early development. Contracts are scaffolded with issues open for implementation.

## Stack

Rust · Soroban SDK · `wasm32-unknown-unknown`

## License

MIT
