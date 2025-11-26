pub fn get_template(project_name: &str) -> String {
    format!(
        r#"module {}::main {{
    use sui::object::{{Self, UID}};
    use sui::transfer;
    use sui::tx_context::{{Self, TxContext}};

    /// A simple object that can be created and transferred
    public struct Item has key, store {{
        id: UID,
        value: u64,
    }}

    /// Create a new Item
    public fun create(value: u64, ctx: &mut TxContext): Item {{
        Item {{
            id: object::new(ctx),
            value,
        }}
    }}

    /// Transfer an Item to a recipient
    public fun transfer_item(item: Item, recipient: address) {{
        transfer::public_transfer(item, recipient);
    }}

    /// Get the value of an Item
    public fun get_value(item: &Item): u64 {{
        item.value
    }}

    /// Update the value of an Item
    public fun update_value(item: &mut Item, new_value: u64) {{
        item.value = new_value;
    }}
}}
"#,
        project_name
    )
}
