import { SystemProgram } from "@solana/web3.js";
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

export const whitelist = async (program, payer, collection, mint) => {
    const [admin, ] = await pda.findAdmin(program);
    const [whitelisted, ] = await pda.findWNFT(program, collection, mint);
    await program.methods
        .whitelist()
        .accounts({
            admin,
            collection,
            mint,
            whitelisted,
            authority: payer.publicKey,
            systemProgram: SystemProgram.programId
        })
        .rpc();
}