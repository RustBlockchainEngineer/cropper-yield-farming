pub const GLOBAL_STATE_TAG:&[u8] = b"golbal-state-seed";
pub const FARM_TAG:&[u8] = b"farm-seed";
pub const USER_INFO_TAG:&[u8] = b"user-info-seed";
pub const FARM_POOL_LP_TAG:&[u8] = b"farm-pool-lp-seed";
pub const FARM_POOL_REWARD_TAG:&[u8] = b"farm-pool-reward-seed";
pub const DUAL_POOL_REWARD_TAG:&[u8] = b"dual-pool-reward-seed";
pub const DUAL_TAG:&[u8] = b"farm-dual";

const DEVNET_MODE:bool = {
    #[cfg(feature = "devnet")]
    {
        true
    }
    #[cfg(not(feature = "devnet"))]
    {
        false
    }
};

pub const VERSION:u8 = 3;

/// initial super owner of this program. this owner can change program state
pub const INITIAL_SUPER_OWNER:&str = if DEVNET_MODE {"61ZuXNtDC8LRV9xREgJv4rhQgU4woRN6BUWCCufppi8V"} else {"AwtDEd9GThBNWNahvLZUok1BiRULNQ86VruXkYAckCtV"};

pub const TOKEN_PROGRAM_ID:&str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const RENT_SYSVAR_ID:&str = "SysvarRent111111111111111111111111111111111";
pub const CLOCK_SYSVAR_ID:&str = "SysvarC1ock11111111111111111111111111111111";
pub const SYSTEM_PROGRAM_ID:&str = "11111111111111111111111111111111";

/// Token mint addresses for specified farms above
pub const CRP_MINT_ADDRESS:&str = if DEVNET_MODE {"GGaUYeET8HXK34H2D1ieh4YYQPhkWcfWBZ4rdp6iCZtG"} else {"DubwWZNWiNGMMeeQHPnMATNj77YZPZSAz2WVR5WjLJqz"};
pub const USDC_MINT_ADDRESS:&str = if DEVNET_MODE {"6MBRfPbzejwVpADXq3LCotZetje3N16m5Yn7LCs2ffU4"} else {"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"};
pub const USDT_MINT_ADDRESS:&str = if DEVNET_MODE {"6La9ryWrDPByZViuQCizmo6aW98cK8DSL7angqmTFf9i"} else {"Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"};
pub const SOL_MINT_ADDRESS:&str = if DEVNET_MODE {"So11111111111111111111111111111111111111112"} else {"So11111111111111111111111111111111111111112"};
pub const ETH_MINT_ADDRESS:&str = if DEVNET_MODE {"2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk"} else {"2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk"};

/// reward multipler constant
pub const REWARD_MULTIPLER:u64 = 1000000000;