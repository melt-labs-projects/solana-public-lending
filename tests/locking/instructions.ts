import { BN } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import * as pda from './pda';

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

export const liquidationPerks = (discount, maxUses, warmupPeriod, refreshPeriod) => {
    return { discount, maxUses, warmupPeriod, refreshPeriod };
}

export const initMembership = async (program, payer, membershipNumber, perks) => {
    const [admin, ] = await pda.findAdmin(program);
    const [membership, ] = await pda.findMembership(program, membershipNumber);
    await program.methods
        .initializeMembership(new BN(membershipNumber), perks)
        .accounts({
            admin,
            membership,
            authority: payer.publicKey,
            systemProgram: SystemProgram.programId
        })
        .rpc();
}

export const updateMembership = async (program, payer, membershipNumber, perks) => {
    const [admin, ] = await pda.findAdmin(program);
    const [membership, ] = await pda.findMembership(program, membershipNumber);
    await program.methods
        .updateMembership(perks)
        .accounts({
            admin,
            membership,
            authority: payer.publicKey,
        })
        .rpc();
}

export const initCube = async (program, payer, cubeMint, membershipNumber) => {
    const [admin, ] = await pda.findAdmin(program);
    const [cube, ] = await pda.findCube(program, cubeMint);
    const [membership, ] = await pda.findMembership(program, membershipNumber);
    await program.methods
        .initializeCube()
        .accounts({
            admin,
            cubeMint,
            cube,
            membership,
            authority: payer.publicKey,
            systemProgram: SystemProgram.programId
        })
        .rpc();
}

export const initLockingAccount = async (program, payer) => {
    const [lockingAccount, ] = await pda.findLockingAccount(program, payer.publicKey);
    await program.methods
        .initializeLockingAccount()
        .accounts({
            lockingAccount,
            signer: payer.publicKey,
            systemProgram: SystemProgram.programId,
        })
        .rpc();
}

export const closeLockingAccount = async (program, payer) => {
    const [lockingAccount, ] = await pda.findLockingAccount(program, payer.publicKey);
    await program.methods
        .closeLockingAccount()
        .accounts({
            lockingAccount,
            signer: payer.publicKey,
            receiver: payer.publicKey,
        })
        .rpc();
}

export const lock = async (program, payer, cubeMint, fromTokenAccount) => {
    const [lockingAccount, ] = await pda.findLockingAccount(program, payer.publicKey);
    const [cube, ] = await pda.findCube(program, cubeMint);
    const cubeData = await program.account.cubeAccount.fetch(cube);
    const membership = cubeData.membership;
    const lockedTokenAccountKP = Keypair.generate();
    await program.methods
        .lock()
        .accounts({
            lockingAccount,
            cube,
            cubeMint,
            membership,
            fromTokenAccount,
            lockedTokenAccount: lockedTokenAccountKP.publicKey,
            locker: payer.publicKey,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([lockedTokenAccountKP])
        .rpc();
}

export const unlock = async (program, payer, toTokenAccount) => {
    const [lockingAccount, ] = await pda.findLockingAccount(program, payer.publicKey);
    const lockingData = await program.account.lockingAccount.fetch(lockingAccount);
    const cubeMint = lockingData.cubeMint;
    const [cube, ] = await pda.findCube(program, cubeMint);
    const membership = lockingData.membership;
    await program.methods
        .unlock()
        .accounts({
            lockingAccount,
            cube,
            cubeMint,
            membership,
            toTokenAccount,
            lockedTokenAccount: lockingData.lockedTokenAccount,
            locker: payer.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
}

export const liquidationUpdate = async (program, payer) => {
    const [lockingAccount, ] = await pda.findLockingAccount(program, payer.publicKey);
    const lockingData = await program.account.lockingAccount.fetch(lockingAccount);
    const membership = lockingData.membership;
    await program.methods
        .liquidationUpdate()
        .accounts({
            lockingAccount,
            membership,
        })
        .rpc();
}

export const liquidationLog = async (program, payer) => {
    const [lockingAccount, ] = await pda.findLockingAccount(program, payer.publicKey);
    await program.methods
        .liquidationLog()
        .accounts({
            lockingAccount,
            locker: payer.publicKey,
        })
        .rpc();
}