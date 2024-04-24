import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { Errors } from "../target/types/errors";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Errors as anchor.Program<Errors>;

// migrateMachineNew
const machines = await program.account.machine.all();
machines.forEach(async (machine) => {
  const [machineNewPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("machine-new"),
      machine.account.owner.toBuffer(),
      Uint8Array.from(machine.account.uuid),
    ],
    program.programId
  );
  const txHash = await program.methods
    .migrateMachineNew()
    .accounts({
      machineBefore: machine.publicKey,
      machineAfter: machineNewPDA,
    })
    .rpc();
  await logTransaction(txHash);
});

// migrateMachineRename
const machineNews = await program.account.machineNew.all();
machineNews.forEach(async (machineNew) => {
  const [machinePDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("machine"),
      machineNew.account.owner.toBuffer(),
      Uint8Array.from(machineNew.account.uuid),
    ],
    program.programId
  );
  const txHash = await program.methods
    .migrateMachineRename()
    .accounts({
      machineBefore: machineNew.publicKey,
      machineAfter: machinePDA,
    })
    .rpc();
  await logTransaction(txHash);
});

// logTransaction
async function logTransaction(txHash) {
  const { blockhash, lastValidBlockHeight } =
    await program.provider.connection.getLatestBlockhash();

  await program.provider.connection.confirmTransaction({
    blockhash,
    lastValidBlockHeight,
    signature: txHash,
  });

  console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
}
