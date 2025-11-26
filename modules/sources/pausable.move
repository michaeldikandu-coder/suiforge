module suiforge::pausable {
    use sui::tx_context::TxContext;

    /// Pausable state for emergency stops
    public struct Pausable has store, copy, drop {
        paused: bool,
    }

    /// Error codes
    const EPaused: u64 = 1;
    const ENotPaused: u64 = 2;

    /// Create a new Pausable instance (unpaused by default)
    public fun new(): Pausable {
        Pausable { paused: false }
    }

    /// Pause the contract
    public fun pause(p: &mut Pausable) {
        p.paused = true;
    }

    /// Unpause the contract
    public fun unpause(p: &mut Pausable) {
        p.paused = false;
    }

    /// Check if paused
    public fun is_paused(p: &Pausable): bool {
        p.paused
    }

    /// Require that contract is not paused
    public fun require_not_paused(p: &Pausable) {
        assert!(!p.paused, EPaused);
    }

    /// Require that contract is paused
    public fun require_paused(p: &Pausable) {
        assert!(p.paused, ENotPaused);
    }
}
