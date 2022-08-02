import * as anchor from '@project-serum/anchor';
import { createMint, mintTo } from "@solana/spl-token";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { Program } from "@project-serum/anchor";
import { Locking } from "../target/types/locking";
import { Lending } from "../target/types/lending";
import { Whitelisting } from "../target/types/whitelisting";

export const bn = (value, decimals=0) => {
    return new anchor.BN(Math.round(value * 10 ** decimals).toString());
}

export const getLendingProgram = () => {
    return anchor.workspace.Lending as Program<Lending>;
}

export const getLockingProgram = () => {
    return anchor.workspace.Locking as Program<Locking>;
}

export const getWhitelistingProgram = () => {
    return anchor.workspace.Whitelisting as Program<Whitelisting>;
}

export const getConnection = () => {
    const provider = anchor.AnchorProvider.env();
    const connection = provider.connection;
    const payer = (provider.wallet as NodeWallet).payer;
    anchor.setProvider(provider);
    return { provider, connection, payer }
}

export const newMint = (
    connection, 
    payer,
    decimals?
) => {
    return createMint(
        connection, 
        payer, 
        payer.publicKey, 
        payer.publicKey, 
        decimals ?? 9 
    );
}

export const mint = (
    connection, 
    mint, 
    payer, 
    to,
    amount,
    decimals?
) => {
    return mintTo(
        connection,
        payer,
        mint,
        to,
        payer.publicKey,
        amount * 10 ** (decimals ?? 9),
        []
    );
}