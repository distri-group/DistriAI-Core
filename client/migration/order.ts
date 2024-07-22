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
  // Derive the Program Derived Address (PDA) for the order using the buyer's public key and order ID.
  // The PDA is generated using the 'findProgramAddressSync' method from the anchor.web3 library.
  // The seeds for the PDA include a buffer with the string "order", the buyer's public key as a buffer,
  // and the order ID as a Uint8Array.
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
