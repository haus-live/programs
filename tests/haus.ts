import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Haus } from "../target/types/haus";
import { Keypair } from "@solana/web3.js";
import { BN } from "bn.js";

function delay(ms: number) {
  return new Promise( resolve => setTimeout(resolve, ms) );
}

const standupComedy_artCategory = {
  standupComedy: {},
};

describe("haus", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.haus as Program<Haus>;

  const payer = provider.wallet as anchor.Wallet;
  let event: anchor.web3.PublicKey;
  let current_timestamp = Date.now() / 1000;

  console.log("current ts!" + current_timestamp.toString());

  it("creates event", async () => {
    const currentSlot = await program.provider.connection.getSlot();
    const currentBlocktime = await program.provider.connection.getBlockTime(currentSlot);
    console.log("curblocktime" + currentBlocktime.toString());
    // Add your test here.
    const realtime_asset = new Keypair();
    const createEventArgs = {
      name: "name",
      uri: "http://u.ri",
      beginTimestamp: new BN(current_timestamp),
      endTimestamp: new BN(current_timestamp + 900),
      reservePrice: new BN(1),
      ticketCollection: payer.publicKey,
      artCategory: standupComedy_artCategory,
    };
    let event_seeds = [
      Buffer.from(anchor.utils.bytes.utf8.encode("event")),
      payer.publicKey.toBuffer(),
    ];
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
    event = event_pubkey;
    console.log("success!");

    // sync localnet timestamp
    await delay(2000);

    const tipping_calculator_seeds = [
      Buffer.from(anchor.utils.bytes.utf8.encode("tipping_calculator")),
      event.toBuffer(),
      payer.publicKey.toBuffer(),
    ];
    const [tipping_calculator_pubkey, __] = anchor.web3.PublicKey.findProgramAddressSync(
        tipping_calculator_seeds,
        program.programId
      );
    console.log(tipping_calculator_pubkey);
    try {
      const tx = await program.methods
        .makeTip({ amount: new BN(1) })
        .accountsPartial({
          event: event,
          tippingCalculator: tipping_calculator_pubkey,
          authority: payer.payer.publicKey,
        })
        .signers([payer.payer])
        .rpc();
      console.log(tx);
    } catch (e) {
      throw e;
    }
    const acc = await program.account.event.fetch(
      event_pubkey
    );
    console.log("total: " + acc.tippingLeaderTotal.toString());
  });
});
