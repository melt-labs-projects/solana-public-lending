import { PublicKey } from "@solana/web3.js";

export const findAdmin = (program) => {
    return PublicKey.findProgramAddress(
        [Buffer.from("admin")],
        program.programId
    );
}

export const findCube = (program, mint) => {
    return PublicKey.findProgramAddress(
        [Buffer.from("cube"), mint.toBuffer()],
        program.programId
    );
}

export const findMembership = (program, number) => {
    let numBuffer = Buffer.alloc(8);
	numBuffer.writeBigUInt64LE(BigInt(number), 0);
    return PublicKey.findProgramAddress(
        [Buffer.from("membership"), numBuffer],
        program.programId
    );
}

export const findLockingAccount = (program, locker) => {
    return PublicKey.findProgramAddress(
        [Buffer.from("locked"), locker.toBuffer()],
        program.programId
    );
}
