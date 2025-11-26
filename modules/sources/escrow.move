module suiforge::escrow {
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::coin::{Self, Coin};
    use sui::balance::{Self, Balance};

    /// Two-party escrow system
    public struct Escrow<phantom T> has key {
        id: UID,
        sender: address,
        recipient: address,
        balance: Balance<T>,
        released: bool,
    }

    /// Error codes
    const ENotRecipient: u64 = 1;
    const ENotSender: u64 = 2;
    const EAlreadyReleased: u64 = 3;
    const ENotReleased: u64 = 4;

    /// Create a new escrow
    public fun create<T>(
        recipient: address,
        coin: Coin<T>,
        ctx: &mut TxContext
    ) {
        let escrow = Escrow<T> {
            id: object::new(ctx),
            sender: tx_context::sender(ctx),
            recipient,
            balance: coin::into_balance(coin),
            released: false,
        };
        transfer::share_object(escrow);
    }

    /// Release funds to recipient
    public fun release<T>(
        escrow: &mut Escrow<T>,
        ctx: &mut TxContext
    ) {
        assert!(tx_context::sender(ctx) == escrow.sender, ENotSender);
        assert!(!escrow.released, EAlreadyReleased);
        
        escrow.released = true;
    }

    /// Claim released funds
    public fun claim<T>(
        escrow: &mut Escrow<T>,
        ctx: &mut TxContext
    ): Coin<T> {
        assert!(tx_context::sender(ctx) == escrow.recipient, ENotRecipient);
        assert!(escrow.released, ENotReleased);
        
        let amount = balance::value(&escrow.balance);
        let withdrawn = balance::split(&mut escrow.balance, amount);
        coin::from_balance(withdrawn, ctx)
    }

    /// Refund to sender (if not released)
    public fun refund<T>(
        escrow: &mut Escrow<T>,
        ctx: &mut TxContext
    ): Coin<T> {
        assert!(tx_context::sender(ctx) == escrow.sender, ENotSender);
        assert!(!escrow.released, EAlreadyReleased);
        
        let amount = balance::value(&escrow.balance);
        let withdrawn = balance::split(&mut escrow.balance, amount);
        coin::from_balance(withdrawn, ctx)
    }

    /// Get escrow details
    public fun get_sender<T>(escrow: &Escrow<T>): address {
        escrow.sender
    }

    public fun get_recipient<T>(escrow: &Escrow<T>): address {
        escrow.recipient
    }

    public fun get_balance<T>(escrow: &Escrow<T>): u64 {
        balance::value(&escrow.balance)
    }

    public fun is_released<T>(escrow: &Escrow<T>): bool {
        escrow.released
    }
}
