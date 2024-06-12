// migrateMachineNew
const machines = await pg.program.account.machine.all();
for (let machine of machines) {
  const [machineNewPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("machine-new"),
      machine.account.owner.toBuffer(),
      Uint8Array.from(machine.account.uuid),
    ],
    pg.PROGRAM_ID
  );
  const txHash = await pg.program.methods
    .migrateMachineNew()
    .accounts({
      machineBefore: machine.publicKey,
      machineAfter: machineNewPDA,
    })
    .rpc();
  await logTransaction(txHash);
};

// migrateMachineRename
const machineNews = await pg.program.account.machineNew.all();
for (let machineNew of machineNews) {
  const [machinePDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("machine"),
      machineNew.account.owner.toBuffer(),
      Uint8Array.from(machineNew.account.uuid),
    ],
    pg.PROGRAM_ID
  );
  const txHash = await pg.program.methods
    .migrateMachineRename()
    .accounts({
      machineBefore: machineNew.publicKey,
      machineAfter: machinePDA,
    })
    .rpc();
  await logTransaction(txHash);
};

// logTransaction
async function logTransaction(txHash) {
  const { blockhash, lastValidBlockHeight } =
    await pg.connection.getLatestBlockhash();

  await pg.connection.confirmTransaction({
    blockhash,
    lastValidBlockHeight,
    signature: txHash,
  });

  console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
}
