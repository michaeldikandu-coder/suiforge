module suiforge::ownable {
    use sui::tx_context::{Self, TxContext};

    /// Ownership management
    public struct Ownable has store, copy, drop {
        owner: address,
    }

    /// Error codes
    const ENotOwner: u64 = 1;

    /// Create a new Ownable instance
    public fun new(ctx: &TxContext): Ownable {
        Ownable {
            owner: tx_context::sender(ctx),
        }
    }

    /// Get the owner address
    public fun owner(o: &Ownable): address {
        o.owner
    }

    /// Check if address is owner
    public fun is_owner(o: &Ownable, account: address): bool {
        o.owner == account
    }

    /// Require that sender is owner
    public fun require_owner(o: &Ownable, ctx: &TxContext) {
        assert!(tx_context::sender(ctx) == o.owner, ENotOwner);
    }

    /// Transfer ownership
    public fun transfer_ownership(o: &mut Ownable, new_owner: address, ctx: &TxContext) {
        require_owner(o, ctx);
        o.owner = new_owner;
    }

    /// Renounce ownership (set to zero address)
    public fun renounce_ownership(o: &mut Ownable, ctx: &TxContext) {
        require_owner(o, ctx);
        o.owner = @0x0;
    }
}
