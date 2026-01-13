import * as anchor from "@coral-xyz/anchor";
import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    Transaction,
    sendAndConfirmTransaction,
} from "@solana/web3.js";
import { POOL_SEED, TICKET_MINT_SEED, TICKET_SEED } from "./constants";
import { BN } from "bn.js";

// fetching all the pda's associated with poolState and ticketMint PDA Seed
export const getHalopotPdas = (programId: PublicKey) => {
    const [poolState] = PublicKey.findProgramAddressSync(
        [Buffer.from(POOL_SEED)],
        programId
    );

    const [ticketMint] = PublicKey.findProgramAddressSync(
        [Buffer.from(TICKET_MINT_SEED)],
        programId
    );


    return { poolState, ticketMint };
};

export const getTicketPda = (programId: PublicKey, ticketId: number) => {
    return PublicKey.findProgramAddressSync(
        [Buffer.from(TICKET_SEED), new BN(ticketId).toArrayLike(Buffer, 'le', 8)],
        programId
    )[0];
};

export function lamportsToSol(lamports: number): number {
    return lamports / LAMPORTS_PER_SOL;
}

export function solToLamports(sol: number): any {
    return new BN(sol * LAMPORTS_PER_SOL);
}

export async function fundAccount(
    connection: anchor.web3.Connection,
    payer: anchor.web3.Keypair,
    toPubkey: PublicKey,
    amountInSol: number
) {
    const transaction = new Transaction().add(
        SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: toPubkey,
            lamports: amountInSol * LAMPORTS_PER_SOL,
        })
    );

    await sendAndConfirmTransaction(connection, transaction, [payer]);
    logDone(`Funded ${amountInSol} SOL to ${toPubkey.toBase58().slice(0, 6)}`);
}

export async function fetchBalance(
    provider: anchor.AnchorProvider,
    pubkey: PublicKey
): Promise<number> {
    const balance = await provider.connection.getBalance(pubkey);
    return balance / LAMPORTS_PER_SOL;
}

// injecting Mock Yield to simulate the yield
export const injectMockYield = async (
    provider: anchor.AnchorProvider,
    vaultPda: PublicKey,
    amountSol: number
) => {
    const tx = new Transaction().add(
        SystemProgram.transfer({
            fromPubkey: provider.wallet.publicKey,
            toPubkey: vaultPda,
            lamports: amountSol * LAMPORTS_PER_SOL,
        })
    );
    await provider.sendAndConfirm(tx);
    logDone(`Injected ${amountSol} SOL Mock Yield into Vault`);
};

export function logSignature(label: string, signature: string) {
    console.log(`\n ${label} Txn: https://explorer.solana.com/tx/${signature}?cluster=custom&customUrl=http://localhost:8899`);
}

export function logDone(label: String) {
    console.log(`   Compelted :  ${label}`);
}

export function logData(label: String) {
    console.log(`   Data :  ${label}`);
}