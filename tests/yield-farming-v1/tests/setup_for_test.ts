
import { FARM_PROGRAM_ID } from "./utils/ids";
import { AccountLayout, ASSOCIATED_TOKEN_PROGRAM_ID, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { 
  Account, 
  AccountInfo, 
  Commitment, 
  Connection, 
  ParsedAccountData, 
  PublicKey, 
  SystemProgram, 
  Transaction, 
  TransactionInstruction, 
  TransactionSignature } from "@solana/web3.js";
import BigNumber from "bignumber.js";
import { readFileSync } from "fs";

const CREATOR_WALLET = "./tests/wallets/creator-wallet.json"
const FARMER_WALLET = "./tests/wallets/farmer-wallet.json"
const FEE_OWNER_WALLET = "./tests/wallets/fee-owner-wallet.json"
const SOURCE_WALLET = "./tests/wallets/source-wallet.json"

export const B2BCRPAmmPubkey = new PublicKey("Hafix8Ge6aeiHVD1qgTw1AKcTXb7xkZdYymd279UNjG2");
export const B2BUSDCAmmPubkey = new PublicKey("BKVnxX1JrTmD6ffXYHHMSsxndEPd9QRrEP75vDK72ocQ");

export const USDCMINTPubkey = new PublicKey("6MBRfPbzejwVpADXq3LCotZetje3N16m5Yn7LCs2ffU4");
export const B2BMINTPubkey = new PublicKey("ECe1Hak68wLS44NEwBVNtZDMxap1bX3jPCoAnDLFWDHz");
export const CRPMINTPubkey = new PublicKey("GGaUYeET8HXK34H2D1ieh4YYQPhkWcfWBZ4rdp6iCZtG");

export const B2BCRPLPPubkey = new PublicKey("CWKdKPGN8JkqQv4eV9xPSm3E99RKumf2vF4nUVwX1b9h");
export const B2BUSDCLPPubkey = new PublicKey("CT9uJhKr9ezxmMsjofeTkdFgmYKuGaqQrAvfJRsZZgbQ");

export const FarmProgramPubkey = new PublicKey(FARM_PROGRAM_ID);

export const connection = new Connection('https://api.devnet.solana.com');

export interface TokenInfo {
    symbol: string
    name: string
  
    mintAddress: string
    decimals: number
    totalSupply?: TokenAmount
  
    referrer?: string
  
    details?: string
    docs?: object
    socials?: object
  
    tokenAccountAddress?: string
    balance?: TokenAmount
    tags: string[]
  }
export const NATIVE_SOL: TokenInfo = {
    symbol: 'SOL',
    name: 'Native Solana',
    mintAddress: '11111111111111111111111111111111',
    decimals: 9,
    tags: ['cropper']
  }

  export class TokenAmount {
    public wei: BigNumber
  
    public decimals: number
    public _decimals: BigNumber
  
    constructor(wei: number | string | BigNumber, decimals: number = 0, isWei = true) {
      this.decimals = decimals
      this._decimals = new BigNumber(10).exponentiatedBy(decimals)
  
      if (isWei) {
        this.wei = new BigNumber(wei)
      } else {
        this.wei = new BigNumber(wei).multipliedBy(this._decimals)
      }
    }
  
    toEther() {
      return this.wei.dividedBy(this._decimals)
    }
  
    toWei() {
      return this.wei
    }
  
    format() {
      const vaule = this.wei.dividedBy(this._decimals)
      return vaule.toFormat(vaule.isInteger() ? 0 : this.decimals)
    }
  
    fixed() {
      return this.wei.dividedBy(this._decimals).toFixed(this.decimals)
    }
  
    isNullOrZero() {
      return this.wei.isNaN() || this.wei.isZero()
    }
    // + plus
    // - minus
    // × multipliedBy
    // ÷ dividedBy
  }

  export function createSplAccount(
    instructions: TransactionInstruction[],
    payer: PublicKey,
    accountRentExempt: number,
    mint: PublicKey,
    owner: PublicKey,
    space: number
  ) {
    const account = new Account();
    instructions.push(
      SystemProgram.createAccount({
        fromPubkey: payer,
        newAccountPubkey: account.publicKey,
        lamports: accountRentExempt,
        space,
        programId: TOKEN_PROGRAM_ID,
      })
    );
  
    instructions.push(
      Token.createInitAccountInstruction(
        TOKEN_PROGRAM_ID,
        mint,
        account.publicKey,
        owner
      )
    );
  
    return account;
  }
export async function findProgramAddress(seeds: Array<Buffer | Uint8Array>, programId: PublicKey) {
    const [publicKey, nonce] = await PublicKey.findProgramAddress(seeds, programId)
    return { publicKey, nonce }
  }
export async function findAssociatedTokenAddress(walletAddress: PublicKey, tokenMintAddress: PublicKey) {
    const { publicKey } = await findProgramAddress(
      [walletAddress.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), tokenMintAddress.toBuffer()],
      ASSOCIATED_TOKEN_PROGRAM_ID
    )
    return publicKey
  }
