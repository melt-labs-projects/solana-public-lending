import { PublicKey } from "@solana/web3.js";

export const findAdmin = (program) => {
    return PublicKey.findProgramAddress(
        [Buffer.from("admin")],
        program.programId
    );
}

export const findWNFT = (program, collection, mint) => {
    return PublicKey.findProgramAddress(
        [Buffer.from("wnft"), collection.toBuffer(), mint.toBuffer()],
        program.programId
    );
}