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
  getBase64EncodedWireTransaction,
  getProgramDerivedAddress,
  getAddressEncoder,
  getAddressDecoder,
} from "@solana/kit";
import { createClient } from "../../clients";
import test, { describe } from "node:test";
function getInstructionData(info: UserData) {
   const data: number[] = [];
  // Add instruction discriminator
  data.push(0);
  // 32+8+8+64
  // Pad name to 16 bytes (data[1..41])
 const ownerBytes = getAddressEncoder().encode(address(info.owner));
  data.push(...ownerBytes);

  // Add 1 byte padding at index 10
 const houseNumberBuffer = Buffer.alloc(8);
  houseNumberBuffer.writeBigUInt64LE(BigInt(info.number));
  data.push(...houseNumberBuffer);
  //const namePadded = Buffer.alloc(40);
  // Pad street to 16 bytes (data[53..204])
  const streetBytes = Buffer.from(info.colorhex, "utf-8");
  const streetPadded = Buffer.alloc(8);
  streetBytes.copy(streetPadded, 0, 0, Math.min(streetBytes.length, 8));
  data.push(...streetPadded);

  // Pad city to 16 bytes (data[36..52])
  for (let i = 0; i < 4; i++) {
    const streetBytes = Buffer.from(info.hobbies[i], "utf-8");
    const streetPadded = Buffer.alloc(16);
    streetBytes.copy(streetPadded, 0, 0, Math.min(streetBytes.length, 16));
    data.push(...streetPadded);
  }
  console.log(data);
  

  return Buffer.from(data);
}

interface UserData {
   owner:string,
   number: number,
   colorhex:string
   hobbies: string[],
}

describe("Account Data!", async () => {
    const client = await createClient("localnet");
  let latestBlockhash = (await client.rpc.getLatestBlockhash().send()).value;
  const programAddress = address(
    "DYNzaGUHfqsEPkYE6x7Fomaqu1N5FS5ne7csFLbwoEK5",
  );

  const systemProgram = address("11111111111111111111111111111111");
 
  const [counterAccount,bump] = await getProgramDerivedAddress({programAddress,seeds:["my_fav", getAddressEncoder().encode(client.wallet.address)]});
  console.log(counterAccount);

  test("Create the fav account", async () =>
  {
    let IXdata: UserData = {
      
     owner:client.wallet.address,
     number: 20,
      colorhex: "#008080",
     hobbies: ["chess","casio","coding","tech"],
  }
    const programTx: Instruction[] = [
    {
      programAddress,
      accounts: [
        {
          address: counterAccount,
          role: AccountRole.WRITABLE, // ← Changed this
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
      data: getInstructionData(IXdata),
    },
  ];

  const transactionMessage = pipe(
    createTransactionMessage({ version: 0 }),
    (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) => appendTransactionMessageInstructions(programTx, tx),
  );

  // Compile the transaction message and sign it.
  const transaction =
    await signTransactionMessageWithSigners(transactionMessage);
  assertIsSendableTransaction(transaction);
    assertIsTransactionWithBlockhashLifetime(transaction);
    const serializedTransaction = getBase64EncodedWireTransaction(transaction);
    const similatio = await client.rpc.simulateTransaction(serializedTransaction, { encoding: "base64" }).send();
    if (similatio.value.err) {
      console.error("❌ Simulation failed:", similatio.value.err);
      console.error("Program logs:", similatio.value.logs);
      // Convert BigInt to string before stringifying
      
      throw new Error(`Transaction simulation failed: ${similatio.value.err.toString()}`);
    }
    
    console.log("✅ Simulation succeeded");
    if (similatio.value.returnData) {
      console.log("Return data:", similatio.value.returnData);
    }
    console.log("Program logs:", similatio.value.logs);
  // Send the transaction and wait for confirmation.
  await client.sendAndConfirmTransaction(transaction, {
    commitment: "confirmed",
  });
  
   const data =getBase64Codec().encode((await client.rpc.getAccountInfo(counterAccount,{encoding:"base64"}).send()).value?.data[0]||'A')
  console.log(data)

  });
  test("get info account", async () => {
    const programTx: Instruction[] = [
    {
      programAddress,
      accounts: [
        {
          address: counterAccount,
          role: AccountRole.READONLY, // ← Changed this
        },
        {
          address: client.wallet.address,
          role: AccountRole.READONLY_SIGNER, // ← Changed this
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
    const serializedTransaction = getBase64EncodedWireTransaction(transaction);
    const similatio = await client.rpc.simulateTransaction(serializedTransaction, { encoding: "base64" }).send();
  if (similatio.value.err) {
    console.error("❌ Simulation failed:", similatio.value.err);
    console.error("Program logs:", similatio.value.logs);
    // Convert BigInt to string before stringifying
 
    throw new Error(`Transaction simulation failed: ${similatio.value.err.toString()}`);
  }
     console.log("Program logs:", similatio.value.logs);
  
    console.log("✅ Simulation succeeded");
    
  if (similatio.value.returnData) {
    console.log("Return data:", similatio.value.returnData);
  }
  // Send the transaction and wait for confirmation.
  await client.sendAndConfirmTransaction(transaction, {
    commitment: "confirmed",
  });
  



  });
});
