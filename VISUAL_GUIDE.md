# SuiForge Visual Guide

Visual diagrams and flowcharts to understand SuiForge architecture and workflows.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER / DEVELOPER                         │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      SUIFORGE CLI (Rust)                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │   init   │  │  build   │  │   test   │  │  deploy  │        │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘        │
│       │             │              │             │               │
│  ┌────┴─────────────┴──────────────┴─────────────┴─────┐        │
│  │              Command Dispatcher                      │        │
│  └──────────────────────┬───────────────────────────────┘        │
│                         │                                         │
│  ┌──────────────────────▼───────────────────────────────┐        │
│  │              Core Services Layer                     │        │
│  │  ┌──────────────┐  ┌──────────────┐  ┌───────────┐ │        │
│  │  │   Config     │  │   Template   │  │    Sui    │ │        │
│  │  │  Management  │  │    Engine    │  │  Wrapper  │ │        │
│  │  └──────────────┘  └──────────────┘  └───────────┘ │        │
│  │  ┌──────────────┐  ┌──────────────┐  ┌───────────┐ │        │
│  │  │     Code     │  │    Error     │  │  Utilities│ │        │
│  │  │  Generator   │  │   Handler    │  │           │ │        │
│  │  └──────────────┘  └──────────────┘  └───────────┘ │        │
│  └──────────────────────┬───────────────────────────────┘        │
│                         │                                         │
└─────────────────────────┼─────────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
        ▼                 ▼                 ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   Sui CLI    │  │  File System │  │   Network    │
│  (External)  │  │   (Local)    │  │  (RPC/API)   │
└──────────────┘  └──────────────┘  └──────────────┘
```

## Command Flow

### Init Command Flow

```
User: suiforge init my-project --template nft
    │
    ▼
┌─────────────────────────┐
│  Parse CLI Arguments    │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│  Validate Template      │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│  Create Directory       │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│  Generate Files:        │
│  • Move.toml            │
│  • suiforge.config.json │
│  • sources/main.move    │
│  • tests/               │
│  • README.md            │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│  Initialize Git Repo    │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│  Display Success        │
│  Show Next Steps        │
└─────────────────────────┘
```

### Build → Test → Deploy Flow

```
┌─────────────────┐
│ suiforge build  │
└────────┬────────┘
         │
         ▼
┌─────────────────────────┐
│  Load Config            │
│  (suiforge.config.json) │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Call Sui CLI:          │
│  sui move build         │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Parse Output           │
│  Handle Errors          │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Display Results        │
└─────────────────────────┘
         │
         ▼
┌─────────────────┐
│ suiforge test   │
└────────┬────────┘
         │
         ▼
┌─────────────────────────┐
│  Run Move Tests         │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Show Test Results      │
└─────────────────────────┘
         │
         ▼
┌──────────────────────────┐
│ suiforge deploy devnet   │
└────────┬─────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Get Active Address     │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Build (if needed)      │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Publish Package        │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Save Deployment Info   │
│  (suiforge.lock.json)   │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Display Package ID     │
└─────────────────────────┘
```

## Project Structure

```
my-project/
│
├── Move.toml                    ← Move package configuration
│   ├── [package]
│   ├── [dependencies]
│   └── [addresses]
│
├── suiforge.config.json         ← SuiForge settings
│   ├── network
│   ├── build
│   ├── deploy
│   └── codegen
│
├── suiforge.lock.json           ← Deployment tracking (auto-generated)
│   ├── packageId
│   ├── objectIds
│   ├── network
│   └── timestamp
│
├── sources/                     ← Move source files
│   └── main.move
│       ├── module definition
│       ├── structs
│       ├── functions
│       └── tests
│
├── tests/                       ← Move test files
│   └── main_tests.move
│
├── scripts/                     ← Deployment scripts (optional)
│
├── build/                       ← Build artifacts (auto-generated)
│   ├── bytecode/
│   └── source_maps/
│
└── README.md                    ← Project documentation
```

## Template System

```
┌─────────────────────────────────────────────────────────┐
│                    Template Selection                    │
└───────────────────────┬─────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│    Basic     │ │     NFT      │ │    Token     │
├──────────────┤ ├──────────────┤ ├──────────────┤
│ • Simple     │ │ • Collection │ │ • Treasury   │
│   objects    │ │ • Minting    │ │ • Mint/Burn  │
│ • Transfer   │ │ • Transfer   │ │ • Supply     │
│ • Getters    │ │ • Events     │ │ • Metadata   │
└──────────────┘ └──────────────┘ └──────────────┘
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Marketplace  │ │     DeFi     │ │     Game     │
├──────────────┤ ├──────────────┤ ├──────────────┤
│ • Listings   │ │ • Vault      │ │ • Characters │
│ • Offers     │ │ • Staking    │ │ • Items      │
│ • Sales      │ │ • Rewards    │ │ • Progression│
│ • Royalties  │ │ • Liquidity  │ │ • Battles    │
└──────────────┘ └──────────────┘ └──────────────┘
```

## Module Library Architecture

```
┌─────────────────────────────────────────────────────────┐
│              SuiForge Modules Library                    │
└───────────────────────┬─────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│    Access    │ │   Pausable   │ │   Ownable    │
│   Control    │ │              │ │              │
├──────────────┤ ├──────────────┤ ├──────────────┤
│ • Roles      │ │ • Pause      │ │ • Owner      │
│ • Permissions│ │ • Unpause    │ │ • Transfer   │
│ • Admin      │ │ • Check      │ │ • Renounce   │
└──────────────┘ └──────────────┘ └──────────────┘
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│    Vault     │ │    Escrow    │ │   Payment    │
│              │ │              │ │   Splitter   │
├──────────────┤ ├──────────────┤ ├──────────────┤
│ • Deposit    │ │ • Create     │ │ • Shares     │
│ • Withdraw   │ │ • Release    │ │ • Distribute │
│ • Balance    │ │ • Refund     │ │ • Withdraw   │
└──────────────┘ └──────────────┘ └──────────────┘
```

## SDK Generation Flow

```
┌──────────────────────────┐
│ suiforge generate ts     │
└────────────┬─────────────┘
             │
             ▼
