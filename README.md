# Halopot

A compact Solana program that pools user SOL deposits into a raffle/prize mechanism while earning yield via Marinade (liquid staking). Users deposit SOL to receive tickets, an admin stakes pooled SOL into Marinade for yield, and winners are selected from ticket holders to claim part of the pool.

---

## What Halopot is

Halopot is a small, practical on-chain application demonstrating how to:
- Pool user deposits in SOL, representing those deposits as tickets,
- Stake pooled SOL through Marinade (mSOL) to earn yield while maintaining liquidity with Marinade’s liquid unstaking, and
- Run a simple raffle-style prize flow (deposit → pick winner → claim prize → withdraw).

It is intentionally minimal so it can be used as a learning project or bootstrapped into a more feature-rich product.

---

## Architecture
<img width="1469" height="753" alt="halopot-arc" src="https://github.com/user-attachments/assets/11330352-cff2-4d4a-9a83-ae5a9dc86f4f" />

- Program (Rust + Anchor): All on-chain logic lives in `programs/escrow`.
  - `instructions/` contains handlers like `deposit`, `withdraw`, `stake` and `unstake`.
  - `states/` defines on-chain state (e.g., `PoolState`, ticket data).
- Integration with Marinade: the program uses Marinade CPI (`marinade-cpi`) to perform liquid stake and liquid un-stake operations.
- TypeScript client & IDL: the generated `idl/` and `types/` make it easy to interact with the program from off-chain code.

Diagram: see the architecture image in the repository for a visual overview.

---

## Tech stack

- On-chain: Rust, Anchor (Anchor framework), `marinade-cpi` for Marinade integration
- Off-chain / tooling: TypeScript, Node, pnpm/npm
- Dev tooling: Solana CLI, `solana-test-validator`, Anchor CLI

---

## Local development & testing

Prerequisites:
- Rust (use project's `rust-toolchain.toml`), + `cargo`
- Anchor CLI (install per Anchor docs), and Solana CLI (`solana`)
- Node.js + pnpm or npm

Quickstart:

1. Clone and install deps

```bash
git clone <repo-url>
cd halopot
pnpm install # or npm install
```

2. Run a local validator (optional; `anchor test` will spawn a test validator automatically):

```bash
solana-test-validator --reset
# in another terminal
anchor test   # runs tests and builds the program
```

3. Build program

```bash
anchor build
```

4. Deploy (localnet)

```bash
anchor deploy --provider.cluster localnet
# or run migrations: `anchor test` usually handles deploy for tests
```

5. Run tests

```bash
anchor test
# or run TypeScript tests if present
pnpm test
```

Notes:
- `migrations/deploy.ts` contains deploy logic for scripts you may want to adapt for integration and staging.
- Use the IDL in `idl/halopot.json` and the types in `types/halopot.ts` for building off-chain clients.

---

## How it works (brief)

1. Users call `deposit` (1 SOL per ticket in this implementation) — SOL is transferred into the program PDA.
2. Admin calls `stake` which performs a Marinade CPI deposit to convert SOL → mSOL that accrues yield.
3. Admin or program picks winners (`pick_winner`) and the winner can `claim_prize`.
4. To give users a way to exit, there is a `withdraw` flow (burn ticket, get SOL back) and the program provides a `liquid_unstake` CPI integration to redeem mSOL to SOL if needed.

---

## Contributing

Thanks for thinking about contributing! A few ground rules:

- Keep changes small and focused — make it easy to review.
- Add tests for behavior you change or add (we use Anchor tests; see `anchor test`).
- Follow existing code style: Rust + Anchor best-practices for on-chain code and TypeScript for off-chain tooling.

Suggested workflow:
1. Fork & branch from `main`.
2. Run tests locally and make sure all existing tests pass.
3. Open a PR with a clear description and include tests and/or screenshots where helpful.

---

## Notes & maintenance

- This project is intended for learning and prototyping. If you plan to run anything with real funds, review security, audits, and Marinade integration assumptions thoroughly.
- To update dependencies, update `Cargo.toml` and `package.json` then re-run builds and tests.

---

If you'd like, I can add an example script that uses the IDL + `@coral-xyz/anchor` to deposit/stake/unstake against a local validator, or add a `CONTRIBUTING.md` template. Want me to add either of those?
