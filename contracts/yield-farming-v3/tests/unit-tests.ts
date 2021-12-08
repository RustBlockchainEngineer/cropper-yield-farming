import * as anchor from '@project-serum/anchor';
import assert from 'assert';
import {
  AMM_ID,
  AMM_PID,
  B2B_MINT_ADDRESS,
  B2B_USER_ADDRESS,
  CLOCK_SYSVAR_ID,
  CRP_B2B_LP_MINT,
  CRP_B2B_LP_USER,
  CRP_MINT_ADDRESS,
  CRP_USER_ADDRESS,
  DUAL_POOL_REWARD_TAG,
  FARM_POOL_LP_TAG,
  FARM_POOL_REWARD_TAG,
  FARM_TAG,
  FEE_OWNER,
  GLOBAL_STATE_TAG,
  HARVEST_FEE_DENOMINATOR,
  HARVEST_FEE_NUMERATOR,
  program,
  RENT_SYSVAR_ID,
  setupAll,
  SYSTEM_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  USER_INFO_TAG,
  wallet,
} from "./setup";

let globalStateKey = null, globalStateKeyNonce = 0;
let farmKey = null, farmKeyNonce = 0;

const startTime = Date.now() / 1000;
const endTime = startTime + 600;
const dualStartTime = startTime + 150;
const dualEndTime = dualStartTime + 300;
const newEndTime = startTime + 1000;
const newDualEndTime = dualStartTime + 400;
const dualRewardAmount = new anchor.BN(100 * 1000000000) ;
const dualRemoveRewardAmount = new anchor.BN(30 * 1000000000) ;
const depositAmount = new anchor.BN(10 * 100000000);
const withdrawAmount = new anchor.BN(10 * 100000000);
const singleRewardAmount = new anchor.BN(100 * 1000000) ;

describe("00. Setup for unit tests", () => {
  it("setup", async () => {
    await setupAll();
    [globalStateKey, globalStateKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(GLOBAL_STATE_TAG)], program.programId);
  });
});

describe("01. Manage global state", () => {
  it("Create global state", async () => {
    const tx = await program.rpc.createGlobalState(
      globalStateKeyNonce,
      HARVEST_FEE_NUMERATOR,
      HARVEST_FEE_DENOMINATOR,
      {
        accounts: {
          superOwner: wallet.publicKey,
          globalState: globalStateKey,
          newSuperOwner: wallet.publicKey,
          feeOwner: FEE_OWNER,
          allowedCreator: wallet.publicKey,
          ammProgramId: AMM_PID,
          systemProgram: SYSTEM_PROGRAM_ID,
          rent: RENT_SYSVAR_ID,
        },
      }
    );
    await program.account.farmProgram.fetch(globalStateKey);
  });
  it("Update global state", async () => {
    const tx = await program.rpc.createGlobalState(
      globalStateKeyNonce,
      HARVEST_FEE_NUMERATOR.add(new anchor.BN(1)),
      HARVEST_FEE_DENOMINATOR.add(new anchor.BN(1000)),
      {
        accounts: {
          superOwner: wallet.publicKey,
          globalState: globalStateKey,
          newSuperOwner: wallet.publicKey,
          feeOwner: wallet.publicKey,
          allowedCreator: wallet.publicKey,
          ammProgramId: AMM_PID,
          systemProgram: SYSTEM_PROGRAM_ID,
          rent: RENT_SYSVAR_ID,
        },
      }
    );
    const globalState = await program.account.farmProgram.fetch(globalStateKey);
    assert(globalState.harvestFeeDenominator.toNumber() === HARVEST_FEE_DENOMINATOR.toNumber() + 1000);
  });
});


