import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Haus } from "../target/types/haus";
import { Keypair, SendTransactionError } from "@solana/web3.js";
import { BN } from "bn.js";

const standupComedy_artCategory = {
  standupComedy: {},
};

describe("haus", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);



  const program = anchor.workspace.haus as Program<Haus>;
  // const mpl_core_program = anchor.

  it("creates event", async () => {
    // Add your test here.
    const payer = provider.wallet as anchor.Wallet;
    const realtime_asset = new Keypair();
    const createEventArgs = {
      'name': 'name',
      'uri': 'http://u.ri',
      'beginTimestamp': new BN(0),
      'endTimestamp': new BN(900),
      'reservePrice': new BN(1),
      'ticketCollection': payer.publicKey,
      'artCategory': standupComedy_artCategory
    }
    let event_seeds = [
      Buffer.from(anchor.utils.bytes.utf8.encode("event")),
      payer.publicKey.toBuffer(),
    ]
    const [event_pubkey, _] = anchor.web3.PublicKey.findProgramAddressSync(
      event_seeds,
      program.programId
    );
    console.log(event_pubkey);
    try {
      const tx = await program.methods
        .createEvent(createEventArgs)
        .accountsPartial({
          realtimeAsset: realtime_asset.publicKey,
          authority: payer.payer.publicKey,
          event: event_pubkey,
        })
        .signers([realtime_asset, payer.payer])
        .rpc();
      console.log(tx);
    } catch (err) {
      console.error(err);
      throw err;
    }
    console.log("success!");
  });
});
