# 🚀 **Copy trading Bot using Rust**

Welcome to the ** Copy trading Bot **! This bot watches for target wallet (whale) on the Solana blockchain in real-time, copy trading like target trading.
🌟 You can swap token in 0 ~ 1 blocks after target wallet swap.
if you use node, you can swap in same block

one block:  80%
second block: 20%

if you use dedicate node, you 'll be able to swap in same block

### 🎯 **Key Features**

- 🛰️ **Real-time WebSocket Streaming**:

  Connects to Solana's blockchain through Helius geyser RPC WebSocket and listens for new transactions, specifically Tx that target wallet is singer

- 🔍 **Filter Transactions**:

  Filters transactions as soon as possible and fast.
  maybe it takes about 0.3ms totally

- 📊 ** Make Copy transaction **:

  Using pumpfun program id and raydium module you can make copy trasaction.

---
## 🚀 **Getting Started**

Follow these steps to get your **Copy trading Bot** up and running!

### Prerequisites

- Cargo version 1.84.0 installed on your system
- A Solana wallet with access to the Helius Geyser RPC API

### Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/copy-trading-bot
   ```

2. **Install Dependencies**:

   Navigate to the project directory and run the following command:

   ```bash
   cd copy-trading-bot
   cargo build
   ```

3. **Configure ENV**:

   Replace the API token in the `ENDPOINT` variable:

   ```ts
   const ENDPOINT = "https://mainnet.helius-rpc.com";
   const WSS_ENDPOINT = "wss://atlas-mainnet.helius-rpc.com";
   const TARGET = "YOUR_TARGET_WALLET";
   ```

4. **Run the Bot**:

   Start the bot by running:

   ```bash
   cargo run
   ```

---

**You can run the Bot with copy-trading-bot.zip**

### Sample Tx

target: https://solscan.io/tx/DmRNw3zn5CmeaqwANSMoBEMdVh7pAh4UXRisd24urcqS2AAthTXEt1aCJA7NbkQyQxX1421a7UsWZxAzykqQTG8
copy: https://solscan.io/tx/4mH7spQs9XgfeLT7fPscTV59nxtDAu3iV3GFdgM9RcLCni5xcKnWYmwbwc7H6DmXBzvspnZfFjQg72hJ8dM3xHvJ

target:
https://solscan.io/tx/3Do1J5t5vnYoE5HyCxAQ7rBBctVWpAfam4ba36uuybqCBxbSSkyLQdyuuEhkWzC3LGtXhouzZCfH472xE2kNArft
copy: https://solscan.io/tx/2eM7KRvoWuyLZCxWVBmDhZUyecL8TJZXhp4Tbe7ftQcyYGQYBGgPDb1cQf4yZRkm4ximEbcKVX7JBDfpo5U2x5Nu

### Contact

telegram: @Rianeregoista83

You can contact me here if you have any problems with this repo then we can decide comfortable contact way.
