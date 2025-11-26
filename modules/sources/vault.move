module suiforge::vault {
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::coin::{Self, Coin};
    use sui::balance::{Self, Balance};

    /// Secure vault for storing tokens
    public struct Vault<phantom T> has key {
        id: UID,
        balance: Balance<T>,
        owner: address,
    }

    /// Error codes
    const ENotOwner: u64 = 1;
    const EInsufficientBalance: u64 = 2;

    /// Create a new vault
    public fun create<T>(ctx: &mut TxContext) {
        let vault = Vault<T> {
            id: object::new(ctx),
            balance: balance::zero(),
            owner: tx_context::sender(ctx),
        };
        transfer::share_object(vault);
    }

    /// Deposit tokens into vault
    public fun deposit<T>(vault: &mut Vault<T>, coin: Coin<T>) {
        let balance = coin::into_balance(coin);
        balance::join(&mut vault.balance, balance);
    }

    /// Withdraw tokens from vault
    public fun withdraw<T>(
        vault: &mut Vault<T>,
        amount: u64,
        ctx: &mut TxContext
    ): Coin<T> {
        assert!(tx_context::sender(ctx) == vault.owner, ENotOwner);
        assert!(balance::value(&vault.balance) >= amount, EInsufficientBalance);
        
        let withdrawn = balance::split(&mut vault.balance, amount);
        coin::from_balance(withdrawn, ctx)
    }

    /// Get vault balance
    public fun balance<T>(vault: &Vault<T>): u64 {
        balance::value(&vault.balance)
    }

    /// Get vault owner
    public fun owner<T>(vault: &Vault<T>): address {
        vault.owner
    }

    /// Transfer vault ownership
    public fun transfer_ownership<T>(
        vault: &mut Vault<T>,
        new_owner: address,
        ctx: &TxContext
    ) {
        assert!(tx_context::sender(ctx) == vault.owner, ENotOwner);
        vault.owner = new_owner;
    }
}
