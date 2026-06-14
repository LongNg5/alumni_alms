# alumni_alms

## Project Title
alumni_alms

## Project Description
alumni_alms is an on-chain alumni donation pool that lets graduates of a school pledge funds to support current students in need of financial aid. Each pledge and each aid request is recorded on Stellar so that the relationship between donors and recipients is transparent, tamper-proof, and easy to audit. The contract is fully storage-focused: it never moves XLM, it only keeps an immutable ledger of who gave, who asked, and which gifts have been matched.

## Project Vision
Build a long-running, institution-agnostic "alumni endowment" primitive for Stellar that any school, bootcamp, or community of practice can reuse. Over time the contract should evolve into a full alumni-scholarship rail where graduation cohorts can pool capital, students can apply for aid from anywhere, and the matching process is auditable by every donor and recipient without relying on a trusted intermediary.

## Key Features
- Alumni self-registration with graduation year stored on-chain for cohort-based reporting
- Donation recording with an attached message, an internal id, and a `matched` flag so a single pledge cannot be double-spent
- Student aid request workflow with a short reason and a `funded` flag
- `match_aid` function that lets the original donor link one of their donations to one open aid request
- Read-only helpers (`donor_count`, `donation_count`, `request_count`, `get_donation`, `get_request`, `open_donations`, `graduation_year`) that make it trivial to build a dashboard on top of the contract

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** education dApp — see `contracts/alumni_alms/src/lib.rs` for the full alumni_alms business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** CCZDXRD7ZCAA2V5YZAQHYR6LJECEJOZI2SWVYFA6XPCC7ALBAYHMOAOQ
- **Explorer template:** https://stellar.expert/explorer/testnet/tx/fc0d307b908cabeb910f6bf9ae82850d7f069587223dad57df66518c9578f9cc
- **Screenshot of deployed contract on Stellar Expert:**
  ![screenshot](https://ibb.co/Ngrz5mW8)


## Future Scope
- Add an admin-managed list of approved schools so a single deployment can serve multiple institutions
- Introduce recurring / streaming donations using Soroban's time-based primitives, letting alumni set up monthly pledges
- Layer in a lightweight reputation score for donors and recipients based on completed matches, useful for surfacing trusted alumni on a public leaderboard

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `alumni_alms` (education)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
