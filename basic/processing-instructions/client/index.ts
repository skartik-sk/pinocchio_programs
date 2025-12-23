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
    } from '@solana/kit';
    import { createClient } from '../../clients';
    interface Intrctiondata{
      name:string,
      height:number, 
    }
    function getInstructionData() {
        const buffer = new ArrayBuffer(44); // 1 byte (discriminator) + 8 bytes (amount)
        const view = new DataView(buffer);
        let data:Intrctiondata={
          name:"kartik",
          height:6,
        }
        const encoder = new TextEncoder();
          const encodedName = encoder.encode(data.name);
          const nameBuffer = new Uint8Array(buffer, 0, 40);
          nameBuffer.set(encodedName.slice(0, 40));
        view.setUint32(40, data.height, true); // The Amount (Little Endian)
        const idata = new Uint8Array(buffer);
      return idata;
    }
    async function test() {
      const client = await createClient("localnet");
        const { value: balance } = await client.rpc.getBalance(client.wallet.address).send();
        console.log(`Balance: ${balance} lamports.`);
    let latestBlockhash =    (await client.rpc.getLatestBlockhash().send()).value
    
    const programAddress = address('3V1KMm8SVaMKAZURDkjNAiUsNL7o7ebMssK96TkxjJFf');
    
    
    console.log(client.wallet.address)
    
         const programTx :Instruction[]=[
            {
                programAddress,
                accounts: [
                   
                ],
                data:getInstructionData()
                
            }
         ]
         console.log("program tx" , programTx)
        const transactionMessage =  pipe(
            createTransactionMessage({ version: 0 }),
            (tx) => setTransactionMessageFeePayerSigner(client.wallet, tx),
            (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
            (tx) => appendTransactionMessageInstructions(programTx, tx),
            // (tx) => client.estimateAndSetComputeUnitLimit(tx),
        );
        
        console.log("trasection message", transactionMessage)
     
        // Compile the transaction message and sign it.
        const transaction = await signTransactionMessageWithSigners(transactionMessage);
        assertIsSendableTransaction(transaction);
         assertIsTransactionWithBlockhashLifetime(transaction);
     console.log("trasection", transaction)
        // Send the transaction and wait for confirmation.
        await client.sendAndConfirmTransaction(transaction, { commitment: 'confirmed' }); 
        const { value: newBalance } = await client.rpc.getBalance(client.wallet.address).send();
        console.log(`New Balance: ${newBalance} lamports.`);
    }
    
    
    
    test();