function getCreatorWalletAccount(){
    const creator = JSON.parse(readFileSync(CREATOR_WALLET, "utf-8"));
    return new Account(creator);
}
function getFarmerWalletAccount(){
    const creator = JSON.parse(readFileSync(FARMER_WALLET, "utf-8"));
    return new Account(creator);
}
function getFeeWalletAccount(){
    const creator = JSON.parse(readFileSync(FEE_OWNER_WALLET, "utf-8"));
    return new Account(creator);
}
function getSourceWalletAccount(){
    const creator = JSON.parse(readFileSync(SOURCE_WALLET, "utf-8"));
    return new Account(creator);
}

export const CREATOR_WALLET_ACCOUNT = getCreatorWalletAccount();
export const FARMER_WALLET_ACCOUNT = getFarmerWalletAccount();
export const FEE_OWNER_WALLET_ACCOUNT = getFeeWalletAccount();
export const SOURCE_WALLET_ACCOUNT = getSourceWalletAccount();

const SOL_DECIMAL_PRECISION = 1000000000;
const WALLET_SOL_AMOUNT_FOR_TEST = 10 * SOL_DECIMAL_PRECISION;

async function airdropSol(wallet:Account){
    let solAmount = await connection.getBalance(wallet.publicKey);
    console.log("sol amount = ",solAmount / SOL_DECIMAL_PRECISION)
    if(solAmount < WALLET_SOL_AMOUNT_FOR_TEST / 2){
      await connection.requestAirdrop(wallet.publicKey,WALLET_SOL_AMOUNT_FOR_TEST)
    }
}

async function getTokenAccounts(wallet:Account) {
    let parsedTokenAccounts = await connection.getParsedTokenAccountsByOwner(
        wallet.publicKey,
        {
        programId: TOKEN_PROGRAM_ID
        },
        'confirmed'
    );
    const tokenAccounts: any = {}
    parsedTokenAccounts.value.forEach((tokenAccountInfo) => {
        const tokenAccountPubkey = tokenAccountInfo.pubkey
        const tokenAccountAddress = tokenAccountPubkey.toBase58()
        const parsedInfo = tokenAccountInfo.account.data.parsed.info
        const mintAddress = parsedInfo.mint
        const balance = new TokenAmount(parsedInfo.tokenAmount.amount, parsedInfo.tokenAmount.decimals)

        tokenAccounts[mintAddress] = {
        tokenAccountAddress,
        balance
        }
    })

    const solBalance = await connection.getBalance(wallet.publicKey, 'confirmed')
    tokenAccounts[NATIVE_SOL.mintAddress] = {
        tokenAccountAddress: wallet.publicKey.toBase58(),
        balance: new TokenAmount(solBalance, NATIVE_SOL.decimals)
    }
    return tokenAccounts;
    
}


