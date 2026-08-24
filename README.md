# Stellar Notes DApp

**Stellar Notes DApp** is a decentralized, immutable note-taking smart contract system built on the Stellar blockchain using the Soroban SDK. It provides a secure, transparent, and tamper-proof platform for storing personal and organizational notes directly on-chain, eliminating dependence on centralized databases and cloud providers.

---

## 🌟 Key Features

- 📝 **Simple Note Creation:** Store notes with titles and text content using a single contract invocation.
- 🔍 **Efficient Data Retrieval:** Fetch all active notes in a single query formatted for easy web frontend integration.
- 🗑️ **Secure Deletion:** Remove specific notes by unique ID to manage on-chain storage cleanly.
- 🛡️ **Data Sovereignty:** All operations are recorded transparently and immutably on the Stellar ledger.
- ⚡ **Low Friction:** Powered by Stellar’s high execution speed and sub-cent transaction costs.

---

## 📜 Contract Details

- **Deployed Network:** Stellar Testnet
- **Contract ID:** `CBZN65CBQ5AI4WV3LQXHR54ANT65EDNAODGLKBV4RLG4TIDBCFIPRFUT`
- **Initial Contract Address:** `CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M`

### 🔗 Blockchain Links & Explorers

- **Stellar Expert Transaction Explorer:**  
  [View Transaction on Stellar Expert](https://stellar.expert/explorer/testnet/tx/6b2c8372c6a265156bbbe0119a1487c230e71163751bbba12ce0dfefbcc45989)
- **Stellar Lab Explorer:**  
  [Interact via Stellar Lab](https://lab.stellar.org/r/testnet/contract/CBZN65CBQ5AI4WV3LQXHR54ANT65EDNAODGLKBV4RLG4TIDBCFIPRFUT)

---

## 🚀 Smart Contract Functions

The Soroban smart contract provides three main entry points:

1. **`create_note(env: Env, title: String, content: String) -> u64`**
   - Assigns an auto-incrementing ID to a new note.
   - Saves the title, body content, and metadata to instance storage.

2. **`get_notes(env: Env) -> Vec<Note>`**
   - Retrieves a list of all active notes stored in contract memory.

3. **`delete_note(env: Env, id: u64)`**
   - Removes the specified note ID from contract storage.

---

## 🛠️ Prerequisites & Setup

### Requirements

- **Rust Toolchain:** `1.71.0` or higher
- **Wasm Target:** `wasm32-unknown-unknown`
- **Stellar CLI:** Latest version (`cargo install --locked stellar-cli --features opt`)

### 1. Build Contract

```bash
# Compile the contract Wasm binary
stellar contract build

**Stellar Notes DApp** - Securing Your Thoughts on the Blockchain

✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/6b2c8372c6a265156bbbe0119a1487c230e71163751bbba12ce0dfefbcc45989
🔗 https://lab.stellar.org/r/testnet/contract/CBZN65CBQ5AI4WV3LQXHR54ANT65EDNAODGLKBV4RLG4TIDBCFIPRFUT
✅ Deployed!
CBZN65CBQ5AI4WV3LQXHR54ANT65EDNAODGLKBV4RLG4TIDBCFIPRFUT
