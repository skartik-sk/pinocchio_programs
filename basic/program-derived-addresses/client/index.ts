import {
  assertIsSendableTransaction,
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createTransactionMessage,
  pipe,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  Instruction,
  address,
  getProgramDerivedAddress,
  AccountRole,
  generateKeyPairSigner,
  getBase64EncodedWireTransaction,
  getBase58Encoder,
  getAddressEncoder,
  airdropFactory,
  lamports,
} from "@solana/kit";
import { createClient } from "../../clients";
import { setTimeout } from "timers/promises";
function getInstructionData(bump: any) {
  const buffer = new ArrayBuffer(6); // 1 byte (discriminator) + 8 bytes (amount)
  const view = new DataView(buffer);

  view.setUint8(0, 0); // The Discriminator (0 for Deposit)
  view.setUint32(1, 0);
  view.setUint8(5, bump); // The Discriminator (0 for Deposit)
  // The Amount (Little Endian)

  return new Uint8Array(buffer);
}
async function test() {
  const client = await createClient("localnet");
  const { value: balance } = await client.rpc
    .getBalance(client.wallet.address)
    .send();
  console.log(`Balance: ${balance} lamports.`);
  let latestBlockhash = (await client.rpc.getLatestBlockhash().send()).value;

  const programAddress = address(
    "CQc7fkDzruPe1qNZDtCeqKBw7kCr3CgrabsPpebsxcmx",
  );
  const system = address("11111111111111111111111111111111");
  const user = (await generateKeyPairSigner()).address;

  console.log(client.wallet.address);
  const bumps = Uint8Array.from([0]);
  let [pda, bump] = await getProgramDerivedAddress({
    programAddress: programAddress,
    seeds: ["page_visits", getAddressEncoder().encode(user)],
  });
  console.log("PDA:", pda, "Bump:", bump);

  const programTx: Instruction[] = [
    {
      programAddress,
      accounts: [
        {
          address: client.wallet.address,
          role: AccountRole.WRITABLE_SIGNER,
        },
        {
          address: user,
          role: AccountRole.READONLY,
        },
        {
          address: pda,
          role: AccountRole.WRITABLE,
        },
        {
          address: system,
          role: AccountRole.READONLY,
        },
      ],
      data: getInstructionData(bump),
    },
  ];
  //console.log("program tx" , programTx)
  const transactionMessage = pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) => appendTransactionMessageInstructions(programTx, tx),
    // (tx) => client.estimateAndSetComputeUnitLimit(tx),
  );

  //   console.log("trasection message", transactionMessage)

  // Compile the transaction message and sign it.
  const transaction =
    await signTransactionMessageWithSigners(transactionMessage);
  assertIsSendableTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);
  // console.log("trasection", transaction)
  const serializedTransaction = getBase64EncodedWireTransaction(transaction);
  const similatio = await client.rpc
    .simulateTransaction(serializedTransaction, { encoding: "base64" })
    .send();

  // Log simulation results
  // console.log("=== Simulation Results ===");
  // console.log("Logs:", similatio.value.logs);
  console.log("Units consumed:", similatio.value.unitsConsumed?.toString());

  if (similatio.value.err) {
    console.error("❌ Simulation failed:", similatio.value.err);
    console.error("Program logs:", similatio.value.logs);
    // Convert BigInt to string before stringifying

    throw new Error(
      `Transaction simulation failed: ${similatio.value.err.toString()}`,
    );
  }

  console.log("✅ Simulation succeeded");
  if (similatio.value.returnData) {
    console.log("Return data:", similatio.value.returnData);
  }
  // Send the transaction and wait for confirmation.
  await client.sendAndConfirmTransaction(transaction, {
    commitment: "confirmed",
  });
  const { value: newBalance } = await client.rpc
    .getBalance(client.wallet.address)
    .send();
  console.log(`New Balance: ${newBalance} lamports.`);
}

async function test2() {
  const client = await createClient("localnet");
  const { value: balance } = await client.rpc
    .getBalance(client.wallet.address)
    .send();
  console.log(`client Balance: ${balance} lamports.`);
  let latestBlockhash = (await client.rpc.getLatestBlockhash().send()).value;

  const programAddress = address(
    "CQc7fkDzruPe1qNZDtCeqKBw7kCr3CgrabsPpebsxcmx",
  );
  const system = address("11111111111111111111111111111111");
  const user = (await generateKeyPairSigner());
  const airdrop = airdropFactory({ rpc: client.rpc, rpcSubscriptions: client.rpcSubscriptions });
  await airdrop({
    recipientAddress: user.address,
    lamports: lamports(1_00_00_00_00_00_00_00n),
    commitment: 'finalized',
  });
  const { value: balance2 } = await client.rpc
    .getBalance(user.address)
    .send();
  console.log(`user Balance: ${balance2} lamports.`);
  console.log(user.address);
  const bumps = Uint8Array.from([0]);
  let [pda, bump] = await getProgramDerivedAddress({
    programAddress: programAddress,
    seeds: ["page_visits", getAddressEncoder().encode(user.address)],
  });
  console.log("PDA:", pda, "Bump:", bump);
  const programTx: Instruction[] = [
    {
      programAddress,
      accounts: [
        {
          address: address("2vJPt3h1yYPks6mYJnNKmF2UrcfzZSi8dyTMsdfzcaWX"),
          role: AccountRole.WRITABLE,
        },
      ],
      data: Buffer.from([1]),
      
    },
  ];
  //console.log("program tx" , programTx)
  const transactionMessage = pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayerSigner(user, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) => appendTransactionMessageInstructions(programTx, tx),
    // (tx) => client.estimateAndSetComputeUnitLimit(tx),
  );

  //   console.log("trasection message", transactionMessage)

  // Compile the transaction message and sign it.
  const transaction =
    await signTransactionMessageWithSigners(transactionMessage);
  assertIsSendableTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);
  // console.log("trasection", transaction)
  const serializedTransaction = getBase64EncodedWireTransaction(transaction);
  const similatio = await client.rpc
    .simulateTransaction(serializedTransaction, { encoding: "base64" })
    .send();

  // Log simulation results
  // console.log("=== Simulation Results ===");
  // console.log("Logs:", similatio.value.logs);
  console.log("Units consumed:", similatio.value.unitsConsumed?.toString());

  if (similatio.value.err) {
    console.error("❌ Simulation failed:", similatio.value.err);
    console.error("Program logs:", similatio.value.logs);
    // Convert BigInt to string before stringifying

    throw new Error(
      `Transaction simulation failed: ${similatio.value.err.toString()}`,
    );
  }

  console.log("✅ Simulation succeeded");
  if (similatio.value.returnData) {
    console.log("Return data:", similatio.value.returnData);
  }
  // Send the transaction and wait for confirmation.
  await client.sendAndConfirmTransaction(transaction, {
    commitment: "confirmed",
  });
  const { value: newBalance } = await client.rpc
    .getBalance(client.wallet.address)
    .send();
  console.log(`New Balance: ${newBalance} lamports.`);
}
// test();
test2();

