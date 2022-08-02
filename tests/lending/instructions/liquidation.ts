import { BN } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import * as pda from '../pda';
import * as lockingPda from '../../locking/pda';

const SLICE_SIZE = 10;

export const liquidate = async (
    program, 
    payer, 
    pool, 
    collection,
    collateralMint
) => {
    const { baseOracle, interestRateModel } = await program.account.lendingPool.fetch(pool);
    const { collectionOracle, totalSlicesLength } = await program.account.collectionAccount.fetch(collection);
    const [collateralAccount, ] = await pda.findCollateral(program, collection, collateralMint);
    const { borrower } = await program.account.collateralAccount.fetch(collateralAccount);
    const lastSliceNumber = Math.floor(totalSlicesLength.toNumber() / SLICE_SIZE);
    const [node, ] = await pda.findNode(program, pool, collection);
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, borrower);
    const { collaterals } = await program.account.borrowAccount.fetch(borrowAccount);
    const [lastSlice, ] = await pda.findSlice(program, collection, lastSliceNumber);
    await program.methods
        .liquidate()
        .accounts({
            pool,
            interestRateModel,
            baseOracle,
            node,
            collection,
            collectionOracle,
            borrowAccount,
            collaterals,
            collateralAccount,
            signer: payer.publicKey,
            receiver: payer.publicKey,
            lastSlice,
        })
        .rpc();
}

export const purchase = async (
    program,
    lockingProgram,
    payer, 
    pool, 
    collection,
    collateralMint,
    paymentTokenAccount,
    toTokenAccount,
) => {
    const [admin, ] = await pda.findAdmin(program);
    const { baseOracle, treasury, reserve } = await program.account.lendingPool.fetch(pool);
    const { collectionOracle, totalSlicesLength } = await program.account.collectionAccount.fetch(collection);
    
    const [node, ] = await pda.findNode(program, pool, collection);
    const [collateralAccount, ] = await pda.findCollateral(program, collection, collateralMint);
    const { collaterals, collateralTokenAccount } = await program.account.collateralAccount.fetch(collateralAccount);

    const lastSliceNumber = Math.floor(totalSlicesLength.toNumber() / SLICE_SIZE);
    const [lastSlice, ] = await pda.findSlice(program, collection, lastSliceNumber);

    const [lockingAccount, ] = await lockingPda.findLockingAccount(lockingProgram, payer.publicKey);
    const { membership } = await lockingProgram.account.lockingAccount.fetch(lockingAccount);

    await program.methods
        .purchase()
        .accounts({
            admin,
            pool,
            treasury,
            reserve,
            baseOracle,
            node,
            collection,
            collectionOracle,
            collaterals,
            collateralAccount,
            signer: payer.publicKey,
            receiver: payer.publicKey,
            paymentTokenAccount,
            collateralTokenAccount,
            toTokenAccount,
            lockingProgram: lockingProgram.programId,
            lockingAccount,
            membershipAccount: membership,
            tokenProgram: TOKEN_PROGRAM_ID,
            aSlice: lastSlice,
            lastSlice,
        })
        .rpc();
}