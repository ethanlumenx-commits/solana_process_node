# Solana Real-Time Trading Engine：从 Rust 新人到可求职项目

> 目标：从零开始，自己实现一个“Solana 实时交易数据处理 + DEX 解析 + 策略 + 交易执行”的工程。
>
> **不是照抄现有 Sniper Bot。**
>
> 你可以参考成熟项目的架构，但最终要能够自己解释每一层为什么存在、数据是什么、为什么这样并发、出了错误怎么办。

---

## 0. 最终项目到底要做什么？

先不要把目标理解成“做一个自动狙击机器人”。

我们的核心目标是：

> **构建一个 Rust 编写的 Solana 实时交易事件处理引擎。**

最终业务链路：

```text
                    Solana
                       │
                       ▼
              Yellowstone gRPC
                       │
                       ▼
              Transaction Stream
                       │
                       ▼
                 Dispatcher
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
       Parser       Parser       Parser
      Raydium      Pump.fun        ...
          │            │            │
          └────────────┼────────────┘
                       ▼
                  Trade Event
                       │
                       ▼
                  Strategy
                       │
             ┌─────────┴─────────┐
             ▼                   ▼
          Ignore                Buy/Sell
                                 │
                                 ▼
                        Transaction Builder
                                 │
                                 ▼
                           RPC / Jito
                                 │
                                 ▼
                              Solana
```

---

# 1. 你最终应该掌握什么？

把知识分成 5 层。

## 第一层：Rust 基础

必须会：

- ownership
- borrowing
- lifetime 基础
- struct
- enum
- trait
- generics
- Result / Option
- error handling
- iterator
- closure
- `Arc`
- `Mutex`
- `RwLock`

这一层不需要做到 Rust 专家。

目标是：

> 能看懂普通 Rust 后端代码，并且能自己写。

---

# 2. 第二层：Tokio / 异步并发

这是这个项目非常重要的一层。

需要掌握：

```text
async / await
tokio::spawn
mpsc
broadcast
oneshot
select!
JoinHandle
Mutex
RwLock
AtomicUsize
```

尤其理解：

```rust
let (tx, rx) = tokio::sync::mpsc::channel(1024);
```

以及：

```text
Producer
   │
   ▼
  tx
   │
   ▼
 channel
   │
   ▼
  rx
   │
   ▼
 Worker
```

你之前做 Binance WebSocket Aggregator 的经验会直接用在这里。

---

# 3. 第三层：Solana

不要一上来研究 MEV。

先搞懂：

```text
Account
Transaction
Message
Instruction
Program
Program ID
Signer
PDA
Token Account
Mint
ATA
Lamports
SPL Token
```

尤其理解：

```text
Transaction
    │
    ▼
Message
    │
    ├── Account Keys
    │
    └── Instructions
             │
             ▼
        Program ID
             │
             ▼
        Instruction Data
             │
             ▼
        Accounts
```

你最终必须能够看到一笔交易，然后回答：

> 这个交易调用了哪个 Program？

> 调用了什么 Instruction？

> 哪些账户参与？

> 输入 Token 是什么？

> 输出 Token 是什么？

> 数量是多少？

---

# 4. 第四层：Solana 数据获取

先理解三种方式。

## RPC

适合：

```text
getBlock
getTransaction
getAccountInfo
sendTransaction
```

特点：

```text
请求 → 响应
```

---

## Solana WebSocket

例如：

```text
logsSubscribe
accountSubscribe
programSubscribe
signatureSubscribe
```

特点：

```text
订阅
 ↓
实时事件
```

适合很多普通实时监听场景。

---

## Yellowstone gRPC

我们的项目重点。

理解：

```text
Validator
    ↓
Geyser
    ↓
Yellowstone gRPC
    ↓
你的 Rust 程序
```

它更接近 Solana 节点的数据流。

适合：

- 实时链上数据
- 高频数据处理
- DEX 监听
- Indexer
- Trading infrastructure
- MEV / Sniper
- Copy Trading

---

# 5. 第五层：交易执行

最后才学：

```text
Transaction Builder
       ↓
Instruction
       ↓
Transaction
       ↓
RPC
       ↓
Jito
```

你需要理解：

- recent blockhash
- signer
- instruction
- transaction message
- transaction signing
- send transaction
- confirmation
- retry
- failure handling

Jito 放在最后。

