import * as anchor from '@project-serum/anchor';
import { AnchorFarm } from '../target/types/anchor_farm';
import {
  globalStateKey,
  globalStateKeyNonce,
  program,
  setupAll,
  wallet,
} from "./setup";

describe("here", () => {
  it("here", async () => {
    await setupAll();
  });
});
