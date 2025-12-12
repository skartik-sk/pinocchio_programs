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
  AccountRole,
  generateKeyPair,
  getBase58Codec,
  getBase58Encoder,
  generateKeyPairSigner,
  SOLANA_ERROR__CODECS__INVALID_BYTE_LENGTH,
  addSignersToTransactionMessage,
  getBase64Codec,
  compressTransactionMessageUsingAddressLookupTables,
} from "@solana/kit";
import { createClient } from "../../clients";
import test, { describe } from "node:test";
function getInstructionData(info: UserData) {
  const data: number[] = [];

  // Add instruction discriminator
  data.push(0);
  // 40 + 10 + 150 + 50;
  // Pad name to 16 bytes (data[1..41])
  const nameBytes = Buffer.from(info.name, "utf-8");
  const namePadded = Buffer.alloc(40);
  nameBytes.copy(namePadded, 0, 0, Math.min(nameBytes.length, 40));
  data.push(...namePadded);

  // Add 1 byte padding at index 10
  const houseNumberBytes = Buffer.from(String(info.house_number), "utf-8");
  const houseNumberPadded = Buffer.alloc(10);
  houseNumberBytes.copy(
    houseNumberPadded,
    0,
    0,
    Math.min(houseNumberBytes.length, 10),
  );
  //(data[41--52])
  data.push(...houseNumberPadded);
  //const namePadded = Buffer.alloc(40);
  // Pad street to 16 bytes (data[53..204])
  const streetBytes = Buffer.from(info.street, "utf-8");
  const streetPadded = Buffer.alloc(150);
  streetBytes.copy(streetPadded, 0, 0, Math.min(streetBytes.length, 150));
  data.push(...streetPadded);

  // Pad city to 16 bytes (data[36..52])
  const cityBytes = Buffer.from(info.city, "utf-8");
  const cityPadded = Buffer.alloc(50);
  cityBytes.copy(cityPadded, 0, 0, Math.min(cityBytes.length, 50));
  data.push(...cityPadded);

  return Buffer.from(data);
}

interface UserData {
  name: string;
  house_number: number;
  street: string;
  city: string;
}

describe("Account Data!", async () => {
    const client = await createClient("localnet");
  const { value: balance } = await client.rpc
    .getBalance(client.wallet.address)
    .send();
  let latestBlockhash = (await client.rpc.getLatestBlockhash().send()).value;
  const programAddress = address(
    "7MPZ3vcJ3tdNEs1Q7zGXoPZaVrACfWmY1svaSJffgtQ9",
  );

  const systemProgram = address("11111111111111111111111111111111");

  const counterAccount = await generateKeyPairSigner();
  console.log(address);

  test("Create the address info account", async () => {
    const programTx: Instruction[] = [
    {
      programAddress,
      accounts: [
        {
          address: counterAccount.address,
          role: AccountRole.WRITABLE_SIGNER, // ← Changed this
        },
        {
          address: client.wallet.address,
          role: AccountRole.WRITABLE_SIGNER,
        },
        {
          address: systemProgram,
          role: AccountRole.READONLY,
        },
      ],
      data: Buffer.from([0]),
    },
  ];

  const transactionMessage = pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) => appendTransactionMessageInstructions(programTx, tx),
     (tx) => addSignersToTransactionMessage([counterAccount], tx),
  );

  // Compile the transaction message and sign it.
  const transaction =
    await signTransactionMessageWithSigners(transactionMessage);
  assertIsSendableTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);
  // Send the transaction and wait for confirmation.
  await client.sendAndConfirmTransaction(transaction, {
    commitment: "confirmed",
  });
  
  const data =getBase64Codec().encode((await client.rpc.getAccountInfo(counterAccount.address,{encoding:"base64"}).send()).value?.data[0]||'A')
  console.log(data)

  });
  test("Increment info account", async () => {
    const programTx: Instruction[] = [
    {
      programAddress,
      accounts: [
        {
          address: counterAccount.address,
          role: AccountRole.WRITABLE, // ← Changed this
        },
        
      ],
      data: Buffer.from([1]),
    },
  ];

  const transactionMessage = pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) => appendTransactionMessageInstructions(programTx, tx),
    //  (tx) => addSignersToTransactionMessage([counterAccount], tx),
  );

  // Compile the transaction message and sign it.
  const transaction =
    await signTransactionMessageWithSigners(transactionMessage);
  assertIsSendableTransaction(transaction);
  assertIsTransactionWithBlockhashLifetime(transaction);

  // Send the transaction and wait for confirmation.
  await client.sendAndConfirmTransaction(transaction, {
    commitment: "confirmed",
  });
  const data =getBase64Codec().encode((await client.rpc.getAccountInfo(counterAccount.address,{encoding:"base64"}).send()).value?.data[0]||'A')
  console.log(data)



  });
});