---

# 6. 项目目录

最终建议：

```text
solana-trading-engine/
│
├── Cargo.toml
├── README.md
├── .env.example
├── Dockerfile
├── docker-compose.yml
│
├── crates/
│   │
│   ├── app/
│   │   └── src/
│   │       └── main.rs
│   │
│   ├── config/
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── ingestion/
│   │   └── src/
│   │       ├── lib.rs
│   │       └── yellowstone.rs
│   │
│   ├── dispatcher/
│   │   └── src/
│   │       ├── lib.rs
│   │       └── dispatcher.rs
│   │
│   ├── parser/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── common.rs
│   │       ├── raydium.rs
│   │       └── pumpfun.rs
│   │
│   ├── event/
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── strategy/
│   │   └── src/
│   │       ├── lib.rs
│   │       └── copy_trade.rs
│   │
│   ├── execution/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── transaction_builder.rs
│   │       └── jito.rs
│   │
│   └── storage/
│       └── src/
│           ├── lib.rs
│           └── postgres.rs
│
├── tests/
│   ├── parser_tests.rs
│   └── integration_tests.rs
│
└── docs/
    ├── architecture.md
    ├── solana.md
    ├── yellowstone.md
    └── parser.md
```

**注意：**

你现在是新人，不要一开始就创建这么多 crate。

真正开始的时候只需要：

```text
src/
├── main.rs
├── config.rs
├── yellowstone.rs
├── dispatcher.rs
├── parser.rs
└── event.rs
```

等代码开始变大，再拆 workspace。

---

# 7. 第一阶段：先把 Rust 工程跑起来

## 功能

```text
Rust Project
    ↓
读取 .env
    ↓
启动 Tokio
    ↓
tracing 日志
    ↓
正常退出
```

目录：

```text
src/
├── main.rs
└── config.rs
```

学习：

- Cargo
- module
- crate
- `pub`
- `Result`
- `std::env`
- dotenv
- tracing

---

# 8. 第二阶段：连接 Solana RPC

不要马上上 Yellowstone。

先理解最普通的 Solana。

实现：

```text
get_latest_blockhash
get_slot
get_balance
get_transaction
```

目录：

```text
src/
├── main.rs
└── solana.rs
```

目标：

> 你知道 RPC 是什么，以及 Rust 程序如何和 Solana 节点通信。

---

# 9. 第三阶段：自己做一个 Solana WebSocket Listener

先不做交易。

实现：

```text
logsSubscribe
     ↓
收到日志
     ↓
打印 signature
```

例如：

```text
Solana
   ↓
WebSocket
   ↓
Rust
   ↓
logsSubscribe
   ↓
signature
```

学习：

- WebSocket
- JSON-RPC
- subscription
- async stream
- reconnect

这一阶段非常重要。

因为以后你看到 Yellowstone 时，知道它解决的到底是什么问题。

---

# 10. 第四阶段：Yellowstone gRPC

现在才开始：

```text
Solana Validator
       ↓
      Geyser
       ↓
Yellowstone gRPC
       ↓
Rust
```

你需要理解：

```text
tonic
protobuf
grpc
stream
SubscribeRequest
SubscribeUpdate
```

第一目标只有一个：

> **连接 Yellowstone，然后持续收到数据。**

不要解析交易。

不要做策略。

不要做 Sniper。

不要做 Jito。

---

# 11. 第五阶段：Subscribe

你需要实现：

```text
SubscribeRequest
       │
       ├── accounts
       ├── transactions
       ├── slots
       └── blocks
```

但是第一版只订阅：

```text
transactions
```

业务：

```text
Yellowstone
     ↓
Subscribe
     ↓
Transaction Update
     ↓
println!
```

此时你应该能够回答：

> Yellowstone 发给我的数据是什么？

---

# 12. 第六阶段：Transaction Receiver

不要在接收循环里干所有事情。

错误设计：

```text
receive
  ↓
parse
  ↓
database
  ↓
strategy
  ↓
send tx
```

因为任何一步变慢都会阻塞接收。

正确设计：

```text
Yellowstone
     ↓
Receiver
     ↓
mpsc
     ↓
Worker
```

例如：

```text
                 ┌──────────────┐
                 │ Yellowstone │
                 └──────┬───────┘
                        │
                        ▼
                 Transaction
                   Receiver
                        │
                        ▼
                    mpsc
                        │
              ┌─────────┼─────────┐
              ▼         ▼         ▼
           Worker 1  Worker 2  Worker 3
```

