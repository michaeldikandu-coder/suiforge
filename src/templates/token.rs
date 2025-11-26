pub fn get_template(project_name: &str) -> String {
    format!(
        r#"module {}::token {{
    use sui::coin::{{Self, Coin, TreasuryCap}};
    use sui::transfer;
    use sui::tx_context::{{Self, TxContext}};

    /// The token type
    public struct TOKEN has drop {{}}

    /// Initialize the token with a treasury cap
    fun init(witness: TOKEN, ctx: &mut TxContext) {{
        let (treasury, metadata) = coin::create_currency(
            witness,
            9, // decimals
            b"TKN", // symbol
            b"My Token", // name
            b"A custom token on Sui", // description
            option::none(), // icon url
            ctx
        );
        
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury, tx_context::sender(ctx));
    }}

    /// Mint new tokens
    public fun mint(
        treasury: &mut TreasuryCap<TOKEN>,
        amount: u64,
        recipient: address,
        ctx: &mut TxContext
    ) {{
        let coin = coin::mint(treasury, amount, ctx);
        transfer::public_transfer(coin, recipient);
    }}

    /// Burn tokens
    public fun burn(treasury: &mut TreasuryCap<TOKEN>, coin: Coin<TOKEN>) {{
        coin::burn(treasury, coin);
    }}
}}
"#,
        project_name
    )
}
