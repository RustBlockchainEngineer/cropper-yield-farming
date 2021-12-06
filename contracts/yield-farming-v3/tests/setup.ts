import * as anchor from '@project-serum/anchor';
import { AnchorFarm } from '../target/types/anchor_farm';

export const GLOBAL_STATE_TAG = 'golbal-state-seed';
export const FARM_TAG = 'farm-seed';
export const USER_INFO_TAG = 'user-info-seed';
export const FARM_POOL_LP_TAG = 'farm-pool-lp-seed';
export const FARM_POOL_REWARD_TAG = 'farm-pool-reward-seed';
export const DUAL_POOL_REWARD_TAG = 'dual-pool-reward-seed';
export const DUAL_TAG = 'farm-dual';

export const DEVNET_MODE = true;
export const VERSION = 3;

export const TOKEN_PROGRAM_ID = new anchor.web3.PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
export const RENT_SYSVAR_ID = new anchor.web3.PublicKey('SysvarRent111111111111111111111111111111111');
export const CLOCK_SYSVAR_ID = new anchor.web3.PublicKey('SysvarC1ock11111111111111111111111111111111');
export const SYSTEM_PROGRAM_ID = new anchor.web3.PublicKey('11111111111111111111111111111111');

export const CRP_MINT_ADDRESS = new anchor.web3.PublicKey(DEVNET_MODE ? 'GGaUYeET8HXK34H2D1ieh4YYQPhkWcfWBZ4rdp6iCZtG' : 'DubwWZNWiNGMMeeQHPnMATNj77YZPZSAz2WVR5WjLJqz');
export const B2B_MINT_ADDRESS = new anchor.web3.PublicKey('ECe1Hak68wLS44NEwBVNtZDMxap1bX3jPCoAnDLFWDHz');
export const USDC_MINT_ADDRESS = new anchor.web3.PublicKey(DEVNET_MODE ? '6MBRfPbzejwVpADXq3LCotZetje3N16m5Yn7LCs2ffU4' : 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
export const USDT_MINT_ADDRESS = new anchor.web3.PublicKey(DEVNET_MODE ? '6La9ryWrDPByZViuQCizmo6aW98cK8DSL7angqmTFf9i' : 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB');
export const SOL_MINT_ADDRESS = new anchor.web3.PublicKey(DEVNET_MODE ? 'So11111111111111111111111111111111111111112' : 'So11111111111111111111111111111111111111112');
export const ETH_MINT_ADDRESS = new anchor.web3.PublicKey(DEVNET_MODE ? '2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk' : '2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk');

export const REWARD_MULTIPLER = 1000000000;

export const HARVEST_FEE_NUMERATOR = new anchor.BN(1);
export const HARVEST_FEE_DENOMINATOR = new anchor.BN(1000);
export const FEE_OWNER = new anchor.web3.PublicKey('7mGv8ysw45zicLWeYvt3fPRdWSxk2T7jGrXQPisn1F8v');
export const AMM_PID = new anchor.web3.PublicKey('7ZZJNL4xD8db6yrT46SeMFZXcVr9MLepGpEtnKW2k6sW');
export const CRP_B2B_LP_MINT = new anchor.web3.PublicKey('GD2BRKZRFoJpue6WJX7Y4oAokX6w95mQ85v9nn6gQkdt');
export const MINIMUM_SOL_AMOUNT = 10;

anchor.setProvider(anchor.Provider.env());
export const program = anchor.workspace.AnchorFarm as anchor.Program<AnchorFarm>;
export const wallet = program.provider.wallet;

export async function setupAll(){
    
    // airdrop
    while(await program.provider.connection.getBalance(program.provider.wallet.publicKey) < MINIMUM_SOL_AMOUNT){
        await program.provider.connection.requestAirdrop(program.provider.wallet.publicKey, 5 * 1000000000);
        await program.provider.connection.requestAirdrop(program.provider.wallet.publicKey, 5 * 1000000000);
    };
}