describe("02. Farm Management", () => {
  it("create new farm", async () => {
    const newFarmSeed = anchor.web3.Keypair.generate();
    [farmKey, farmKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_TAG), newFarmSeed.publicKey.toBuffer()], program.programId);
    const [farmPoolLpKey, farmPoolLpKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_POOL_LP_TAG), farmKey.toBuffer()], program.programId);
    const [farmPoolRewardKey, farmPoolRewardKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_POOL_REWARD_TAG), farmKey.toBuffer()], program.programId);

    const tx = await program.rpc.createFarm(
      globalStateKeyNonce,
      farmKeyNonce,
      farmPoolLpKeyNonce,
      farmPoolRewardKeyNonce,
      new anchor.BN(startTime),
      new anchor.BN(endTime),
      {
        accounts: {
          creator: wallet.publicKey,
          globalState: globalStateKey,
          newFarm: farmKey,
          farmSeed: newFarmSeed.publicKey,
          poolLpMint: CRP_B2B_LP_MINT,
          poolRewardMint: CRP_MINT_ADDRESS,
          poolLpToken: farmPoolLpKey,
          poolRewardToken: farmPoolRewardKey,
          ammSwap: AMM_ID,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: RENT_SYSVAR_ID
        }
      }
    );

    await program.account.farmPool.fetch(farmKey);
  });
  it("extend old farm", async () => {
    const oldFarm = await program.account.farmPool.fetch(farmKey);
    const tx = await program.rpc.extendFarm(
      farmKeyNonce,
      new anchor.BN(newEndTime),
      {
        accounts: {
          creator: wallet.publicKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey
        }
      }
    );
    const newFarm = await program.account.farmPool.fetch(farmKey);
    assert(newFarm.endTimestamp.toNumber() - newFarm.startTimestamp.toNumber() === newEndTime - startTime);
  });
  it("create dual", async () => {
    const oldFarm = await program.account.farmPool.fetch(farmKey);
    [farmKey, farmKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_TAG), oldFarm.seedKey.toBuffer()], program.programId);
    const [farmPoolRewardDualKey, farmPoolRewardDualKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(DUAL_POOL_REWARD_TAG), farmKey.toBuffer()], program.programId);
    const tx = await program.rpc.createDual(
      globalStateKeyNonce,
      farmKeyNonce,
      farmPoolRewardDualKeyNonce,
      new anchor.BN(dualStartTime),
      new anchor.BN(dualEndTime),
      {
        accounts: {
          creator: wallet.publicKey,
          globalState: globalStateKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey,
          poolRewardMintDual: B2B_MINT_ADDRESS,
          poolRewardTokenDual: farmPoolRewardDualKey,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: RENT_SYSVAR_ID
        }
      }
    );

    const dualFarm = await program.account.farmPool.fetch(farmKey);
    assert(dualFarm.poolRewardTokenAccountDual.toBase58() === farmPoolRewardDualKey.toBase58());
  });
  it("extend dual farm", async () => {
    const oldFarm = await program.account.farmPool.fetch(farmKey);
    const tx = await program.rpc.extendDual(
      farmKeyNonce,
      new anchor.BN(newDualEndTime),
      {
        accounts: {
          creator: wallet.publicKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey
        }
      }
    );
    const newFarm = await program.account.farmPool.fetch(farmKey);
    assert(newFarm.endTimestampDual.toNumber() - newFarm.startTimestampDual.toNumber() === newDualEndTime - dualStartTime);
  });
});

