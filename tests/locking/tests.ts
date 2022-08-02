import * as anchor from "@project-serum/anchor";
import * as instructions from './instructions';
import * as utils from '../utils';

import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

describe("locking", () => {

    // Configure the client to use the local cluster.
    const { provider, connection, payer } = utils.getConnection();
    const program = utils.getLockingProgram();

    var bronzeMint;
    var silverMint;
    var goldMint;

    const goldPerks = instructions.liquidationPerks(
        utils.bn(200), // 20% discount
        utils.bn(5), // 5 uses
        utils.bn(60 * 60 * 24), // 1 day warmup 
        utils.bn(60 * 60 * 24 * 7), // Refresh weekly
    )

    const silverPerks = instructions.liquidationPerks(
        utils.bn(150), // 15% discount
        utils.bn(5), // 5 uses
        utils.bn(60 * 60 * 24), // 1 day warmup 
        utils.bn(60 * 60 * 24 * 7), // Refresh weekly
    )

    const bronzePerks = instructions.liquidationPerks(
        utils.bn(100), // 10% discount
        utils.bn(5), // 5 uses
        utils.bn(60 * 60 * 24), // 1 day warmup 
        utils.bn(60 * 60 * 24 * 7), // Refresh weekly
    )

    it("init authority", async () => {
        await instructions.initAuthority(program, payer);
    });

    it("update authority", async () => {
        await instructions.updateAuthority(program, payer, payer.publicKey);
    });

    it("init memberships", async () => {
        await instructions.initMembership(program, payer, 0, bronzePerks);
        await instructions.initMembership(program, payer, 1, silverPerks);
        await instructions.initMembership(program, payer, 2, goldPerks);
    });

    it("init cubes", async () => {
        bronzeMint = await utils.newMint(connection, payer, 0);
        silverMint = await utils.newMint(connection, payer, 0);
        goldMint = await utils.newMint(connection, payer, 0);

        await instructions.initCube(program, payer, bronzeMint, 0);
        await instructions.initCube(program, payer, silverMint, 1);
        await instructions.initCube(program, payer, goldMint, 2);
    });

    it('init locking account', async () => {
        await instructions.initLockingAccount(program, payer);
    });

    it('lock', async () => {
        const bronzeTA = await getOrCreateAssociatedTokenAccount(connection, payer, bronzeMint, payer.publicKey);
        await utils.mint(connection, bronzeMint, payer, bronzeTA.address, 1, 0);
        await instructions.lock(program, payer, bronzeMint, bronzeTA.address);
    });

    it('liquidation', async () => {
        await instructions.liquidationUpdate(program, payer);
        await instructions.liquidationLog(program, payer);
    });

    it('unlock', async () => {
        const bronzeTA = await getOrCreateAssociatedTokenAccount(connection, payer, bronzeMint, payer.publicKey);
        await instructions.unlock(program, payer, bronzeTA.address);
    });

    it('close locking account', async () => {
        await instructions.closeLockingAccount(program, payer);
    });

});