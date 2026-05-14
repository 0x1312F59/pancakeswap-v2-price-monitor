# PancakeSwap V2 Price Monitor (Rust)

[![Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Ethers-rs](https://img.shields.io/badge/Web3-Ethers--rs-blue)](https://github.com/gakonst/ethers-rs)
[![BSC](https://img.shields.io/badge/Network-BNB%20Smart%20Chain-yellow)](https://www.bnbchain.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

Real-time token price monitoring on **PancakeSwap V2** using **Rust + ethers-rs** over WebSocket RPC.  
The application continuously fetches swap quotes through the PancakeSwap router and displays live token prices in the terminal.

---

## Features

- Real-time PancakeSwap V2 price monitoring
- Uses direct on-chain `getAmountsOut()` router calls
- Supports any BEP-20 token
- WebSocket-based provider for low-latency updates
- Automatic token symbol detection via ERC20 contract
- Clean terminal output with timestamps
- Configurable input amount via `.env`
- Lightweight and fully async using Tokio

---

## Example Output

```bash
Monitoring PancakeSwap price...
--------------------------------
[21:44:01] 1 CAKE = 2.315842 USDT
[21:44:02] 1 CAKE = 2.315901 USDT
[21:44:03] 1 CAKE = 2.316155 USDT
```

---

## Tech Stack

| Component      | Description                         |
| -------------- | ----------------------------------- |
| Rust           | Main programming language           |
| Tokio          | Async runtime                       |
| ethers-rs      | Ethereum / BSC blockchain library   |
| PancakeSwap V2 | DEX router for swap quote retrieval |
| WebSocket RPC  | Real-time blockchain communication  |

---

## Project Structure

```text
.
├── src
│   └── main.rs
├── .env
├── Cargo.toml
└── README.md
```

---

## Installation

### 1. Clone Repository

```bash
git clone https://github.com/your-username/pancakeswap-price-monitor.git

cd pancakeswap-price-monitor
```

### 2. Install Dependencies

```bash
cargo build
```

---

## Configuration

Create a `.env` file in the project root:

```env
# WebSocket RPC URL
RPC_WSS=wss://bsc-ws-node.nariox.org:443

# Token contract address
TOKEN_IN=0x0E09FaBB73Bd3Ade0a17ECC321fD13a19e81cE82

# Amount for quote simulation
AMOUNT_IN=1
```

---

## Running

```bash
cargo run
```

---

## How It Works

The application calculates token prices using the following swap path:

```text
TOKEN → WBNB → USDT
```

Internally it calls PancakeSwap V2 Router:

```solidity
getAmountsOut(uint256 amountIn, address[] memory path)
```

This allows accurate live quote estimation directly from on-chain liquidity pools.

---

## Smart Contract Addresses

| Contract           | Address                                      |
| ------------------ | -------------------------------------------- |
| PancakeSwap Router | `0x10ED43C718714eb63d5aA57B78B54704E256024E` |
| WBNB               | `0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c` |
| Binance-Peg USDT   | `0x55d398326f99059fF775485246999027B3197955` |

---

## Dependencies

| Crate    | Purpose                     |
| -------- | --------------------------- |
| `ethers` | Blockchain interaction      |
| `tokio`  | Async runtime               |
| `dotenv` | Environment variable loader |
| `chrono` | Timestamp formatting        |
| `anyhow` | Error handling              |

---

## License

This project is licensed under the MIT License.

```

```
