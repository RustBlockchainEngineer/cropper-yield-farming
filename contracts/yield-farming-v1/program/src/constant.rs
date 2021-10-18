/// constants declaration file

/// mode of mainnet-beta or devnet, in case of mainnet-beta - const DEVNET_MODE:bool = false;
const DEVNET_MODE:bool = true;

/// Farm additaional fee
/// To create new farm without CRP token pairing, the creator must pay this additional farm fee as stable coin (USDC)
/// If the creator doesn't pay farm fee, displays "Not Allowed" instead of "Stake" button
/// So creator and farmers can't stake/unstake/harvest

pub const VERSION:u8 = 1;
pub const PREFIX:&str = "cropperfarm";

/// harvest fee. 0.1%
// pub const HARVEST_FEE_NUMERATOR:u64 = 1;
// pub const HARVEST_FEE_DENOMINATOR:u64 = 1000;
// pub const FARM_FEE:u64 = 5000;

/// initial super owner of this program. this owner can change program state
pub const INITIAL_SUPER_OWNER:&str = if DEVNET_MODE {"4GJ3z4skEHJADz3MVeNYBg4YV8H27rBQey2YYdiPC8PA"} else {"DyDdJM9KVsvosfXbcHDp4pRpmbMHkRq3pcarBykPy4ir"};

/// Fee owner wallet address
/// This includes harvest fee
/// So this wallet address should have all token accounts of registered token-list
// pub const FEE_OWNER:&str = if DEVNET_MODE {"BRmxAJ3ThceU2SXt6weyXarRNvAwZUtKuKbzSRneRxJn"} else {"4GJ3z4skEHJADz3MVeNYBg4YV8H27rBQey2YYdiPC8PA"};
// pub const AMM_PROGRAM_ID:&str = if DEVNET_MODE {"7ZZJNL4xD8db6yrT46SeMFZXcVr9MLepGpEtnKW2k6sW"} else {"7ZZJNL4xD8db6yrT46SeMFZXcVr9MLepGpEtnKW2k6sW"};

/// This is allowed wallet address to create specified farms by site owner
/// Specified farms are SOL-USDC, SOL-USDT, ETH-USDC, ETH-USDT, CRP-USDC, CRP-USDT, CRP-SOL, CRP-ETH
// pub const ALLOWED_CREATOR:&str = if DEVNET_MODE {"4GJ3z4skEHJADz3MVeNYBg4YV8H27rBQey2YYdiPC8PA"} else {"BRmxAJ3ThceU2SXt6weyXarRNvAwZUtKuKbzSRneRxJn"};

pub const TOKEN_PROGRAM_ID:&str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const RENT_SYSVAR_ID:&str = "SysvarRent111111111111111111111111111111111";
pub const CLOCK_SYSVAR_ID:&str = "SysvarC1ock11111111111111111111111111111111";
pub const SYSTEM_PROGRAM_ID:&str = "11111111111111111111111111111111";

/// Token mint addresses for specified farms above
pub const CRP_MINT_ADDRESS:&str = if DEVNET_MODE {"GGaUYeET8HXK34H2D1ieh4YYQPhkWcfWBZ4rdp6iCZtG"} else {"DubwWZNWiNGMMeeQHPnMATNj77YZPZSAz2WVR5WjLJqz"};
pub const USDC_MINT_ADDRESS:&str = if DEVNET_MODE {"6MBRfPbzejwVpADXq3LCotZetje3N16m5Yn7LCs2ffU4"} else {"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"};
pub const USDT_MINT_ADDRESS:&str = if DEVNET_MODE {"6La9ryWrDPByZViuQCizmo6aW98cK8DSL7angqmTFf9i"} else {"Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"};
pub const SOL_MINT_ADDRESS:&str = if DEVNET_MODE {"11111111111111111111111111111111"} else {"11111111111111111111111111111111"};
pub const ETH_MINT_ADDRESS:&str = if DEVNET_MODE {"2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk"} else {"2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk"};

/// reward multipler constant
pub const REWARD_MULTIPLER:u64 = 1000000000;