这和你之前的 Binance 项目非常类似。

---

# 13. 第七阶段：Dispatcher

现在实现：

```text
Receiver
   ↓
Dispatcher
   ↓
不同 Worker
```

例如：

```text
Program ID
    │
    ├── Raydium → Raydium Worker
    │
    ├── Pump.fun → Pump Worker
    │
    └── Other → Ignore
```

核心思想：

> Receiver 负责收数据。

> Dispatcher 负责分发。

> Worker 负责处理。

不要把三个职责写在一起。

---

# 14. 第八阶段：Transaction Parser

这是整个项目最重要的业务部分之一。

先不要解析 Raydium。

先做：

```text
Transaction
    ↓
Program IDs
    ↓
Instructions
```

实现：

```text
get_program_ids()
get_instructions()
get_accounts()
```

然后打印：

```text
signature
slot
program_id
instruction
accounts
```

目标：

> 你可以从一笔 Solana Transaction 中找出它调用了什么。

---

# 15. 第九阶段：先实现一个 DEX Parser

不要同时做：

```text
Raydium
Pump.fun
Orca
Jupiter
Meteora
```

只做一个。

建议：

> **先选一个你最容易获得真实交易样本的 DEX。**

Parser：

```text
Raw Transaction
      ↓
DEX Parser
      ↓
TradeEvent
```

---

# 16. TradeEvent 是什么？

这是整个系统非常重要的一层。

不要让 Strategy 直接理解 Solana 原始 Transaction。

定义统一事件：

```rust
struct TradeEvent {
    pub signature: String,
    pub slot: u64,
    pub trader: Pubkey,
    pub dex: Dex,
    pub side: Side,
    pub token_in: Pubkey,
    pub token_out: Pubkey,
    pub amount_in: u64,
    pub amount_out: u64,
}
```

于是：

```text
Raydium Transaction
        ↓
Raydium Parser
        ↓
TradeEvent
```

以及：

```text
Pump.fun Transaction
        ↓
Pump Parser
        ↓
TradeEvent
```

最后 Strategy 不需要知道：

> Raydium 的账户布局是什么。

它只需要知道：

```text
TradeEvent
```

这就是**分层设计**。

---

# 17. 第十阶段：Strategy

现在才开始做策略。

例如最简单：

```text
监听地址 A
       ↓
发现 A 买入 Token X
       ↓
TradeEvent
       ↓
Strategy
       ↓
Copy Trade
```

第一版甚至不要真的交易。

只打印：

```text
SMART MONEY BUY

wallet: xxx
token: xxx
amount: xxx
```

然后：

```text
Strategy
    ↓
Signal
```

例如：

```rust
struct TradeSignal {
    pub token: Pubkey,
    pub side: Side,
    pub amount: u64,
}
```

---

# 18. 第十一阶段：Transaction Builder

现在才进入真正的交易。

业务：

```text
TradeSignal
     ↓
Transaction Builder
     ↓
Instruction
     ↓
Message
     ↓
Transaction
```

你需要理解：

```text
Instruction
Message
Blockhash
Signer
Signature
Transaction
```

第一版：

> 构造交易，但不要发送。

然后：

> 在 Devnet / 安全环境测试。

---

# 19. 第十二阶段：发送交易

实现：

```text
Transaction
    ↓
RPC
    ↓
sendTransaction
    ↓
Signature
```

需要考虑：

```text
成功
失败
timeout
retry
blockhash expired
RPC error
```

不要一上来：

```rust
unwrap()
```

而应该学习：

```rust
Result<T, E>
```

以及错误分类。

---

# 20. 第十三阶段：Jito

最后再学。

理解：

```text
普通 RPC

Transaction
   ↓
RPC
   ↓
Validator
```

和：

```text
Jito

Transaction / Bundle
       ↓
     Jito
       ↓
 Validator
```

然后研究：

- bundle
- tip
- leader
- priority
- transaction landing
- latency

这一阶段才进入：

> Solana Trading Infrastructure

---

# 21. 第十四阶段：性能优化

功能跑通以后再优化。

观察：

```text
ingestion latency
parser latency
queue latency
execution latency
```

记录：

```text
received_at
parsed_at
strategy_at
built_at
sent_at
confirmed_at
```

