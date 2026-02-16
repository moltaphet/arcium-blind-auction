# 🔨 Arcium Blind Auction: Confidential & Fair Price Discovery

**Preventing Bid Sniping and Market Manipulation through Secure Multi-Party Computation (MPC).**

**Arcium Blind Auction** is a decentralized "Sealed-Bid" protocol designed to ensure 100% fairness in on-chain auctions. By leveraging the **Arcium Network**, this engine allows participants to submit bids that remain mathematically obscured and confidential until the moment of resolution.

---

## 🛡️ The Problem: Transparency as a Vulnerability
In traditional on-chain auctions, transparency is often exploited. When all bids are public:
* **Bid Sniping**: Predatory bots monitor the mempool or the ledger to outbid participants in the final millisecond.
* **Collusion**: Participants can adjust their behavior based on others' revealed bids, leading to unfair pricing.
* **Information Leakage**: Strategic intentions are exposed to the entire market.

## 🚀 The Solution: Confidential Bidding via MPC
Our engine moves the bidding and resolution logic into a **Confidential Execution Layer**. 

### Core Features:
1. **Secret-Shared Bids**: Bids are never stored in plain text. They are split into secret shares and distributed across the Arcium cluster.
2. **Confidential Resolution**: The network executes a secure maximum-value comparison ($Bid_{i} > Bid_{j}$) without ever decrypting or revealing individual bid amounts.
3. **Guaranteed Integrity**: Only the winning Bidder ID and the final price are broadcasted, ensuring absolute privacy for all other participants.

---

## 🛠 Tech Stack & Architecture
* **Language**: Rust 🦀
* **Privacy Infrastructure**: Arcium MPC Cluster
* **Serialization**: Borsh (for secure, high-performance data handling)
* **Core Logic**: `src/auction.rs` (Implements the confidential sorting & winner selection)

---

## 🧪 Technical Validation
The engine includes a verified test suite to simulate a multi-bidder scenario and ensure the integrity of the confidential resolution.

### Run the Simulation:
```bash
cargo test -- --nocapture
```

```
Compiling arcium-blind-auction v0.1.0...
Running unittests src/main.rs...

running 1 test
MPC Resolution Complete.
Winner ID: 102 with the highest confidential bid!
test auction::tests::test_blind_auction_resolution ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
🛡 Security & Privacy Guarantees

. Zero-Knowledge State: No single node in the Arcium network can reconstruct any bidder's amount.

. Anti-Manipulation: Eliminates the possibility of last-minute sniping since the leading bid is always hidden.

. Trustless Price Discovery: Ensures the auction settles at the true market valuation.

🚀 Getting Started
Clone the repository: 
```
git clone https://github.com/your-username/arcium-blind-auction
```
Run tests: 
```
cargo test
```