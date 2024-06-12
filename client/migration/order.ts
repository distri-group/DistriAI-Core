// migrateOrderNew
const orders = await pg.program.account.order.all();
for (let order of orders) {
  const [orderNewPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("order-new"),
      order.account.buyer.toBuffer(),
      Uint8Array.from(order.account.orderId),
    ],
    pg.PROGRAM_ID
  );
  const txHash = await pg.program.methods
    .migrateOrderNew()
    .accounts({
      orderBefore: order.publicKey,
      orderAfter: orderNewPDA,
    })
    .rpc();
  await logTransaction(txHash);
}

// migrateOrderRename
const orderNews = await pg.program.account.orderNew.all();
for (let orderNew of orderNews) {
  const [orderPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("order"),
      orderNew.account.buyer.toBuffer(),
      Uint8Array.from(orderNew.account.orderId),
    ],
    pg.PROGRAM_ID
  );
  const txHash = await pg.program.methods
    .migrateOrderRename()
    .accounts({
      orderBefore: orderNew.publicKey,
      orderAfter: orderPDA,
    })
    .rpc();
  await logTransaction(txHash);
};

// adminRemoveOrder
let timeSeconds = new Date("2024-05-01T00:00:00Z").getTime() / 1000;
const allOrders = await pg.program.account.order.all();
for (let order of allOrders) {
  console.log(timeSeconds);
  console.log(order.account.orderTime.toNumber());

  if (order.account.orderTime.toNumber() < timeSeconds) {
    const txHash = await pg.program.methods
      .adminRemoveOrder()
      .accounts({
        order: order.publicKey,
      })
      .rpc();
    await logTransaction(txHash);
  }
  console.log("-----------------------");
}

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
