import * as anchor from '@project-serum/anchor';
import assert from 'assert';
import {
  AMM_PID,
  FEE_OWNER,
  GLOBAL_STATE_TAG,
  HARVEST_FEE_DENOMINATOR,
  HARVEST_FEE_NUMERATOR,
  program,
  RENT_SYSVAR_ID,
  setupAll,
  SYSTEM_PROGRAM_ID,
  wallet,
} from "./setup";

describe("01. Manage global state", () => {
  let globalStateKey = null, globalStateKeyNonce = 0;

  it("Create global state", async () => {
    await setupAll();
    [globalStateKey, globalStateKeyNonce] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from(GLOBAL_STATE_TAG)], program.programId);
    
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
    await setupAll();
    
    const tx = await program.rpc.createGlobalState(
      globalStateKeyNonce,
      HARVEST_FEE_NUMERATOR.add(new anchor.BN(1)),
      HARVEST_FEE_DENOMINATOR.add(new anchor.BN(1000)),
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
    const globalState = await program.account.farmProgram.fetch(globalStateKey);
    assert(globalState.harvestFeeDenominator.toNumber() === HARVEST_FEE_DENOMINATOR.toNumber() + 1000);
  });
});
