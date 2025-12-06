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
} from '@solana/kit';
 
export type Client = {
    rpc: Rpc<SolanaRpcApi>;
    rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
    sendAndConfirmTransaction: ReturnType<typeof sendAndConfirmTransactionFactory>; 
    wallet: TransactionSigner & MessageSigner; 
};
 
let client: Client | undefined;
export async function createClient(): Promise<Client> {
    if (!client) {
        // Create RPC objects and airdrop function.
        const rpc = createSolanaRpc('http://127.0.0.1:8899');
        const rpcSubscriptions = createSolanaRpcSubscriptions('ws://127.0.0.1:8900');
        const airdrop = airdropFactory({ rpc, rpcSubscriptions }); 
  const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
            rpc,
            rpcSubscriptions,
        });
        // Create a wallet with lamports.
        const wallet = await generateKeyPairSigner();
        await airdrop({
            recipientAddress: wallet.address,
            lamports: lamports(100_000_000n),
            commitment: 'confirmed',
        });
 
        // Store the client.
        client = { rpc, rpcSubscriptions,sendAndConfirmTransaction, wallet };
    }
    return client;
}