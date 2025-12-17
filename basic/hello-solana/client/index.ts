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
        getSignatureFromTransaction,
        getBase64EncodedWireTransaction,
    } from '@solana/kit';
    import { createClient } from '../../clients';
    function getInstructionData(amount: bigint) {
        const buffer = new ArrayBuffer(9); // 1 byte (discriminator) + 8 bytes (amount)
        const view = new DataView(buffer);
        
        view.setUint8(0, 0); // The Discriminator (0 for Deposit)
        view.setBigUint64(1, amount, true); // The Amount (Little Endian)
        
        return new Uint8Array(buffer);
    }
    async function test() {
      const client = await createClient("localnet");
        const { value: balance } = await client.rpc.getBalance(client.wallet.address).send();
        console.log(`Balance: ${balance} lamports.`);
    let latestBlockhash =    (await client.rpc.getLatestBlockhash().send()).value
    
    const programAddress = address('9FxtmzhHF5zzm8ecoo3Jk5mCtTC2VWa56npHabAy88a1');
    
    
    // console.log(client.wallet.address)
    
         const programTx :Instruction[]=[
            {
                programAddress,
                accounts: [
                ],
                data:new Buffer([]),
                
            }
         ]
        const transactionMessage =  pipe(
            createTransactionMessage({ version: 0 }),
            (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
            (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
            (tx) => appendTransactionMessageInstructions(programTx, tx),
            // (tx) => client.estimateAndSetComputeUnitLimit(tx),
        );
        
        // console.log("trasection message", transactionMessage)
     
        // Compile the transaction message and sign it.
        const transaction = await signTransactionMessageWithSigners(transactionMessage);
        assertIsSendableTransaction(transaction);
         assertIsTransactionWithBlockhashLifetime(transaction);


         const serializedTransaction = getBase64EncodedWireTransaction(transaction);
         const similatio= await client.rpc.simulateTransaction(serializedTransaction,{encoding:"base64"}).send();
         if(similatio.value.err){
             return
            }
            console.log(similatio.value.logs)
    //  console.log("trasection", similatio)
        // Send the transaction and wait for confirmation.
        await client.sendAndConfirmTransaction(transaction, { commitment: 'confirmed' ,skipPreflight:true}); 
console.log("transaction : ", getSignatureFromTransaction(transaction))

        const { value: newBalance } = await client.rpc.getBalance(client.wallet.address).send();
        console.log(`New Balance: ${newBalance} lamports.`);
    }
    
    
    
    test();
