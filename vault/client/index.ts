import {
    airdropFactory,
    createSolanaRpc,
    createSolanaRpcSubscriptions,
    generateKeyPairSigner,
    lamports,
    MessageSigner,
    RpcSubscriptions,
    SolanaRpcApi,
    SolanaRpcSubscriptionsApi,
     sendAndConfirmTransactionFactory, 
    TransactionSigner,
    Rpc,
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
    getProgramDerivedAddress,
    getAddressEncoder,
    getBytesEncoder,
} from '@solana/kit';
import { createClient } from './clients';
function getDepositInstructionData(amount: bigint) {
    const buffer = new ArrayBuffer(9); // 1 byte (discriminator) + 8 bytes (amount)
    const view = new DataView(buffer);
    
    view.setUint8(0, 0); // The Discriminator (0 for Deposit)
    view.setBigUint64(1, amount, true); // The Amount (Little Endian)
    
    return new Uint8Array(buffer);
}
function getWithdrawInstructionData() {
    const buffer = new ArrayBuffer(1); // 1 byte (discriminator) + 8 bytes (amount)
    const view = new DataView(buffer);
    
    view.setUint8(0, 1); // The Discriminator (1 for Withdraw)
    return new Uint8Array(buffer);
}
async function test() {
  const client = await createClient();
    const { value: balance } = await client.rpc.getBalance(client.wallet.address).send();
    console.log(`Balance: ${balance} lamports.`);
let latestBlockhash =    (await client.rpc.getLatestBlockhash().send()).value

const programAddress = address('C74nvDyYRN9KbvY44G8ucnzXgjFXWLHyG5y58UTqaNEa');
const addressEncoder = getAddressEncoder();
const vaultAccount2 = await getProgramDerivedAddress({
    programAddress,
    seeds:
     [
       getBytesEncoder().encode(
                new Uint8Array([118, 97, 117, 108, 116])
              ), addressEncoder.encode( client.wallet.address)],
})
let vaultAccount = await getProgramDerivedAddress({
    programAddress,
    seeds:
     [
      'vault', addressEncoder.encode( client.wallet.address)],
})
// const vaultAccount =await generateKeyPairSigner();
console.log(vaultAccount)
console.log(client.wallet.address)
     const programTx :Instruction[]=[
        {
            programAddress,
            accounts: [
                {
                    address: client.wallet.address,
                    role:AccountRole.WRITABLE_SIGNER
                },
                {
                    address: vaultAccount[0],
                    role:AccountRole.WRITABLE
                },
               {
                    address: address('11111111111111111111111111111111'), 
                    role: AccountRole.READONLY,
                },
            ],
            data:getDepositInstructionData(10000000n)
            
        }
     ]
    const transactionMessage = await pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
        (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
        (tx) => appendTransactionMessageInstructions(programTx, tx),
        // (tx) => client.estimateAndSetComputeUnitLimit(tx),
    );
 
    // Compile the transaction message and sign it.
    const transaction = await signTransactionMessageWithSigners(transactionMessage);
    assertIsSendableTransaction(transaction);
     assertIsTransactionWithBlockhashLifetime(transaction);
 
    // Send the transaction and wait for confirmation.
    await client.sendAndConfirmTransaction(transaction, { commitment: 'confirmed' }); 
    const { value: vaultBalance } = await client.rpc.getBalance(vaultAccount[0]).send();
    const { value: newBalance } = await client.rpc.getBalance(client.wallet.address).send();
    console.log(`vault Balance: ${vaultBalance} lamports.`);
    console.log(`New Balance: ${newBalance} lamports.`);





     const programTxs2 :Instruction[]=[
        {
            programAddress,
            accounts: [
                {
                    address: client.wallet.address,
                    role:AccountRole.WRITABLE_SIGNER
                },
                {
                    address: vaultAccount[0],
                    role:AccountRole.WRITABLE   
                },
               {
                    address: address('11111111111111111111111111111111'), 
                    role: AccountRole.READONLY,
                },
            ],
            data:getWithdrawInstructionData()
            
        }
     ]
    const transactionMessage2 = await pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
        (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
        (tx) => appendTransactionMessageInstructions(programTxs2, tx),
        // (tx) => client.estimateAndSetComputeUnitLimit(tx),
    );
 
    // Compile the transaction message and sign it.
    const transaction2 = await signTransactionMessageWithSigners(transactionMessage2);
    assertIsSendableTransaction(transaction2);
     assertIsTransactionWithBlockhashLifetime(transaction2);
 
    // Send the transaction and wait for confirmation.
    await client.sendAndConfirmTransaction(transaction2, { commitment: 'confirmed' }); 
    const { value: newBalance2 } = await client.rpc.getBalance(client.wallet.address).send();
    console.log(`New Balance: ${newBalance2} lamports.`);
}



test();
