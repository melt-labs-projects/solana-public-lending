import { BN } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import * as pda from '../pda';


export const initAuthority = async (program, payer) => {
    const [admin, ] = await pda.findAdmin(program);
    await program.methods
        .initializeAuthority()
        .accounts({
            admin,
            authority: payer.publicKey,
            systemProgram: SystemProgram.programId
        })
        .rpc();
}

export const updateAuthority = async (program, payer, newAuthority) => {
    const [admin, ] = await pda.findAdmin(program);
    await program.methods
        .updateAuthority(newAuthority)
        .accounts({
            admin,
            authority: payer.publicKey
        })
        .rpc();
}

export const initOracle = async (program, payer, decimals, price) => {
    const [admin, ] = await pda.findAdmin(program);
    const oracleKP = Keypair.generate();
    await program.methods
        .initializeOracle(decimals, price)
        .accounts({
            admin,
            authority: payer.publicKey,
            oracle: oracleKP.publicKey,
            systemProgram: SystemProgram.programId
        })
        .signers([oracleKP])
        .rpc();
    return oracleKP.publicKey;
}

export const updateOracle = async (program, payer, oracle, price) => {
    const [admin, ] = await pda.findAdmin(program);
    await program.methods
        .updateOracle(price)
        .accounts({
            admin,
            authority: payer.publicKey,
            oracle,
        })
        .rpc();
}

export const initCollection = async (program, payer, collectionOracle, maxCollaterals) => {
    const [admin, ] = await pda.findAdmin(program);
    const collectionKP = Keypair.generate();
    await program.methods
        .initializeCollection(maxCollaterals)
        .accounts({
            admin,
            authority: payer.publicKey,
            collection: collectionKP.publicKey, 
            collectionOracle,
            systemProgram: SystemProgram.programId
        })
        .signers([collectionKP])
        .rpc();
    return collectionKP.publicKey;
}

export const updateCollection = async (program, payer, collection, collectionOracle, maxCollaterals) => {
    const [admin, ] = await pda.findAdmin(program);
    await program.methods
        .updateCollection(maxCollaterals)
        .accounts({
            admin,
            authority: payer.publicKey,
            collection,
            collectionOracle
        })
        .rpc();
}

export const initCollateral = async (program, payer, collection, mint) => {
    const [admin, ] = await pda.findAdmin(program);
    const [collateral, ] = await pda.findCollateral(program, collection, mint);
    await program.methods
        .initializeCollateral()
        .accounts({
            admin,
            authority: payer.publicKey,
            collection, 
            mint,
            collateral,
            systemProgram: SystemProgram.programId
        })
        .rpc();
    return collateral;
}

export const initNSlopeModel = async (program, payer, slopes) => {
    const [admin, ] = await pda.findAdmin(program);
    const modelKP = Keypair.generate();
    await program.methods
        .initializeNSlopeModel(slopes, new BN(slopes.length))
        .accounts({
            admin,
            authority: payer.publicKey,
            model: modelKP.publicKey,
            systemProgram: SystemProgram.programId
        })
        .signers([modelKP])
        .rpc();
    return modelKP.publicKey;
}

export const initPool = async (program, payer, nSlopeModel, baseMint, baseOracle, facilitatorFee) => {
    const [admin, ] = await pda.findAdmin(program);
    const poolKP = Keypair.generate();
    const creditMintKP = Keypair.generate();
    const treasuryKP = Keypair.generate();
    const feeTokenAccountKP = Keypair.generate();
    const reserveKP = Keypair.generate();
    await program.methods
        .initializeLendingPool({ facilitatorFee })
        .accounts({
            admin,
            authority: payer.publicKey,
            pool: poolKP.publicKey,
            nSlopeModel,
            baseOracle,
            baseMint,
            treasury: treasuryKP.publicKey,
            reserve: reserveKP.publicKey,
            feeTokenAccount: feeTokenAccountKP.publicKey,
            creditMint: creditMintKP.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY
        })
        .signers([poolKP, creditMintKP, treasuryKP, reserveKP, feeTokenAccountKP])
        .rpc();
    return poolKP.publicKey;
}

export const updatePool = async (program, payer, pool, baseOracle, nSlopeModel, facilitatorFee) => {
    const [admin, ] = await pda.findAdmin(program);
    await program.methods
        .updateLendingPool({ facilitatorFee })
        .accounts({
            admin,
            authority: payer.publicKey,
            pool,
            nSlopeModel,
            baseOracle,
        })
        .rpc();
}

export const nodeParams = (enabled, maxBorrowRatio, liquidationRatio) => {
    return { enabled, maxBorrowRatio, liquidationRatio }
}

export const initNode = async (program, payer, pool, collection, nodeParams) => {
    const [admin, ] = await pda.findAdmin(program);
    const [node, ] = await pda.findNode(program, pool, collection);
    await program.methods
        .initializeBorrowingNode(nodeParams)
        .accounts({
            admin,
            authority: payer.publicKey,
            node,
            pool,
            collection,
            systemProgram: SystemProgram.programId,
        })
        .rpc();
    return node;
}

export const updateNode = async (program, payer, pool, collection, nodeParams) => {
    const [admin, ] = await pda.findAdmin(program);
    const [node, ] = await pda.findNode(program, pool, collection);
    await program.methods
        .updateBorrowingNode(nodeParams)
        .accounts({
            admin,
            authority: payer.publicKey,
            node
        })
        .rpc();
}

export const initSlice = async (program, payer, collection, sliceNumber) => {
    const [admin, ] = await pda.findAdmin(program);
    const [slice, ] = await pda.findSlice(program, collection, sliceNumber);
    await program.methods
        .initializeSlice(new BN(sliceNumber))
        .accounts({
            admin,
            authority: payer.publicKey,
            collection,
            slice,
            systemProgram: SystemProgram.programId
        })
        .rpc();
}