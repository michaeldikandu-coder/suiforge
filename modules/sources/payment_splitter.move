module suiforge::payment_splitter {
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::coin::{Self, Coin};
    use sui::balance::{Self, Balance};
    use sui::table::{Self, Table};

    /// Payment splitter for revenue distribution
    public struct PaymentSplitter<phantom T> has key {
        id: UID,
        payees: vector<address>,
        shares: Table<address, u64>,
        total_shares: u64,
        balance: Balance<T>,
    }

    /// Error codes
    const EInvalidShares: u64 = 1;
    const ENotPayee: u64 = 2;
    const EInsufficientBalance: u64 = 3;

    /// Create a new payment splitter
    public fun create<T>(
        payees: vector<address>,
        shares: vector<u64>,
        ctx: &mut TxContext
    ) {
        assert!(vector::length(&payees) == vector::length(&shares), EInvalidShares);
        assert!(vector::length(&payees) > 0, EInvalidShares);

        let shares_table = table::new<address, u64>(ctx);
        let mut total_shares = 0u64;
        let mut i = 0;

        while (i < vector::length(&payees)) {
            let payee = *vector::borrow(&payees, i);
            let share = *vector::borrow(&shares, i);
            assert!(share > 0, EInvalidShares);
            
            table::add(&mut shares_table, payee, share);
            total_shares = total_shares + share;
            i = i + 1;
        };

        let splitter = PaymentSplitter<T> {
            id: object::new(ctx),
            payees,
            shares: shares_table,
            total_shares,
            balance: balance::zero(),
        };
        transfer::share_object(splitter);
    }

    /// Deposit funds to be split
    public fun deposit<T>(splitter: &mut PaymentSplitter<T>, coin: Coin<T>) {
        let balance = coin::into_balance(coin);
        balance::join(&mut splitter.balance, balance);
    }

    /// Withdraw share for a payee
    public fun withdraw<T>(
        splitter: &mut PaymentSplitter<T>,
        ctx: &mut TxContext
    ): Coin<T> {
        let sender = tx_context::sender(ctx);
        assert!(table::contains(&splitter.shares, sender), ENotPayee);

        let total_balance = balance::value(&splitter.balance);
        let share = *table::borrow(&splitter.shares, sender);
        let amount = (total_balance * share) / splitter.total_shares;

        assert!(amount > 0, EInsufficientBalance);

        let withdrawn = balance::split(&mut splitter.balance, amount);
        coin::from_balance(withdrawn, ctx)
    }

    /// Get total balance
    public fun balance<T>(splitter: &PaymentSplitter<T>): u64 {
        balance::value(&splitter.balance)
    }

    /// Get payee share
    public fun get_share<T>(splitter: &PaymentSplitter<T>, payee: address): u64 {
        if (table::contains(&splitter.shares, payee)) {
            *table::borrow(&splitter.shares, payee)
        } else {
            0
        }
    }

    /// Get total shares
    public fun total_shares<T>(splitter: &PaymentSplitter<T>): u64 {
        splitter.total_shares
    }
}
