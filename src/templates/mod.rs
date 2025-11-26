pub mod basic;
pub mod nft;
pub mod token;

use crate::error::{Result, SuiForgeError};

pub enum Template {
    Basic,
    Nft,
    Token,
    Marketplace,
    Defi,
    Game,
}

impl Template {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(Template::Basic),
            "nft" => Ok(Template::Nft),
            "token" => Ok(Template::Token),
            "marketplace" => Ok(Template::Marketplace),
            "defi" => Ok(Template::Defi),
            "game" => Ok(Template::Game),
            _ => Err(SuiForgeError::InvalidTemplate(s.to_string())),
        }
    }

    pub fn get_move_toml(&self, project_name: &str) -> String {
        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024.beta"

[dependencies]
Sui = {{ git = "https://github.com/MystenLabs/sui.git", subdir = "crates/sui-framework/packages/sui-framework", rev = "framework/mainnet" }}

[addresses]
{} = "0x0"
"#,
            project_name, project_name
        )
    }

    pub fn get_main_move(&self, project_name: &str) -> String {
        match self {
            Template::Basic => basic::get_template(project_name),
            Template::Nft => nft::get_template(project_name),
            Template::Token => token::get_template(project_name),
            Template::Marketplace => self.get_marketplace_template(project_name),
            Template::Defi => self.get_defi_template(project_name),
            Template::Game => self.get_game_template(project_name),
        }
    }

    fn get_marketplace_template(&self, project_name: &str) -> String {
        format!(
            r#"module {}::marketplace {{
    use sui::object::{{Self, UID}};
    use sui::transfer;
    use sui::tx_context::{{Self, TxContext}};
    use sui::coin::{{Self, Coin}};
    use sui::sui::SUI;
    use sui::table::{{Self, Table}};

    // Marketplace for trading NFTs
    public struct Marketplace has key {{
        id: UID,
        listings: Table<address, Listing>,
    }}

    public struct Listing has store {{
        price: u64,
        seller: address,
    }}

    public fun create(ctx: &mut TxContext) {{
        let marketplace = Marketplace {{
            id: object::new(ctx),
            listings: table::new(ctx),
        }};
        transfer::share_object(marketplace);
    }}
}}
"#,
            project_name
        )
    }

    fn get_defi_template(&self, project_name: &str) -> String {
        format!(
            r#"module {}::vault {{
    use sui::object::{{Self, UID}};
    use sui::transfer;
    use sui::tx_context::{{Self, TxContext}};
    use sui::coin::{{Self, Coin}};
    use sui::balance::{{Self, Balance}};

    // Simple vault for depositing and withdrawing tokens
    public struct Vault<phantom T> has key {{
        id: UID,
        balance: Balance<T>,
    }}

    public fun create<T>(ctx: &mut TxContext) {{
        let vault = Vault<T> {{
            id: object::new(ctx),
            balance: balance::zero(),
        }};
        transfer::share_object(vault);
    }}

    public fun deposit<T>(vault: &mut Vault<T>, coin: Coin<T>) {{
        let balance = coin::into_balance(coin);
        balance::join(&mut vault.balance, balance);
    }}
}}
"#,
            project_name
        )
    }

    fn get_game_template(&self, project_name: &str) -> String {
        format!(
            r#"module {}::game {{
    use sui::object::{{Self, UID}};
    use sui::transfer;
    use sui::tx_context::{{Self, TxContext}};

    // Simple on-chain game character
    public struct Character has key, store {{
        id: UID,
        name: vector<u8>,
        level: u64,
        experience: u64,
    }}

    public fun create_character(name: vector<u8>, ctx: &mut TxContext): Character {{
        Character {{
            id: object::new(ctx),
            name,
            level: 1,
            experience: 0,
        }}
    }}

    public fun gain_experience(character: &mut Character, amount: u64) {{
        character.experience = character.experience + amount;
        if (character.experience >= 100) {{
            character.level = character.level + 1;
            character.experience = 0;
        }}
    }}
}}
"#,
            project_name
        )
    }
}
