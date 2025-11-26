module suiforge::access_control {
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    use sui::table::{Self, Table};
    use std::string::String;

    /// Role-based access control
    public struct AccessControl has store {
        roles: Table<address, vector<String>>,
        admin: address,
    }

    /// Error codes
    const ENotAuthorized: u64 = 1;
    const ENotAdmin: u64 = 2;

    /// Create a new AccessControl instance
    public fun new(ctx: &mut TxContext): AccessControl {
        AccessControl {
            roles: table::new(ctx),
            admin: tx_context::sender(ctx),
        }
    }

    /// Grant a role to an address
    public fun grant_role(
        ac: &mut AccessControl,
        account: address,
        role: String,
        ctx: &TxContext
    ) {
        assert!(tx_context::sender(ctx) == ac.admin, ENotAdmin);
        
        if (!table::contains(&ac.roles, account)) {
            table::add(&mut ac.roles, account, vector::empty());
        };
        
        let roles = table::borrow_mut(&mut ac.roles, account);
        vector::push_back(roles, role);
    }

    /// Revoke a role from an address
    public fun revoke_role(
        ac: &mut AccessControl,
        account: address,
        role: String,
        ctx: &TxContext
    ) {
        assert!(tx_context::sender(ctx) == ac.admin, ENotAdmin);
        
        if (table::contains(&ac.roles, account)) {
            let roles = table::borrow_mut(&mut ac.roles, account);
            let (found, index) = vector::index_of(roles, &role);
            if (found) {
                vector::remove(roles, index);
            };
        };
    }

    /// Check if an address has a specific role
    public fun has_role(ac: &AccessControl, account: address, role: &String): bool {
        if (!table::contains(&ac.roles, account)) {
            return false
        };
        
        let roles = table::borrow(&ac.roles, account);
        vector::contains(roles, role)
    }

    /// Require that sender has a specific role
    public fun require_role(ac: &AccessControl, role: &String, ctx: &TxContext) {
        assert!(has_role(ac, tx_context::sender(ctx), role), ENotAuthorized);
    }

    /// Check if address is admin
    public fun is_admin(ac: &AccessControl, account: address): bool {
        ac.admin == account
    }

    /// Transfer admin role
    public fun transfer_admin(ac: &mut AccessControl, new_admin: address, ctx: &TxContext) {
        assert!(tx_context::sender(ctx) == ac.admin, ENotAdmin);
        ac.admin = new_admin;
    }
}