然后计算：

```text
receive → parse
parse → strategy
strategy → build
build → send
send → confirm
```

这会让你的项目从：

> “能运行的 Demo”

变成：

> “有工程意识的实时系统”。

---

# 22. 第十五阶段：错误处理

必须处理：

```text
Yellowstone disconnect
gRPC error
RPC timeout
Parser error
Invalid transaction
Unknown program
Channel full
Database unavailable
Transaction failed
Blockhash expired
```

你的系统不能：

```rust
.unwrap()
.expect()
```

满天飞。

至少做到：

```text
错误分类
 ↓
tracing
 ↓
retry / ignore / shutdown
```

---

# 23. 第十六阶段：Storage

最后再接数据库。

不要让 PostgreSQL 阻塞实时处理。

错误：

```text
Transaction
 ↓
Parse
 ↓
INSERT PostgreSQL
 ↓
Strategy
```

更好的方式：

```text
Transaction
 ↓
Parse
 ↓
TradeEvent
 ↓
Strategy

同时

TradeEvent
 ↓
Storage Channel
 ↓
DB Worker
 ↓
PostgreSQL
```

也就是说：

> Trading path 和 storage path 分离。

---

# 24. 最终完整业务流

最终你应该能画出：

```text
                         Solana
                            │
                            ▼
                     Geyser / Validator
                            │
                            ▼
                   Yellowstone gRPC
                            │
                            ▼
                    Ingestion Layer
                            │
                            ▼
                         mpsc
                            │
                            ▼
                       Dispatcher
                            │
             ┌──────────────┼──────────────┐
             ▼              ▼              ▼
         Raydium          Pump.fun        Other
          Parser            Parser         Parser
             │              │              │
             └──────────────┼──────────────┘
                            ▼
                       TradeEvent
                            │
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
           Strategy      PostgreSQL      Redis
              │
              ▼
          TradeSignal
              │
              ▼
      Transaction Builder
              │
              ▼
        Signed Transaction
              │
        ┌─────┴─────┐
        ▼           ▼
       RPC         Jito
        │           │
        └─────┬─────┘
              ▼
           Solana
```

---

# 25. 你每一步应该达到什么程度？

| 阶段 | 能力 | 完成标准 |
|---|---|---|
| 1 | Rust | 能独立写基本 Rust |
| 2 | Tokio | 能写 async worker |
| 3 | Solana RPC | 能读取链上数据 |
| 4 | WebSocket | 能实时监听 |
| 5 | Yellowstone | 能接收 gRPC stream |
| 6 | Channel | 能做 receiver/worker |
| 7 | Dispatcher | 能按 Program 分发 |
| 8 | Parser | 能解析 Transaction |
| 9 | DEX | 能识别 Swap |
| 10 | TradeEvent | 能统一不同 DEX |
| 11 | Strategy | 能产生交易信号 |
| 12 | Builder | 能构造交易 |
| 13 | RPC | 能发送交易 |
| 14 | Jito | 能理解低延迟执行 |
| 15 | Performance | 能测 latency |
| 16 | Production | 能处理错误/重连 |

---

# 26. 每一阶段不要贪多

你的学习方法应该固定成：

```text
① 学一个概念
      ↓
② 写一个最小 Demo
      ↓
③ 放进项目
      ↓
④ 理解为什么这么设计
      ↓
⑤ 重构
```

例如学 mpsc：

不要：

> 看 Tokio 教程 3 小时。

而是：

```text
producer
   ↓
mpsc
   ↓
consumer
```

自己写出来。

然后：

```text
Solana Receiver
   ↓
mpsc
   ↓
Parser Worker
```

再把它放进项目。

---

# 27. 不要一开始学这些

下面这些先不要碰：

```text
❌ Shred
❌ Jito
❌ MEV
❌ Bundle
❌ 多 DEX
❌ 多线程极限优化
❌ lock-free
❌ SIMD
❌ unsafe
❌ Validator 源码
❌ 自己跑 Validator
```

不是它们不重要。

而是：

> **你现在学这些，会把认知负担直接拉爆。**

你的路线应该是：

```text
Rust
 ↓
Tokio
 ↓
Solana
 ↓
WebSocket
 ↓
gRPC
 ↓
Yellowstone
 ↓
Transaction
 ↓
Parser
 ↓
TradeEvent
 ↓
Strategy
 ↓
Execution
 ↓
Jito
```

