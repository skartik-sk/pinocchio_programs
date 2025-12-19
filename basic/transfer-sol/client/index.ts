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
    console.log(client.wallet.address)
        const { value: balance } = await client.rpc.getBalance(client.wallet.address).send();
        console.log(`Balance: ${balance} lamports.`);
        
    let latestBlockhash =    (await client.rpc.getLatestBlockhash().send()).value
    
    const programAddress = address('Atqt8U2XJL6MqAJ2gRMmwVVaKkMNY9EuCARwcN3PHsnZ');
    let reciverAcc= await generateKeyPairSigner()
    console.log(reciverAcc.address)
    const { value: balance1 } = await client.rpc.getBalance(reciverAcc.address).send();
    console.log(`Balance: ${balance1} lamports.`);
         const programTx :Instruction[]=[
            {
                programAddress,
                accounts: [
                   {
                     address:client.wallet.address,
                     role:AccountRole.WRITABLE_SIGNER
                     },
                   {
                     address:reciverAcc.address,
                     role:AccountRole.WRITABLE//WRITABLE_SIGNER
                   }
                ],
                data:getInstructionData(1_00n)
                
            }
         ]
         // console.log("program tx" , programTx)
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
     // console.log("trasection", transaction)
        // Send the transaction and wait for confirmation.
        await client.sendAndConfirmTransaction(transaction, { commitment: 'confirmed' }); 
        const { value: newBalance } = await client.rpc.getBalance(client.wallet.address).send();
        console.log(`New Balance: ${newBalance} lamports.`);
        
        const { value: newBalance2 } = await client.rpc.getBalance(reciverAcc.address).send();
        console.log(`New Balance user: ${newBalance2} lamports.`);
    }
    
    
    
    test();
