import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as pda from '../pda';

export const depositTokens = async (
    program, 
    payer, 
    pool, 
    baseTokenAccount,
    creditTokenAccount,
    amount,
) => {
    const [admin, ] = await pda.findAdmin(program);
    const { treasury, creditMint, interestRateModel } = await program.account.lendingPool.fetch(pool);
    await program.methods
        .depositTokens(amount)
        .accounts({
            admin,
            pool,
            interestRateModel,
            signer: payer.publicKey,
            treasury,
            baseTokenAccount,
            creditTokenAccount,
            creditMint,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .rpc();
}

export const withdrawTokens = async (
    program, 
    payer, 
    pool, 
    baseTokenAccount,
    creditTokenAccount,
    amount,
) => {
    const [admin, ] = await pda.findAdmin(program);
    const { treasury, creditMint, interestRateModel } = await program.account.lendingPool.fetch(pool);
    await program.methods
        .withdrawTokens(amount)
        .accounts({
            admin,
            pool,
            interestRateModel,
            signer: payer.publicKey,
            treasury,
            baseTokenAccount,
            creditTokenAccount,
            creditMint,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .rpc();
}