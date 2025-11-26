# SuiForge Modules Library

Battle-tested, reusable Move modules for common smart contract patterns on Sui.

## Available Modules

### AccessControl
Role-based access control for managing permissions.

```move
use suiforge::access_control::{Self, Role, AccessControl};
```

### Pausable
Emergency stop mechanism for contracts.

```move
use suiforge::pausable::{Self, Pausable};
```

### Ownable
Simple ownership management.

```move
use suiforge::ownable::{Self, Ownable};
```

### TokenUtils
Utilities for fungible tokens.

```move
use suiforge::token_utils;
```

### NFTUtils
Utilities for non-fungible tokens.

```move
use suiforge::nft_utils;
```

### Vault
Secure asset storage and management.

```move
use suiforge::vault::{Self, Vault};
```

### Escrow
Two-party escrow system.

```move
use suiforge::escrow::{Self, Escrow};
```

### PaymentSplitter
Revenue distribution among multiple parties.

```move
use suiforge::payment_splitter::{Self, PaymentSplitter};
```

## Installation

Add to your `Move.toml`:

```toml
[dependencies]
SuiForge = { git = "https://github.com/yourusername/suiforge", subdir = "modules" }
```

## Usage Example

```move
module my_project::my_contract {
    use suiforge::access_control::{Self, AccessControl};
    use suiforge::pausable::{Self, Pausable};
    
    public struct MyContract has key {
        id: UID,
        access_control: AccessControl,
        pausable: Pausable,
    }
    
    public fun create(ctx: &mut TxContext) {
        let contract = MyContract {
            id: object::new(ctx),
            access_control: access_control::new(ctx),
            pausable: pausable::new(),
        };
        transfer::share_object(contract);
    }
}
```

## Contributing

Contributions are welcome! Please ensure all modules:
- Follow Sui Move best practices
- Include comprehensive tests
- Have clear documentation
- Are gas-optimized

## License

MIT
