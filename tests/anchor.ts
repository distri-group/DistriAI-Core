import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { Errors } from "../target/types/errors";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Errors as anchor.Program<Errors>;
  
  it("machine", async () => {
    const uuid = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const metadata = "{}";
    const [machinePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("machine"),
        program.provider.publicKey.toBuffer(),
        Uint8Array.from(uuid),
      ],
      program.programId
    );

    // addMachine
    let txHash = await program.methods
      .addMachine(uuid, metadata)
      .accounts({
        machine: machinePDA,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    await program.provider.connection.confirmTransaction(txHash);

    // makeOffer
    const price = new BN(10_000_000_000);
    const maxDuration = 100;
    const disk = 1000;
    txHash = await program.methods
      .makeOffer(price, maxDuration, disk)
      .accounts({
        machine: machinePDA,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    await program.provider.connection.confirmTransaction(txHash);

    // cancelOffer
    txHash = await program.methods
      .cancelOffer()
      .accounts({
        machine: machinePDA,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    await program.provider.connection.confirmTransaction(txHash);

    // removeMachine
    txHash = await program.methods
      .removeMachine()
      .accounts({
        machine: machinePDA,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    await program.provider.connection.confirmTransaction(txHash);
  });
});