describe("03. reward management", () => {
  it("add single reward", async () => {
    const [farmPoolLpKey, farmPoolLpKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_POOL_LP_TAG), farmKey.toBuffer()], program.programId);
    const [farmPoolRewardKey, farmPoolRewardKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_POOL_REWARD_TAG), farmKey.toBuffer()], program.programId);
    const oldFarm = await program.account.farmPool.fetch(farmKey);
    const tx = await program.rpc.addRewardSingle(
      globalStateKeyNonce,
      farmKeyNonce,
      farmPoolLpKeyNonce,
      farmPoolRewardKeyNonce,
      new anchor.BN(singleRewardAmount),
      {
        accounts: {
          depositor: wallet.publicKey,
          globalState: globalStateKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey,
          poolLpToken: farmPoolLpKey,
          poolRewardToken: farmPoolRewardKey,
          userRewardToken: CRP_USER_ADDRESS,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: RENT_SYSVAR_ID,
          clock: CLOCK_SYSVAR_ID
        }
      }
    );
    const newFarm = await program.account.farmPool.fetch(farmKey);
    assert(newFarm.currentRewards.toNumber() === singleRewardAmount.toNumber());
  });
  
  it("add dual reward", async () => {
    const [farmPoolLpKey, farmPoolLpKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_POOL_LP_TAG), farmKey.toBuffer()], program.programId);
    const [farmPoolRewardKey, farmPoolRewardKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(DUAL_POOL_REWARD_TAG), farmKey.toBuffer()], program.programId);
    const oldFarm = await program.account.farmPool.fetch(farmKey);
    const tx = await program.rpc.addRewardDual(
      globalStateKeyNonce,
      farmKeyNonce,
      farmPoolLpKeyNonce,
      farmPoolRewardKeyNonce,
      new anchor.BN(dualRewardAmount),
      {
        accounts: {
          depositor: wallet.publicKey,
          globalState: globalStateKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey,
          poolLpToken: farmPoolLpKey,
          poolRewardTokenDual: farmPoolRewardKey,
          userRewardTokenDual: B2B_USER_ADDRESS,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: RENT_SYSVAR_ID,
          clock: CLOCK_SYSVAR_ID
        }
      }
    );
    const newFarm = await program.account.farmPool.fetch(farmKey);
    assert(newFarm.currentRewardsDual.toNumber() === dualRewardAmount.toNumber());
  });
  it("remove dual reward", async () => {
    const [farmPoolLpKey, farmPoolLpKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_POOL_LP_TAG), farmKey.toBuffer()], program.programId);
    const [farmPoolRewardKey, farmPoolRewardKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(DUAL_POOL_REWARD_TAG), farmKey.toBuffer()], program.programId);

    const oldFarm = await program.account.farmPool.fetch(farmKey);
    const tx = await program.rpc.removeRewardDual(
      globalStateKeyNonce,
      farmKeyNonce,
      farmPoolLpKeyNonce,
      farmPoolRewardKeyNonce,
      new anchor.BN(dualRemoveRewardAmount),
      {
        accounts: {
          depositor: wallet.publicKey,
          globalState: globalStateKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey,
          poolLpToken: farmPoolLpKey,
          poolRewardTokenDual: farmPoolRewardKey,
          userRewardTokenDual: B2B_USER_ADDRESS,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: RENT_SYSVAR_ID,
          clock: CLOCK_SYSVAR_ID
        }
      }
    );
    const newFarm = await program.account.farmPool.fetch(farmKey);
    assert(newFarm.currentRewardsDual.toNumber() === dualRewardAmount.toNumber() - dualRemoveRewardAmount.toNumber());
  });
});