async function addToken(wallet:Account, mintPubkey:PublicKey){
    //check if wallet has this token
    let tokenAccounts = await getTokenAccounts(wallet);
    let findToken = tokenAccounts[mintPubkey.toBase58()] ? tokenAccounts[mintPubkey.toBase58()].tokenAccountAddress : undefined;
    if(findToken){
        console.log("finded given token");
        return;
    }

    let instructions: TransactionInstruction[] = [];
    const accountRentExempt = await connection.getMinimumBalanceForRentExemption(
      AccountLayout.span
    );
    let tokenAccount = await createSplAccount(
      instructions,
      wallet.publicKey,
      accountRentExempt,
      mintPubkey,
      wallet.publicKey,
      AccountLayout.span
    );
    let transaction = new Transaction()
    instructions.forEach((inst)=>{
      transaction.add(inst)
    });
    
    let result = await connection.sendTransaction(transaction,[wallet,tokenAccount]);
    
    //let tx = await sendTransaction(connection, wallet, transaction, [
    //  tokenAccount,
    //]);
}
async function provideSPLToken(wallet:Account, mintPubkey:PublicKey){
    let amountToGive = 1000;
    let dstTokenAccounts = await getTokenAccounts(wallet);
    let dstTokenAddress = dstTokenAccounts[mintPubkey.toBase58()] ? dstTokenAccounts[mintPubkey.toBase58()].tokenAccountAddress : undefined;
    let dstTokenAccountPubkey = new PublicKey(dstTokenAddress);
    let dstBalance = dstTokenAccounts[mintPubkey.toBase58()] ? dstTokenAccounts[mintPubkey.toBase58()].balance : undefined;
    dstBalance = Number.parseInt(dstBalance);
    if(dstTokenAddress && dstBalance < amountToGive){
        let srcTokenAccounts = await getTokenAccounts(wallet);
        let srcTokenAddress = srcTokenAccounts[mintPubkey.toBase58()] ? srcTokenAccounts[mintPubkey.toBase58()].tokenAccountAddress : undefined;
        let srcTokenAccountPubkey = new PublicKey(srcTokenAddress);
        let srcBalance = srcTokenAccounts[mintPubkey.toBase58()] ? srcTokenAccounts[mintPubkey.toBase58()].balance : undefined;
        srcBalance = Number.parseInt(srcBalance);
        if(srcBalance > amountToGive){
            let token = new Token(connection,mintPubkey,TOKEN_PROGRAM_ID,SOURCE_WALLET_ACCOUNT);
            await token.transfer(srcTokenAccountPubkey,dstTokenAccountPubkey,SOURCE_WALLET_ACCOUNT,[],amountToGive);
        }
    }
}
export async function airdropAndMakeTokens(){

  console.log("airdroping ...")
  await airdropSol(CREATOR_WALLET_ACCOUNT);
  await airdropSol(FARMER_WALLET_ACCOUNT);
  await airdropSol(FEE_OWNER_WALLET_ACCOUNT);
  await delay(5000);

  console.log("adding tokens ...")
  await addToken(CREATOR_WALLET_ACCOUNT,USDCMINTPubkey);
  await addToken(CREATOR_WALLET_ACCOUNT,CRPMINTPubkey);
  await addToken(CREATOR_WALLET_ACCOUNT,B2BMINTPubkey);

  await addToken(FARMER_WALLET_ACCOUNT,USDCMINTPubkey);
  await addToken(FARMER_WALLET_ACCOUNT,CRPMINTPubkey);
  await addToken(FARMER_WALLET_ACCOUNT,B2BMINTPubkey);

  console.log("providng token amount ...")
  await provideSPLToken(CREATOR_WALLET_ACCOUNT, USDCMINTPubkey);
  await provideSPLToken(CREATOR_WALLET_ACCOUNT, CRPMINTPubkey);
  await provideSPLToken(CREATOR_WALLET_ACCOUNT, B2BMINTPubkey);

  await provideSPLToken(FARMER_WALLET_ACCOUNT, USDCMINTPubkey);
  await provideSPLToken(FARMER_WALLET_ACCOUNT, CRPMINTPubkey);
  await provideSPLToken(FARMER_WALLET_ACCOUNT, B2BMINTPubkey);
}

export async function delay(ms: number) {
  console.log("delaying "+ms+" ms ...")
  return new Promise( resolve => setTimeout(resolve, ms) );
}