┌──────────────────────────┐
│  Analyze Move Modules    │
│  • Parse Move.toml       │
│  • Read source files     │
│  • Extract types         │
└────────────┬─────────────┘
             │
             ▼
┌──────────────────────────┐
│  Generate TypeScript     │
│  • package.json          │
│  • tsconfig.json         │
│  • src/index.ts          │
│  • src/types.ts          │
└────────────┬─────────────┘
             │
             ▼
┌──────────────────────────┐
│  Create Client Wrapper   │
│  • Transaction builders  │
│  • Type definitions      │
│  • Helper functions      │
└────────────┬─────────────┘
             │
             ▼
┌──────────────────────────┐
│  Write to Output Dir     │
│  ./sdk/typescript/       │
└──────────────────────────┘
```

## Deployment Workflow

```
Developer                SuiForge              Sui Network
    │                        │                      │
    │  suiforge deploy       │                      │
    ├───────────────────────>│                      │
    │                        │                      │
    │                        │  Get Active Address  │
    │                        ├─────────────────────>│
    │                        │<─────────────────────┤
    │                        │  0x123...            │
    │                        │                      │
    │                        │  Build Package       │
    │                        │  (if needed)         │
    │                        │                      │
    │                        │  Publish Package     │
    │                        ├─────────────────────>│
    │                        │                      │
    │                        │  Transaction Result  │
    │                        │<─────────────────────┤
    │                        │  Package ID: 0xabc...│
    │                        │                      │
    │                        │  Save Lock File      │
    │                        │  suiforge.lock.json  │
    │                        │                      │
    │  Success! Package ID   │                      │
    │<───────────────────────┤                      │
    │  0xabc...              │                      │
    │                        │                      │
```

## Configuration Hierarchy

```
┌─────────────────────────────────────────────────────────┐
│                  Configuration Sources                   │
└───────────────────────┬─────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│   Default    │ │    Config    │ │   CLI Args   │
│   Values     │ │     File     │ │  (Override)  │
├──────────────┤ ├──────────────┤ ├──────────────┤
│ • Networks   │ │ suiforge     │ │ --gas-budget │
│ • Gas budget │ │ .config.json │ │ --network    │
│ • Paths      │ │              │ │ --output     │
└──────────────┘ └──────────────┘ └──────────────┘
        │               │               │
        └───────────────┼───────────────┘
                        │
                        ▼
        ┌───────────────────────────┐
        │   Final Configuration     │
        │   (Merged with Priority)  │
        └───────────────────────────┘
```

## Error Handling Flow

```
┌──────────────────────────┐
│  User Command            │
└────────────┬─────────────┘
             │
             ▼
┌──────────────────────────┐
│  Try Execute             │
└────────────┬─────────────┘
             │
        ┌────┴────┐
        │         │
        ▼         ▼
    Success    Error
        │         │
        │         ▼
        │    ┌──────────────────────────┐
        │    │  Classify Error:         │
        │    │  • IO Error              │
        │    │  • Config Error          │
        │    │  • Build Error           │
        │    │  • Network Error         │
        │    └────────────┬─────────────┘
        │                 │
        │                 ▼
        │    ┌──────────────────────────┐
        │    │  Format Error Message    │
        │    │  • Add context           │
        │    │  • Suggest fix           │
        │    │  • Color output          │
        │    └────────────┬─────────────┘
        │                 │
        ▼                 ▼
┌──────────────────────────┐
│  Display to User         │
│  • Success: ✓ Green      │
│  • Error: ✗ Red          │
│  • Info: ℹ Blue          │
│  • Warning: ⚠ Yellow     │
└──────────────────────────┘
```

## Development Workflow

```
┌─────────────────────────────────────────────────────────┐
│                  Developer Workflow                      │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
        ┌───────────────────────────────┐
        │  1. suiforge init my-project  │
        └───────────────┬───────────────┘
                        │
                        ▼
        ┌───────────────────────────────┐
        │  2. Edit sources/main.move    │
        └───────────────┬───────────────┘
                        │
                        ▼
        ┌───────────────────────────────┐
        │  3. suiforge build            │
        └───────────────┬───────────────┘
                        │
                        ▼
        ┌───────────────────────────────┐
        │  4. suiforge test             │
        └───────────────┬───────────────┘
                        │
                   ┌────┴────┐
                   │         │
                   ▼         ▼
               Pass      Fail
                   │         │
                   │         └──> Fix & Repeat
                   │
                   ▼
        ┌───────────────────────────────┐
        │  5. suiforge deploy devnet    │
        └───────────────┬───────────────┘
                        │
                        ▼
        ┌───────────────────────────────┐
        │  6. suiforge generate ts      │
        └───────────────┬───────────────┘
                        │
                        ▼
        ┌───────────────────────────────┐
        │  7. Integrate with Frontend   │
        └───────────────────────────────┘
```

## Legend

```
┌──────────┐
│   Box    │  = Component or Process
└──────────┘

    │
    ▼         = Flow Direction

┌──────────┐
│  Bold    │  = User Action
└──────────┘

┌──────────┐
│ Dashed   │  = Optional Step
└──────────┘

    ├──>      = Branch/Choice

    <──>      = Bidirectional
```

---

These diagrams provide a visual understanding of SuiForge's architecture, workflows, and component interactions.
