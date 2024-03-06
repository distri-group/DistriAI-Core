// Migration

const orders = await pg.program.account.order.all();
console.log(orders.length);

orders.forEach(async (order) => {
  const [orderNewPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("order-new"),
      order.account.buyer.toBuffer(),
      order.account.orderId,
    ],
    pg.PROGRAM_ID
  );
  const txHash = await pg.program.methods
    .migrateOrderNew()
    .accounts({
      order: order.publicKey,
      orderNew: orderNewPDA,
    })
    .rpc();
  await logTransaction(txHash);
});

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
