import { BN } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import * as pda from '../pda';
import * as whitelistingPda from '../../whitelisting/pda';

export const initBorrowAccount = async (program, payer, node, collateralCapacity) => {
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, payer.publicKey);
    const collateralsKP = Keypair.generate();
    await program.methods
        .initializeBorrowAccount(collateralCapacity)
        .accounts({
            signer: payer.publicKey,
            node, 
            borrowAccount,
            collaterals: collateralsKP.publicKey,
            systemProgram: SystemProgram.programId
        })
        .signers([collateralsKP])
        .rpc();
}

export const closeBorrowAccount = async (program, payer, node) => {
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, payer.publicKey);
    const { collaterals } = await program.account.borrowAccount.fetch(borrowAccount);
    await program.methods
        .closeBorrowAccount()
        .accounts({
            signer: payer.publicKey,
            node, 
            borrowAccount,
            collaterals,
            receiver: payer.publicKey,
        })
        .rpc();
}

export const depositCollateral = async (
    program, 
    whitelistingProgram,
    payer, 
    pool, 
    collection, 
    collateralMint,
    userTokenAccount
) => {
    const [admin, ] = await pda.findAdmin(program);
    const [node, ] = await pda.findNode(program, pool, collection);
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, payer.publicKey);
    const { collaterals } = await program.account.borrowAccount.fetch(borrowAccount);
    const [collateralAccount, ] = await pda.findCollateral(program, collection, collateralMint);
    const [whitelisted, ] = await whitelistingPda.findWNFT(whitelistingProgram, collection, collateralMint);
    const collateralTokenAccountKP = Keypair.generate();
    await program.methods
        .depositCollateral()
        .accounts({
            admin,
            node,
            collection,
            signer: payer.publicKey,
            borrowAccount,
            collaterals,
            collateralMint,
            whitelisted,
            collateralAccount,
            collateralTokenAccount: collateralTokenAccountKP.publicKey,
            userTokenAccount,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([collateralTokenAccountKP])
        .rpc();
}

export const withdrawCollateral = async (
    program, 
    payer, 
    pool, 
    collection, 
    collateralMint,
    userTokenAccount
) => {
    const { baseOracle, interestRateModel } = await program.account.lendingPool.fetch(pool);
    const [admin, ] = await pda.findAdmin(program);
    const [node, ] = await pda.findNode(program, pool, collection);
    const { collectionOracle } = await program.account.collectionAccount.fetch(collection);
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, payer.publicKey);
    const { collaterals } = await program.account.borrowAccount.fetch(borrowAccount);
    const [collateralAccount, ] = await pda.findCollateral(program, collection, collateralMint);
    const { collateralTokenAccount } = await program.account.collateralAccount.fetch(collateralAccount);
    await program.methods
        .withdrawCollateral()
        .accounts({
            admin,
            pool,
            interestRateModel,
            node,
            collection,
            borrower: payer.publicKey,
            borrowAccount,
            collaterals,
            collateralAccount,
            collateralTokenAccount,
            userTokenAccount,
            baseOracle,
            collectionOracle,
            receiver: payer.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .rpc();
}

export const borrow = async (
    program,
    payer,
    pool,
    collection,
    userTokenAccount,
    amount,
) => {
    const { baseOracle, treasury, interestRateModel } = await program.account.lendingPool.fetch(pool);
    const [admin, ] = await pda.findAdmin(program);
    const [node, ] = await pda.findNode(program, pool, collection);
    const { collectionOracle } = await program.account.collectionAccount.fetch(collection);
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, payer.publicKey);
    const { collaterals } = await program.account.borrowAccount.fetch(borrowAccount);
    await program.methods
        .borrow(amount)
        .accounts({
            admin,
            pool,
            interestRateModel,
            treasury,
            baseOracle,
            node,
            collection,
            collectionOracle,
            borrowAccount,
            collaterals,
            userTokenAccount,
            signer: payer.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .rpc();
}

export const repay = async (
    program,
    payer,
    pool,
    collection,
    userTokenAccount,
    amount,
) => {
    const { treasury, interestRateModel } = await program.account.lendingPool.fetch(pool);
    const [node, ] = await pda.findNode(program, pool, collection);
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, payer.publicKey);
    await program.methods
        .repay(amount)
        .accounts({
            pool,
            interestRateModel,
            treasury,
            node,
            borrowAccount,
            userTokenAccount,
            signer: payer.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .rpc();
}