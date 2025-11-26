pub fn get_template(project_name: &str) -> String {
    format!(
        r#"module {}::nft {{
    use sui::object::{{Self, UID}};
    use sui::transfer;
    use sui::tx_context::{{Self, TxContext}};
    use std::string::{{Self, String}};
    use sui::url::{{Self, Url}};
    use sui::event;

    /// NFT Collection
    public struct Collection has key {{
        id: UID,
        name: String,
        total_supply: u64,
    }}

    /// Individual NFT
    public struct NFT has key, store {{
        id: UID,
        name: String,
        description: String,
        url: Url,
        collection_id: address,
    }}

    /// Event emitted when NFT is minted
    public struct MintEvent has copy, drop {{
        nft_id: address,
        minter: address,
        name: String,
    }}

    /// Create a new collection
    public fun create_collection(name: vector<u8>, ctx: &mut TxContext) {{
        let collection = Collection {{
            id: object::new(ctx),
            name: string::utf8(name),
            total_supply: 0,
        }};
        transfer::share_object(collection);
    }}

    /// Mint a new NFT
    public fun mint(
        collection: &mut Collection,
        name: vector<u8>,
        description: vector<u8>,
        url: vector<u8>,
        ctx: &mut TxContext
    ) {{
        let nft_id = object::new(ctx);
        let nft_address = object::uid_to_address(&nft_id);
        
        let nft = NFT {{
            id: nft_id,
            name: string::utf8(name),
            description: string::utf8(description),
            url: url::new_unsafe_from_bytes(url),
            collection_id: object::uid_to_address(&collection.id),
        }};

        collection.total_supply = collection.total_supply + 1;

        event::emit(MintEvent {{
            nft_id: nft_address,
            minter: tx_context::sender(ctx),
            name: string::utf8(name),
        }});

        transfer::public_transfer(nft, tx_context::sender(ctx));
    }}

    /// Transfer NFT to another address
    public fun transfer_nft(nft: NFT, recipient: address) {{
        transfer::public_transfer(nft, recipient);
    }}

    /// Get NFT name
    public fun get_name(nft: &NFT): String {{
        nft.name
    }}

    /// Get NFT description
    public fun get_description(nft: &NFT): String {{
        nft.description
    }}
}}
"#,
        project_name
    )
}
