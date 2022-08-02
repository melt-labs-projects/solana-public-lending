import assert from 'node:assert/strict';
import { PublicKey } from '@solana/web3.js';
import * as pda from './pda';


export const checkWhitelistedNFT = async (
    program, 
    collection: PublicKey,
    mint: PublicKey
) => {
    const [wnft, ] = await pda.findWNFT(program, collection, mint);
    const whitelistData = await program.account.whitelisted.fetch(wnft);
    assert(whitelistData.collection.equals(collection));
    assert(whitelistData.mint.equals(mint));
}