import * as anchor from '@project-serum/anchor';
import assert from 'assert';
import {
  AMM_PID,
  B2B_MINT_ADDRESS,
  CRP_B2B_LP_MINT,
  CRP_MINT_ADDRESS,
  DUAL_POOL_REWARD_TAG,
  FARM_POOL_LP_TAG,
  FARM_POOL_REWARD_TAG,
  FARM_TAG,
  GLOBAL_STATE_TAG,
  program,
  RENT_SYSVAR_ID,
  setupAll,
  SYSTEM_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  wallet,
} from "./setup";

describe("02. Farm Management", () => {
  let globalStateKey = null, globalStateKeyNonce = 0;
  let farmKey = null, farmKeyNonce = 0;

  const startTime = Date.now() / 1000;
  const endTime = startTime + 600;
  const dualStartTime = startTime + 150;
  const dualEndTime = dualStartTime + 300;
  const newEndTime = startTime + 1000;
  const newDualEndTime = dualStartTime + 400;

  it("create new farm", async () => {
    await setupAll();
    const newFarmSeed = anchor.web3.Keypair.generate();
    [globalStateKey, globalStateKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(GLOBAL_STATE_TAG)], program.programId);
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
          // poolLpToken: farmPoolLpKey,
          // poolRewardToken: farmPoolRewardKey,
          // ammSwap: AMM_PID,
          systemProgram: SYSTEM_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: RENT_SYSVAR_ID
        }
      }
    );
    console.log('txid=',tx);

    await program.account.farmPool.fetch(farmKey);
  });
  it("extend old farm", async () => {
    await setupAll();
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
    await setupAll();
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
    await setupAll();
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
