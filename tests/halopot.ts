import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Halopot } from "../target/types/halopot";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

import {
    getHalopotPdas,
    getTicketPda,
    fundAccount,
    fetchBalance,
    injectMockYield,
    logDone,
    logData,
    logSignature,
    solToLamports,
    lamportsToSol
} from "./utils";

import {
    TICKET_PRICE_SOL,
    WRONG_PRICE_SOL,
    MOCK_YIELD_SOL,
    ERR_INCORRECT_PRICE
} from "./constants";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("halopot protocol", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Halopot as Program<Halopot>;

    // test users
    const admin = Keypair.generate();
    const user1 = Keypair.generate();
    const user2 = Keypair.generate();

    //  PDAs & addresses
    let poolState: PublicKey;
    let ticketMint: PublicKey;
    let user1TicketAccount: PublicKey;
    let user2TicketAccount: PublicKey;

    before(async () => {
        console.log("---------------------------------------------------------");
        console.log("STARTING TEST");

        // funding acccounts
        const payer = provider.wallet.payer; // by default there are sol in provider wallet
        await fundAccount(provider.connection, payer, admin.publicKey, 5);
        await fundAccount(provider.connection, payer, user1.publicKey, 5);
        await fundAccount(provider.connection, payer, user2.publicKey, 5);

        // global PDAs
        const pdas = getHalopotPdas(program.programId);
        poolState = pdas.poolState;
        ticketMint = pdas.ticketMint;

        // associated ticket accounts
        user1TicketAccount = getAssociatedTokenAddressSync(ticketMint, user1.publicKey);
        user2TicketAccount = getAssociatedTokenAddressSync(ticketMint, user2.publicKey);

        console.log("---------------------------------------------------------");
    });

    /* Pool State Initialization */
    describe("initialization", () => {
        it("should initialize the protocol", async () => {
            const initAccounts = {
                admin: admin.publicKey,
                poolState,
                ticketMint,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            };

            const tx = await program.methods.initialize().accounts(initAccounts).signers([admin]).rpc();

            logSignature("Initialize", tx);

            // verify pool state
            const state = await program.account.poolState.fetch(poolState);
            expect(state.roundId.toNumber()).to.equal(1);
            expect(state.totalTickets.toNumber()).to.equal(0);
            expect(state.admin.toString()).to.equal(admin.publicKey.toString());

            logDone("Protocol Initialized successfully");
        });
    });

    // what should the user1 do to deposit sol to pool state ? 
    // user1 send sol -> pool pda state 

    // /* Deposit Funcionality */
    describe("deposits", () => {
        it("should allow User 1 to deposit 1 SOL", async () => {
            const ticketId = (await program.account.poolState.fetch(poolState)).ticketCount.toNumber();
            const ticketPda = getTicketPda(program.programId, ticketId);

            const balanceBefore = await fetchBalance(provider, user1.publicKey);

            const deposit1Accounts = {
                user: user1.publicKey,
                poolState: poolState,
                ticketMint: ticketMint,
                ticket: ticketPda,
                userTicketAta: user1TicketAccount,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            };

            const tx = await program.methods.deposit(solToLamports(TICKET_PRICE_SOL)).accounts(deposit1Accounts).signers([user1]).rpc();

            logSignature("Deposit User 1", tx);

            // verify
            const balanceAfter = await fetchBalance(provider, user1.publicKey);
            const state = await program.account.poolState.fetch(poolState);

            console.log("balance : ", balanceBefore - balanceAfter);

            expect(state.ticketCount.toNumber()).to.equal(ticketId + 1);
            expect(balanceBefore - balanceAfter).to.be.greaterThan(TICKET_PRICE_SOL); // price + Gas

            logDone("User 1 Deposited 1 SOL and got Ticket #0");
        });

        it("should fail when depositing incorrect amount (0.5 SOL)", async () => {
            const ticketId = (await program.account.poolState.fetch(poolState)).ticketCount.toNumber();
            const ticketPda = getTicketPda(program.programId, ticketId);

            try {
                const depositWrongAccounts = {
                    user: user1.publicKey,
                    poolState: poolState,
                    ticketMint: ticketMint,
                    ticket: ticketPda,
                    userTicketAta: user1TicketAccount,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                };

                await program.methods.deposit(solToLamports(WRONG_PRICE_SOL)).accounts(depositWrongAccounts).signers([user1]).rpc();

                expect.fail("Transaction should have failed");
            } catch (err) {
                // Assert error message
                expect(err.message).to.include(ERR_INCORRECT_PRICE);
                logDone("Correctly rejected 0.5 SOL deposit");
            }
        });

        it("should allow User 2 to deposit 1 SOL (Ticket #1)", async () => {
            const ticketId = (await program.account.poolState.fetch(poolState)).ticketCount.toNumber();
            const ticketPda = getTicketPda(program.programId, ticketId);

            const deposit2Accounts = {
                user: user2.publicKey,
                poolState: poolState,
                ticketMint: ticketMint,
                ticket: ticketPda,
                userTicketAta: user2TicketAccount,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            };

            const tx = await program.methods.deposit(solToLamports(TICKET_PRICE_SOL)).accounts(deposit2Accounts).signers([user2]).rpc();

            logSignature("Deposit User 2", tx);

            const state = await program.account.poolState.fetch(poolState);
            console.log("pool state adming : ", state.admin.toBase58());
            expect(state.ticketCount.toNumber()).to.equal((await program.account.poolState.fetch(poolState)).ticketCount.toNumber()); // Should be 2 tickets now

            logDone("User 2 Deposited 1 SOL and got Ticket #1");
        });
    });

    // /* Yeild and Winnerr */
    describe("Yield Simulation", () => {
        it("should simulate yield generation", async () => {
            await injectMockYield(provider, poolState, MOCK_YIELD_SOL);

            const vaultBalance = await fetchBalance(provider, poolState);
            const state = await program.account.poolState.fetch(poolState);
            const principal = lamportsToSol(state.totalPrincipal.toNumber());

            logData(`Vault Balance: ${vaultBalance} SOL`);
            logData(`Total Principal: ${principal} SOL`);

            expect(vaultBalance).to.be.greaterThan(principal);
            logDone("Yield Injected successfully");
        });

        it("should pick a winner on-chain", async () => {
            console.log("admin public key : ", admin.publicKey.toBase58());

            const pickWinnerAccounts = {
                admin: admin.publicKey,
                poolState: poolState,
                // winner: Keypair.,
            };

            const tx = await program.methods.pickWinner().accounts(pickWinnerAccounts).signers([admin]).rpc();

            logSignature("Pick Winner", tx);

            const state = await program.account.poolState.fetch(poolState);
            expect(state.winningId).to.not.be.null;

            logData(`Winning Ticket ID: ${state.winningId}`);
            logDone("Winner selected on-chain");
        });

        it("should allow winner to claim prize", async () => {
            // identifying the winner
            const state = await program.account.poolState.fetch(poolState);
            const winningId = state.winningId.toNumber();

            // determining which user won based on Ticket ID
            // Ticket 0 = User 1, Ticket 1 = User 2
            let winnerKeypair = winningId === 0 ? user1 : user2;
            let ticketPda = getTicketPda(program.programId, winningId);

            console.log(`   and the winner is .... : ${winningId === 0 ? "User 1" : "User 2"}`);

            const balanceBefore = await fetchBalance(provider, winnerKeypair.publicKey);

            const claimAccounts = {
                user: winnerKeypair.publicKey,
                poolState: poolState,
                ticket: ticketPda,
                winner: winnerKeypair.publicKey,
                systemProgram: SystemProgram.programId,
            };

            const tx = await program.methods.claimPrize().accounts(claimAccounts).signers([winnerKeypair]).rpc();

            logSignature("Claim Prize", tx);

            const balanceAfter = await fetchBalance(provider, winnerKeypair.publicKey);
            const profit = balanceAfter - balanceBefore;

            logData(`Profit Made: ~${profit.toFixed(2)} SOL`);

            // Profit should be close to Mock Yield (2 SOL) minus gas fees
            expect(profit).to.be.greaterThan(1.0);

            // verify Reset
            const stateAfter = await program.account.poolState.fetch(poolState);
            expect(stateAfter.winningId).to.be.null;

            logDone("Prize claimed and round reset");
        });
    });
});