# 🚀 **Copy trading Bot using Rust**

Welcome to the ** Copy trading Bot **! This bot watches for target wallet (whale) on the Solana blockchain in real-time, copy trading like target trading.
🌟 You can swap token in 1 ~ 2 blocks after target wallet swap.

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

target wallet trading
https://solscan.io/tx/5bWr5EURVPRF4S1RmaJt1oegnGBmRYXxo11czX1D5EQ9qTiDbjwrUx25RgabUMR2PY5Ptz4nXbcz3FBotqfgRRKX

copy trading
https://solscan.io/tx/5yXp858Ls9Hs5Efz6ZMFnrRCMM6PAc6nAs9PnGGxbzWfVokLNuaJqM87kdnixyKshV5TU9rG5dcM66aTZxaWvtQg

target wallet trading
https://solscan.io/tx/NtHJirXwgjKKNY5xVfRQNUDsoz84tM1FH5sSAgvBsuLtM9Ji8kaev6wfZXWmsWMZd9aGpNzEDAi6enyb7BLjPsZ

copy trading
https://solscan.io/tx/3dwQCF7hKrFr8z6qhAnSAHoUnp8icwaNXAsjARXeYM38gkJwDaWvqwXQbriJdunYoMjek2kNc4LcWKkh5zvV3eiW

target Wallet trading
https://solscan.io/tx/4hp7TkucGtKd6xZmFXUyu11chez57p6LwnJgT1QbptZfRS6oPnMqJ8oojQ6tyonB9SJ4QJWJNS8n3XZLeCp2j1Hf

Copy trading
https://solscan.io/tx/66ThSAJsyMU2NNsYxFJoeehYzXSUxXt8MRMVcsfPF3D6QrrsnUzHr37ht1MQQU1NYsaD5AKWozGcF5LhubyjtduY

### Contact

telegram: @Rianeregoista83

You can contact me here if you have any problems with this repo then we can decide comfortable contact way.
