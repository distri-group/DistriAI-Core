// No imports needed: web3, anchor, pg and more are globally available

describe("Test", () => {
  it("machine", async () => {
    const uuid = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    const metadata = "{}";
    const [machinePDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("machine"),
        pg.wallet.publicKey.toBuffer(),
        Uint8Array.from(uuid),
      ],
      pg.PROGRAM_ID
    );

    // addMachine
    let txHash = await pg.program.methods
      .addMachine(uuid, metadata)
      .accounts({
        machine: machinePDA,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    await pg.connection.confirmTransaction(txHash);

    // makeOffer
    const price = new BN(10_000_000_000);
    const maxDuration = 100;
    const disk = 1000;
    txHash = await pg.program.methods
      .makeOffer(price, maxDuration, disk)
      .accounts({
        machine: machinePDA,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    await pg.connection.confirmTransaction(txHash);

    // cancelOffer
    txHash = await pg.program.methods
      .cancelOffer()
      .accounts({
        machine: machinePDA,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    await pg.connection.confirmTransaction(txHash);

    // removeMachine
    txHash = await pg.program.methods
      .removeMachine()
      .accounts({
        machine: machinePDA,
      })
      .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    await pg.connection.confirmTransaction(txHash);
  });
});