describe("04. farming operations", () => {
  it("deposit lp", async () => {
    const [farmPoolLpKey, farmPoolLpKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_POOL_LP_TAG), farmKey.toBuffer()], program.programId);
    const [userInfoKey, userInfoKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(USER_INFO_TAG), farmKey.toBuffer(), wallet.publicKey.toBuffer()], program.programId);
    const withSwapAction = 0;
    const oldFarm = await program.account.farmPool.fetch(farmKey);
    const tx = await program.rpc.deposit(
      globalStateKeyNonce,
      farmKeyNonce,
      farmPoolLpKeyNonce,
      userInfoKeyNonce,
      withSwapAction,
      depositAmount,
      {
        accounts: {
          depositor: wallet.publicKey,
          globalState: globalStateKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey,
          userInfo: userInfoKey,
          poolLpToken: farmPoolLpKey,
          userLpToken: CRP_B2B_LP_USER,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: RENT_SYSVAR_ID,
          clock: CLOCK_SYSVAR_ID
        }
      }
    );
    const newFarm = await program.account.farmPool.fetch(farmKey);
    assert(newFarm.poolLpBalance.toNumber() === depositAmount.toNumber() + oldFarm.poolLpBalance.toNumber());
  });

  it("harvest single reward", async () => {
    const [farmPoolRewardKey] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_POOL_REWARD_TAG), farmKey.toBuffer()], program.programId);
    const [userInfoKey, userInfoKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(USER_INFO_TAG), farmKey.toBuffer(), wallet.publicKey.toBuffer()], program.programId);
    const rewardType = 0;
    const oldFarm = await program.account.farmPool.fetch(farmKey);
    const tx = await program.rpc.harvest(
      globalStateKeyNonce,
      farmKeyNonce,
      userInfoKeyNonce,
      rewardType,
      {
        accounts: {
          harvester: wallet.publicKey,
          globalState: globalStateKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey,
          userInfo: userInfoKey,
          poolRewardToken: farmPoolRewardKey,
          userRewardToken: CRP_USER_ADDRESS,
          feeRewardToken: CRP_USER_ADDRESS,
          tokenProgram: TOKEN_PROGRAM_ID,
          clock: CLOCK_SYSVAR_ID
        }
      }
    );
    const newFarm = await program.account.farmPool.fetch(farmKey);
    assert(newFarm.harvestedRewards.toNumber() > oldFarm.harvestedRewards.toNumber());
  });
  it("harvest dual reward", async () => {
    const [farmPoolRewardKey] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(DUAL_POOL_REWARD_TAG), farmKey.toBuffer()], program.programId);
    const [userInfoKey, userInfoKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(USER_INFO_TAG), farmKey.toBuffer(), wallet.publicKey.toBuffer()], program.programId);
    const rewardType = 1;
    const oldFarm = await program.account.farmPool.fetch(farmKey);
    const tx = await program.rpc.harvest(
      globalStateKeyNonce,
      farmKeyNonce,
      userInfoKeyNonce,
      rewardType,
      {
        accounts: {
          harvester: wallet.publicKey,
          globalState: globalStateKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey,
          userInfo: userInfoKey,
          poolRewardToken: farmPoolRewardKey,
          userRewardToken: B2B_USER_ADDRESS,
          feeRewardToken: B2B_USER_ADDRESS,
          tokenProgram: TOKEN_PROGRAM_ID,
          clock: CLOCK_SYSVAR_ID
        }
      }
    );
    const newFarm = await program.account.farmPool.fetch(farmKey);
    assert(newFarm.harvestedRewardsDual.toNumber() > oldFarm.harvestedRewardsDual.toNumber());
  });
  it("withdraw lp", async () => {
    const [farmPoolLpKey, farmPoolLpKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(FARM_POOL_LP_TAG), farmKey.toBuffer()], program.programId);
    const [userInfoKey, userInfoKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(USER_INFO_TAG), farmKey.toBuffer(), wallet.publicKey.toBuffer()], program.programId);
    const withSwapAction = 0;
    const oldFarm = await program.account.farmPool.fetch(farmKey);
    const tx = await program.rpc.withdraw(
      globalStateKeyNonce,
      farmKeyNonce,
      farmPoolLpKeyNonce,
      userInfoKeyNonce,
      withSwapAction,
      withdrawAmount,
      {
        accounts: {
          withdrawer: wallet.publicKey,
          globalState: globalStateKey,
          farm: farmKey,
          farmSeed: oldFarm.seedKey,
          userInfo: userInfoKey,
          poolLpToken: farmPoolLpKey,
          userLpToken: CRP_B2B_LP_USER,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: RENT_SYSVAR_ID,
          clock: CLOCK_SYSVAR_ID
        }
      }
    );
    const newFarm = await program.account.farmPool.fetch(farmKey);
    assert(newFarm.poolLpBalance.toNumber() === oldFarm.poolLpBalance.toNumber() - withdrawAmount.toNumber());
  });
});
