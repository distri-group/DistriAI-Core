import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { Errors } from "../target/types/errors";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Errors as anchor.Program<Errors>;

// migrateOrderNew
const orders = await program.account.order.all();
// Iterate through each order asynchronously
orders.forEach(async (order) => {
  // Generate a new program-derived address (PDA) for the order
  const [orderNewPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("order-new"),
      order.account.buyer.toBuffer(),
      Uint8Array.from(order.account.orderId),
    ],
    program.programId
  );
  // Execute a transaction to migrate the order to the new PDA
  const txHash = await program.methods
    .migrateOrderNew()
    .accounts({
      orderBefore: order.publicKey,
      orderAfter: orderNewPDA,
    })
    .rpc();
  await logTransaction(txHash);
});

// migrateOrderRename
const orderNews = await program.account.orderNew.all();
orderNews.forEach(async (orderNew) => {
  const [orderPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("order"),
      orderNew.account.buyer.toBuffer(),
      Uint8Array.from(orderNew.account.orderId),
    ],
    program.programId
  );
  const txHash = await program.methods
    .migrateOrderRename()
    .accounts({
      orderBefore: orderNew.publicKey,
      orderAfter: orderPDA,
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
