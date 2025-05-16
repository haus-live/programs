import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Haus } from "../target/types/haus";
import { Keypair } from "@solana/web3.js";
import { BN } from "bn.js";
import { expect } from "chai";
import { SessionTokenManager, SDK as ST } from "@magicblock-labs/gum-sdk";

function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

const standupComedy_artCategory = {
  standupComedy: {},
};

describe("haus", async () => {
  // Configure the client to use the local cluster.
  // const context = await startAnchor('', [
  //   { name: 'session_keys', programId: 'KeyspM2ssCJbqUhQ4k7sveSiY4WjnYsrXkC8oDbwde5' },
  //   { name: 'core_asset', programId: 'CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d' },
  //   { name: 'token_metadata', programId: 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'}
  // ], []);
  // const provider = new BankrunProvider(context);

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.haus as Program<Haus>;

  const payer = provider.wallet as anchor.Wallet;
  let event: anchor.web3.PublicKey;
  let current_timestamp = Date.now() / 1000;

  console.log("current ts!" + current_timestamp.toString());

  it("tests all instruction at once (the happy path only) as I didn't have time to do proper describe/it structure", async () => {
    const currentSlot = await program.provider.connection.getSlot();
    const currentBlocktime = await program.provider.connection.getBlockTime(
      currentSlot
    );
    console.log("curblocktime " + currentBlocktime.toString());

    /// ::create_event
    const realtime_asset = new Keypair();
    console.log("rta pubkey " + realtime_asset.publicKey);
    const createEventArgs = {
      name: "name",
      uri: "http://u.ri",
      beginTimestamp: new BN(current_timestamp),
      endTimestamp: new BN(current_timestamp + 10),
      reservePrice: new BN(1),
      ticketCollection: payer.publicKey,
      artCategory: standupComedy_artCategory,
      chunkUploader: payer.payer.publicKey,
    };
    let event_seeds = [
      Buffer.from(anchor.utils.bytes.utf8.encode("event")),
      realtime_asset.publicKey.toBuffer(),
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

    /// ::init_tipping_calculator
    let tipping_calculator_seeds = [
      Buffer.from(anchor.utils.bytes.utf8.encode("tipping_calculator")),
      realtime_asset.publicKey.toBuffer(),
      payer.publicKey.toBuffer(),
    ];
    const [tipping_calculator_pubkey, __] =
      anchor.web3.PublicKey.findProgramAddressSync(
        tipping_calculator_seeds,
        program.programId
      );
    try {
      const tx = program.methods
        .initTippingCalculator()
        .accountsPartial({
          realtimeAsset: realtime_asset.publicKey,
          event: event_pubkey,
        })
        .signers([payer.payer])
        .rpc();
      console.log(tx);
    } catch (e) {
      console.log(e);
      throw e;
    }

    /// ::make_tip
    const session_signer = new Keypair();
    console.log("session_signer ", session_signer.publicKey);
    const stm = new SessionTokenManager(payer, program.provider.connection);
    await stm.program.methods
      .createSession(true, new BN(current_timestamp + 1000), new BN(1000000000))
      .accountsPartial({
        // targetProgram: anchor.web3.SystemProgram.programId,
        targetProgram: program.programId,
        sessionSigner: session_signer.publicKey,
        authority: payer.payer.publicKey,
      })
      .signers([session_signer, payer.payer])
      .rpc();
    await delay(1000);
    const sstseeds = [
      Buffer.from(anchor.utils.bytes.utf8.encode("session_token")),
      program.programId.toBuffer(),
      session_signer.publicKey.toBuffer(),
      payer.payer.publicKey.toBuffer(),
    ];
    const [sstkey, ___] = anchor.web3.PublicKey.findProgramAddressSync(
      sstseeds,
      stm.program.programId
    );
    // const sstx = await stm.get(sstkey);
    // console.log(sstx.authority);
    // console.log(sstx.sessionSigner);
    // console.log("session token " + sstkey.toString());
    // console.log("tipping calc authority " + payer.payer.publicKey);

    // // const sstkey2

    console.log(tipping_calculator_pubkey);
    try {
      const tx = await program.methods
        .makeTip({
          amount: new BN(1),
          realtimeAssetKey: realtime_asset.publicKey,
        })
        .accountsPartial({
          event: event_pubkey,
          tippingCalculator: tipping_calculator_pubkey,
          signer: session_signer.publicKey,
          sessionToken: sstkey,
        })
        .signers([session_signer])
        .rpc();
      console.log(tx);
    } catch (e) {
      console.log(e);
      throw e;
    }
    const acc = await program.account.event.fetch(event_pubkey);
    console.log("total: " + acc.tippingLeaderTotal.toString());

    // await delay(10_000);

    /// ::upload_chunks
    try {
      const tx = await program.methods
        .loadChunks({
          uri: "u.r.i",
        })
        .accountsPartial({
          realtimeAsset: realtime_asset.publicKey,
          event: event_pubkey,
          authority: payer.payer.publicKey,
        })
        .signers([payer.payer])
        .rpc();
      console.log(tx);
    } catch (e) {
      console.log(e);
      throw e;
    }

    console.log("chunks uploaded");

    await delay(10_000);

    /// ::claim_realtime_asset
    try {
      const tx = await program.methods
        .claimRealtimeAsset()
        .accountsPartial({
          event: event_pubkey,
          realtimeAsset: realtime_asset.publicKey,
          authority: payer.payer.publicKey,
        })
        .signers([payer.payer])
        .rpc();
      console.log(tx);
    } catch (e) {
      console.log(e);
      throw e;
    }

    console.log("rta claimed");

    /// ::withdraw_tips
    try {
      const tx = await program.methods
        .withdrawTips()
        .accountsPartial({
          event: event_pubkey,
          realtimeAsset: realtime_asset.publicKey,
          authority: payer.payer.publicKey,
        })
        .signers([payer.payer])
        .rpc();
      console.log(tx);
    } catch (e) {
      console.log(e);
      throw e;
    }

    console.log("tips withdrawn");
  });
});
