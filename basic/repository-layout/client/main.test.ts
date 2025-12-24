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
  getBase64Encoder,
  getBase58Decoder,
} from "@solana/kit";
import { createClient } from "../../clients";
import test, { describe } from "node:test";

import { InitializeStateInstructionData, getInitializeStateInstruction } from "./index"



describe("Account Data!", async () => {
  const client = await createClient("localnet");
  let latestBlockhash = (await client.rpc.getLatestBlockhash().send()).value;
const programId = address("EuNENfWwZPyrHznUFNtqU3aBJoqyGJmrKeGXtXLWqkgb");
  const systemProgram = address("11111111111111111111111111111111");

  const [counterAccount,bump] = await getProgramDerivedAddress({
    programAddress:programId,
    seeds: [
      "mystatev2",
      getBase58Encoder().encode(client.wallet.address as string),
    ],
  });
  console.log(counterAccount)
  console.log("9Zskedr5JNQUj1tcXv289n6LNNb382ZauwH6BYwJM9xn")
  const RentAccount = await generateKeyPairSigner();
  test("Create the address info account", async () => {
    // Pad data to exactly 32 bytes
    const rawData = Array.from(getBase64Encoder().encode("Hello+Solana"));
    const userdata = [...rawData, ...Array(32 - rawData.length).fill(0)].slice(0, 32);
    // console.log("this is the data", userdata, "length:", userdata.length);
    
    let programTx = [getInitializeStateInstruction({
      payerAcc: client.wallet,
      stateAcc: counterAccount,
      // Correct Rent sysvar address
      sysvarRentAcc: address("SysvarRent111111111111111111111111111111111"),
      systemProgramAcc: systemProgram,
      initializeMyStateV1IxData: {
        owner: client.wallet.address ,
        data: userdata,  // Now exactly 32 bytes
      },
    })]
    
    
    const transactionMessage = pipe(
      createTransactionMessage({ version: 0 }),
      (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
      (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
      (tx)=>appendTransactionMessageInstructions(programTx,tx),
      // (tx) => addSignersToTransactionMessage([counterAccount], tx),
    );
    // console.log(transactionMessage)

    // Compile the transaction message and sign it.
    const transaction = await signTransactionMessageWithSigners(transactionMessage);
    assertIsSendableTransaction(transaction);
    assertIsTransactionWithBlockhashLifetime(transaction);
    const serializedTransaction = getBase64EncodedWireTransaction(transaction);
    const similatio = await client.rpc.simulateTransaction(serializedTransaction, { encoding: "base64" }).send();
    
    // Log simulation results
    console.log("=== Simulation Results ===");
    console.log("Logs:", similatio.value.logs);
    console.log("Units consumed:", similatio.value.unitsConsumed?.toString());
    
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
    
    // Send the transaction and wait for confirmation.
    await client.sendAndConfirmTransaction(transaction, {
      commitment: "confirmed",
    });

    const data = 
        (await client.rpc
          .getAccountInfo(counterAccount, { encoding: "base64" })
          .send())
      

    console.log("this is account data",data);
  });
 
});
