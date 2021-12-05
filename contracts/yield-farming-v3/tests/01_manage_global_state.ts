import * as anchor from '@project-serum/anchor';
import { AnchorFarm } from '../target/types/anchor_farm';
import {
  AMM_PID,
  FEE_OWNER,
  globalStateKey,
  globalStateKeyNonce,
  HARVEST_FEE_DENOMINATOR,
  HARVEST_FEE_NUMERATOR,
  program,
  RENT_SYSVAR_ID,
  setupAll,
  SYSTEM_PROGRAM_ID,
  wallet,
} from "./setup";

describe("Manage global state", () => {
  it("Create global state", async () => {
    await setupAll();
    
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
    console.log('tx id',tx);
  });
});
