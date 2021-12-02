import * as anchor from '@project-serum/anchor';
import { AnchorFarm } from '../target/types/anchor_farm';

describe('anchor_farm', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.AnchorFarm as anchor.Program<AnchorFarm>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
