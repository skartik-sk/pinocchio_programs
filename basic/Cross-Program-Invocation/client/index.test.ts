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
function getInstructionData(info: Power) {
  const data: number[] = [];

  // Add instruction discriminator
  data.push(0);
  data.push(0);
  // 40 + 10 + 150 + 50;
  // Pad name to 16 bytes (data[1..41])
  const nameBytes = Buffer.from(info.name, "utf-8");
  const namePadded = Buffer.alloc(39);
  nameBytes.copy(namePadded, 0, 0, Math.min(nameBytes.length, 39));
  data.push(...namePadded);

  
  return Buffer.from(data);
}

interface Power {
  name: string;
 
}

describe("Account Data!", async () => {
    const client = await createClient("localnet");
  const { value: balance } = await client.rpc
    .getBalance(client.wallet.address)
    .send();
  let latestBlockhash = (await client.rpc.getLatestBlockhash().send()).value;
  const handprogramAddress = address(
    "4Euu33H2aSscDxaWLenfszUD95QAM1NFtRon62WLxhgf",
  );

  const levelprogramAddress = address(
    "5J7yQ8tDgA73nSCBAitKiAakp153WTtztiJELrAzLFbq",
  );
const info:Power={
  name:"stranger_things"
}
  const systemProgram = address("11111111111111111111111111111111");

  const PowerAccount = await generateKeyPairSigner();
  console.log(address);

  test("Create the address of liver", async () => {
    const programTx: Instruction[] = [
    {
      programAddress:levelprogramAddress,
      accounts: [
        {
          address: client.wallet.address,
          role: AccountRole.WRITABLE_SIGNER,
        },
        {
          address: PowerAccount.address,
          role: AccountRole.WRITABLE_SIGNER, // ← Changed this
        },
        {
          address: systemProgram,
          role: AccountRole.READONLY,
        },
      ],
      data: getInstructionData(info)
    },
  ];

  const transactionMessage = pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) => appendTransactionMessageInstructions(programTx, tx),
     (tx) => addSignersToTransactionMessage([PowerAccount], tx),
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
  
  const data =getBase64Codec().encode((await client.rpc.getAccountInfo(levelprogramAddress,{encoding:"base64"}).send()).value?.data[0]||'A')
  console.log("level",data)
const data1 =getBase64Codec().encode((await client.rpc.getAccountInfo(PowerAccount.address,{encoding:"base64"}).send()).value?.data[0]||'A')
  console.log("power",data1)
  });
   test("Toggle liver", async () => {
    console.log(levelprogramAddress,client.wallet.address,PowerAccount.address,systemProgram)
    const programTx: Instruction[] = [
    {
      programAddress:levelprogramAddress,
      accounts: [
        {
          address: client.wallet.address,
          role: AccountRole.WRITABLE_SIGNER,
        },
        {
          address: PowerAccount.address,
          role: AccountRole.WRITABLE_SIGNER, // ← Changed this
        },
        {
          address: systemProgram,
          role: AccountRole.READONLY,
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
     (tx) => addSignersToTransactionMessage([PowerAccount], tx),
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
  const data =getBase64Codec().encode((await client.rpc.getAccountInfo(PowerAccount.address,{encoding:"base64"}).send()).value?.data[0]||'A')
  console.log(data)



  });
  test("Toggle liver using cpi ", async () => {
    console.log(levelprogramAddress,client.wallet.address,PowerAccount.address,systemProgram)
    const programTx: Instruction[] = [
    {
      programAddress:handprogramAddress,
      accounts: [
        {
          address: levelprogramAddress,
          role: AccountRole.READONLY,
        },
        {
          address: client.wallet.address,
          role: AccountRole.WRITABLE_SIGNER,
        },
        {
          address: PowerAccount.address,
          role: AccountRole.WRITABLE_SIGNER, // ← Changed this
        },
        {
          address: systemProgram,
          role: AccountRole.READONLY,
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
     (tx) => addSignersToTransactionMessage([PowerAccount], tx),
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
  const data =getBase64Codec().encode((await client.rpc.getAccountInfo(PowerAccount.address,{encoding:"base64"}).send()).value?.data[0]||'A')
  console.log(data)



  });
});
