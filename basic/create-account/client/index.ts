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
        generateKeyPairSigner,
        addSignersToTransactionMessage,
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
    
    const programAddress = address('5ARYz7UUdw9dsjyYnMFw6guJFqF5WsBNKm7fmc9JkSCm');
    const newAcc =   await generateKeyPairSigner();
    
    console.log(client.wallet.address)
    console.log(newAcc.address)
    
         const programTx :Instruction[]=[
            {
                programAddress,
                accounts: [
                    {
                        address: client.wallet.address,
                        role:AccountRole.WRITABLE_SIGNER,
                    },
                    {
                        address: newAcc.address,
                        role:AccountRole.WRITABLE_SIGNER,
                    },
                    {
                        address: address("11111111111111111111111111111111"),
                        role:AccountRole.READONLY,
                    }


                ],
                data: new Buffer([]),
                
            }
         ]
         console.log("program tx" , programTx)
        const transactionMessage =  pipe(
            createTransactionMessage({ version: 0 }),
            (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
            (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
            (tx) => appendTransactionMessageInstructions(programTx, tx),
        );
        
       const newTrasectionMessage=  addSignersToTransactionMessage([newAcc], transactionMessage);
        console.log("trasection message", transactionMessage)
     
        // Compile the transaction message and sign it.
        const transaction = await signTransactionMessageWithSigners(newTrasectionMessage);
        assertIsSendableTransaction(transaction);
         assertIsTransactionWithBlockhashLifetime(transaction);
     console.log("trasection", transaction)
        // Send the transaction and wait for confirmation.
        await client.sendAndConfirmTransaction(transaction, { commitment: 'confirmed' }); 
        const { value: newBalance } = await client.rpc.getBalance(client.wallet.address).send();
        console.log(` Balance: difference ${newBalance-balance} lamports.`);
    }
    
    
    
    test();
