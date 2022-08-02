import { PublicKey } from "@solana/web3.js";

export const findAdmin = (program) => {
    return PublicKey.findProgramAddress(
        [Buffer.from("admin")],
        program.programId
    );
}

export const findCollateral = (program, collection, mint) => {
    return PublicKey.findProgramAddress(
        [Buffer.from("collateral"), collection.toBuffer(), mint.toBuffer()],
        program.programId
    );
}

export const findNode = (program, pool, collection) => {
    return PublicKey.findProgramAddress(
        [Buffer.from("node"), pool.toBuffer(), collection.toBuffer()],
        program.programId
    );
}

export const findBorrowAccount = (program, node, borrower) => {
    return PublicKey.findProgramAddress(
        [Buffer.from("borrow"), node.toBuffer(), borrower.toBuffer()],
        program.programId
    );
}

export const findSlice = (program, collection, number) => {
    let numBuffer = Buffer.alloc(8);
	numBuffer.writeBigUInt64LE(BigInt(number), 0);
    return PublicKey.findProgramAddress(
        [Buffer.from("slice"), collection.toBuffer(), numBuffer],
        program.programId
    );
}