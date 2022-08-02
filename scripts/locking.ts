import 'dotenv/config';
import * as anchorUtils from './utils';
import * as utils from '../tests/utils';
import * as lockingIxs from '../tests/locking/instructions';
import { AnchorProvider } from '@project-serum/anchor';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { PublicKey } from '@solana/web3.js';
import { getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import { Account } from "@metaplex-foundation/mpl-core";
import { Metadata } from "@metaplex-foundation/mpl-token-metadata";


const lockingProgram = anchorUtils.getLockingProgram("mainnet");
const { connection, wallet } = lockingProgram.provider as AnchorProvider;
const payer = (wallet as NodeWallet).payer;

const createMemberships = async () => {

    const goldPerks = lockingIxs.liquidationPerks(
        utils.bn(2_500), // 25% discount
        utils.bn(3), // 5 uses
        utils.bn(24 * 60 * 60), // 1 day warmup 
        utils.bn(7 * 24 * 60 * 60), // Refresh weekly
    );

    const silverPerks = lockingIxs.liquidationPerks(
        utils.bn(2_000), // 20% discount
        utils.bn(3), // 5 uses
        utils.bn(24 * 60 * 60), // 1 day warmup 
        utils.bn(7 * 24 * 60 * 60), // Refresh weekly
    );

    const bronzePerks = lockingIxs.liquidationPerks(
        utils.bn(1_500), // 15% discount
        utils.bn(3), // 5 uses
        utils.bn(24 * 60 * 60), // 1 day warmup 
        utils.bn(7 * 24 * 60 * 60), // Refresh weekly
    );

    console.log("GOLD");
    // await lockingIxs.initMembership(lockingProgram, payer, 0, goldPerks);
    await lockingIxs.updateMembership(lockingProgram, payer, 0, goldPerks)


    console.log("SIVLER");
    // await lockingIxs.initMembership(lockingProgram, payer, 1, silverPerks);
    await lockingIxs.updateMembership(lockingProgram, payer, 1, silverPerks)


    console.log("BRONZE");
    // await lockingIxs.initMembership(lockingProgram, payer, 2, bronzePerks);
    await lockingIxs.updateMembership(lockingProgram, payer, 2, bronzePerks)


}

const createCubeFake = async (membership, to) => {
    const mint = await utils.newMint(connection, payer, 0);
    console.log("CUBE", mint.toString())
    const ta = await getOrCreateAssociatedTokenAccount(connection, payer, mint, to);
    await utils.mint(connection, mint, payer, ta.address, 1, 0);
    await lockingIxs.initCube(lockingProgram, payer, mint, membership);
}

const getMetaplexMetadata = async (mint) => {
    const metadataPDA = await Metadata.getPDA(mint);
    const mintAccInfo = await connection.getAccountInfo(metadataPDA);
    const { data: { data: metadata } } = Metadata.from(new Account(mint, mintAccInfo));
    return metadata;
}

const getUriMetadata = async (url) => {
    return await fetch(url).then(res => res.json());
}

const MEMBERSHIP = {
    'bronze': 2,
    'silver': 1,
    'gold': 0
}

const createCube = async (mints) => {
    for (let mint of mints) {
        console.log(mint)
        const metaplexMD = await getMetaplexMetadata(mint);
        const uriMD = await getUriMetadata(metaplexMD.uri);
        const membership = MEMBERSHIP[uriMD.attributes[0].value];
        await lockingIxs.initCube(lockingProgram, payer, new PublicKey(mint), membership);
    }
}

const LOCAL = new PublicKey("8SSovXRiS532H9tjqkmrvECEZWVK4gTGs8kq6pH1nCX2");

// DEVNET
const CUBES = [

    // BRONZE
    "3zVdotM9L8yRdfQ3xFtTxCju3WxPq7xxX3VNKNUEufBg",
    "6oCXUBQwX1kKymcxceP3Gzy9mKTVrBdGvyeese6BZsDY",
    "DefmwSKS1VBCbBNZaqWEJjTgzBVXkJENoryV1SUu2STe",

    // SILVER
    "9VmusEvqe6SdQQnr5RQq1zQYUyzaWcPM61HbN9pxA6kr",
    "BNgrTVc2RAg8HwtzstnF5BSM2VNt6x7t1UJecXiP1EEp",
    "H64QG1ZSVXqKC6forfJCDhfNe75Ba8QkQB2uQgZcajHG",

    // GOLD
    "8RCKN9q7gD3kpdmGGV5xMPpzzUPZGYjBRR2i9PuRUqhU",
    "8ff3DjUnP2bRTSFA5L3gx6L1tqc63QArFnMTk66vqpNN",
    "GRKDv5WZ7HSNifFChtDoGEtQd8jfPXLXiFfzm9uf7cCm",

];

const main = async () => {

    // await lockingIxs.initAuthority(lockingProgram, payer);

    await createMemberships();

    // await createCube(CUBES)

}

main()