---

# 28. 这个项目真正的求职价值

最终简历不要写：

> 做了一个 Solana Sniper Bot。

更好的表达：

> **基于 Rust/Tokio 构建 Solana 实时交易数据处理引擎，使用 Yellowstone gRPC 接收链上 Transaction Stream，通过异步 Dispatcher/Worker Pipeline 进行交易分发与解析，抽象统一 TradeEvent，并实现 DEX Swap 识别、策略信号生成及交易构建与执行。**

如果你进一步完成：

- reconnect
- retry
- backpressure
- tracing
- benchmark
- Docker
- PostgreSQL
- Redis
- Jito
- 测试

项目的工程含量会明显提升。

---

# 29. 面试时你必须能回答的问题

做完以后，至少准备这些。

### Rust

> 为什么这里用 Arc？

> Mutex 和 RwLock 怎么选？

> mpsc 为什么适合这里？

> `spawn` 做了什么？

> async task 和 OS thread 有什么区别？

> AtomicUsize 为什么存在？

---

### Tokio

> channel 满了怎么办？

> worker panic 怎么处理？

> receiver disconnect 怎么处理？

> 如何做 backpressure？

> 如何优雅 shutdown？

---

### Solana

> Transaction 和 Instruction 什么关系？

> Program ID 是什么？

> PDA 是什么？

> SPL Token 和 SOL 有什么区别？

---

### Yellowstone

> Yellowstone 是什么？

> 为什么不用 Solana WebSocket？

> Geyser 在这里是什么角色？

> SubscribeRequest 是什么？

> Stream 怎么处理？

> gRPC 断开怎么办？

---

### Parser

> 怎么判断一笔交易是不是 Raydium Swap？

> 如何找到 Program ID？

> Instruction accounts 是什么？

> 如何把原始交易转换成 TradeEvent？

---

### 架构

> 为什么 Receiver 和 Parser 分开？

> 为什么要 Dispatcher？

> 为什么数据库不能直接放在交易处理链路里？

> 如何保证一个慢 Worker 不影响整个系统？

---

### Trading

> 怎么构造 Transaction？

> Blockhash 是什么？

> Transaction 为什么会失败？

> RPC 和 Jito 有什么区别？

---

# 30. 你现在真正的第一步

**不要开始写 Sniper。**

第一周只做：

```text
Rust
 ↓
Tokio
 ↓
Solana RPC
 ↓
Solana WebSocket
```

然后：

```text
第二阶段

Yellowstone
 ↓
Subscribe
 ↓
Receive Transaction
```

再往后：

```text
Transaction
 ↓
Dispatcher
 ↓
Parser
 ↓
TradeEvent
```

---

# 31. 最重要的一条原则

这个项目不是为了让你：

> “把 GitHub 上那个项目复制出来。”

而是为了让你最终达到：

> **看到那个项目的代码，你能够知道它在整个系统中处于哪一层。**

比如看到：

```rust
let (tx, rx) = mpsc::channel(1024);
```

你知道：

> 这是数据管道。

看到：

```rust
tokio::spawn(...)
```

你知道：

> 这是并发 worker。

看到：

```rust
AtomicUsize
```

你知道：

> 这里可能存在共享状态/并发索引分配。

看到：

```rust
SubscribeRequest
```

你知道：

> 这是 Yellowstone 数据订阅。

看到：

```rust
Instruction
```

你知道：

> 正在从 Transaction 往 Solana Program 调用层深入。

看到：

```rust
TradeEvent
```

你知道：

> Raw blockchain data 已经被抽象成业务事件。

看到：

```text
Transaction Builder
```

你知道：

> 已经从“读链”进入“写链”。

---

# 32. 最终目标

你的学习路线不是：

```text
Rust → 做一个项目 → 找工作
```

而是：

```text
Rust
  ↓
异步并发
  ↓
实时数据系统
  ↓
Solana
  ↓
链上数据解析
  ↓
Trading Infrastructure
  ↓
自己设计系统
  ↓
能够解释架构
  ↓
能够独立 Debug
  ↓
能够面试
```

**当你能够从零把这个项目的核心链路重新写出来，并且不依赖原项目逐行照抄时，这个项目就足够成为你 Rust + Solana 求职的核心作品之一。**
