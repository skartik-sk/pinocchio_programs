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
    createKeyPairSignerFromBytes,
} from '@solana/kit';
import * as fs from 'fs';
 
export type Client = {
    rpc: Rpc<SolanaRpcApi>;
    rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
    sendAndConfirmTransaction: ReturnType<typeof sendAndConfirmTransactionFactory>; 
    wallet: TransactionSigner & MessageSigner; 
};
 
let client: Client | undefined;
export async function createClient(type: string): Promise<Client> {
    if (!client) {
      
        // Create RPC objects and airdrop function.
        const rpc = createSolanaRpc(type=="devnet"?"https://api.devnet.solana.com":"http://127.0.0.1:8899");
        const rpcSubscriptions = createSolanaRpcSubscriptions(type=="devnet"?"wss://api.devnet.solana.com":"ws://127.0.0.1:8900");
        const airdrop = airdropFactory({ rpc, rpcSubscriptions }); 
  const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
            rpc,
            rpcSubscriptions,
        });
        // Create a wallet with lamports.
        const wallet = await createKeyPairSignerFromBytes(new Uint8Array(JSON.parse(fs.readFileSync('/Users/singupallikartik/.config/solana/id.json', 'utf-8'))));
        // await airdrop({
        //     recipientAddress: wallet.address,
        //     lamports: lamports(100_000_000n),
        //     commitment: 'confirmed',
        // });
 
        // Store the client.
        client = { rpc, rpcSubscriptions,sendAndConfirmTransaction, wallet };
    }
    return client;
}