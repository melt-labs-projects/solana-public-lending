import * as lockingInstructions from '../locking/instructions';
import * as whitelistingInstructions from '../whitelisting/instructions';
import * as whitelisting from '../whitelisting';
import * as instructions from './instructions';
import * as utils from '../utils';
import * as model from './model';
import * as checks from './checks';
import * as pda from './pda';
import { getOrCreateAssociatedTokenAccount } from '@solana/spl-token';


const DD = 9;
const NFTD = 0;

const state = {
    pool: null,
    node: null,
    baseMint: null,
    baseOracle: null,
    collection: null,
    collaterals: [],
    collectionOracle: null,
    model: null,
    goldMint: null,
    silverMint: null,
    bronzeMint: null,
};

describe("lending", () => {

    // Configure the client to use the local cluster.
    const { provider, connection, payer } = utils.getConnection();
    const program = utils.getLendingProgram();
    const whitelistingProgram = utils.getWhitelistingProgram();

    it("init authority", async () => {
        await instructions.admin.initAuthority(program, payer);
        await checks.checkAuthority(program, payer.publicKey);
    });

    it("update authority", async () => {
        await instructions.admin.updateAuthority(program, payer, payer.publicKey);
        await checks.checkAuthority(program, payer.publicKey);
    });

    it("init oracle", async () => {
        const price = utils.bn(11_000, DD)
        state.collectionOracle = await instructions.admin.initOracle(program, payer, utils.bn(DD), price);
        await checks.checkOracle(program, state.collectionOracle, price);
    });

    it('update oracle', async () => {
        const price = utils.bn(10_000, DD)
        await instructions.admin.updateOracle(program, payer, state.collectionOracle, price);
        await checks.checkOracle(program, state.collectionOracle, price);
    });

    it('init collection', async () => {
        const maxCollaterals = utils.bn(100);
        state.collection = await instructions.admin.initCollection(program, payer, state.collectionOracle, maxCollaterals);
        await checks.checkInitialisedCollection(program, state.collection, state.collectionOracle, maxCollaterals);
    });

    it('update collection', async () => {
        const maxCollaterals = utils.bn(10);
        const price = utils.bn(10_000, DD)
        state.collectionOracle = await instructions.admin.initOracle(program, payer, utils.bn(DD), price);
        await instructions.admin.updateCollection(program, payer, state.collection, state.collectionOracle, maxCollaterals);
        await checks.checkInitialisedCollection(program, state.collection, state.collectionOracle, maxCollaterals);
    });

    it('whitelist nft', async () => {
        // Make sure the whitelisting program is ready 
        const isInitialised = await whitelisting.isInitialised(whitelistingProgram) 
        if (!isInitialised) await whitelistingInstructions.initAuthority(whitelistingProgram, payer);
        const mint = await utils.newMint(connection, payer, NFTD);
        await whitelistingInstructions.whitelist(whitelistingProgram, payer, state.collection, mint);
        await whitelisting.checks.checkWhitelistedNFT(whitelistingProgram, state.collection, mint);
        state.collaterals.push(mint);
    });

    it('init slope model', async () => {
        state.model = await instructions.admin.initNSlopeModel(program, payer, 
            model.from([
                model.point(0, 0),
                model.point(0.5, 0.5),
                model.point(1, 2),
            ])
        );
    });

    it('init pool', async () => {
        state.baseMint = await utils.newMint(connection, payer, DD);
        state.baseOracle = await instructions.admin.initOracle(program, payer, utils.bn(DD), utils.bn(1, DD));
        const facilitatorFee = utils.bn(1_000);
        state.pool = await instructions.admin.initPool(program, payer, state.model, state.baseMint, state.baseOracle, facilitatorFee);
        await checks.checkInitialisedPool(program, state.pool, state.baseMint, state.model, state.baseOracle, facilitatorFee);
    });

    it('update pool', async () => {
        state.baseOracle = await instructions.admin.initOracle(program, payer, utils.bn(DD), utils.bn(1, DD));
        state.model = await instructions.admin.initNSlopeModel(program, payer, 
            model.from([
                model.point(0, 0),
                model.point(0.5, 0.5),
                model.point(1, 2),
            ])
        );
        const facilitatorFee = utils.bn(0);
        await instructions.admin.updatePool(program, payer, state.pool, state.baseOracle, state.model, facilitatorFee);
        await checks.checkInitialisedPool(program, state.pool, state.baseMint, state.model, state.baseOracle, facilitatorFee);
    });

    it('init node', async () => {
        const nodeParams = instructions.admin.nodeParams(true, utils.bn(1_000), utils.bn(2_000));
        state.node = await instructions.admin.initNode(program, payer, state.pool, state.collection, nodeParams);
        await checks.checkInitialisedNode(program, state.pool, state.collection, nodeParams);
    });

    it('update node', async () => {
        const nodeParams = instructions.admin.nodeParams(true, utils.bn(5_000), utils.bn(8_000));
        await instructions.admin.updateNode(program, payer, state.pool, state.collection, nodeParams);
        await checks.checkInitialisedNode(program, state.pool, state.collection, nodeParams);
    });

    it('init slices', async () => {
        await instructions.admin.initSlice(program, payer, state.collection, 0);
        await instructions.admin.initSlice(program, payer, state.collection, 1);
        await instructions.admin.initSlice(program, payer, state.collection, 2);
        await instructions.admin.initSlice(program, payer, state.collection, 3);
    });

    it('deposit tokens', async () => {
        const poolDataBefore = await program.account.lendingPool.fetch(state.pool);
        const baseTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, state.baseMint, payer.publicKey);
        const creditTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, poolDataBefore.creditMint, payer.publicKey);
        await utils.mint(connection, state.baseMint, payer, baseTokenAccount.address, 11_000, DD);

        await checks.assertBalance(connection, creditTokenAccount.address, 0);
        await checks.assertBalance(connection, baseTokenAccount.address, 11_000);
        await checks.assertBalance(connection, poolDataBefore.treasury, 0);

        await instructions.lending.depositTokens(
            program,
            payer,
            state.pool,
            baseTokenAccount.address,
            creditTokenAccount.address,
            utils.bn(11_000, DD)
        );

        // Make sure tokens were transfered to the treasury 
        await checks.assertBalance(connection, creditTokenAccount.address, 11_000);
        await checks.assertBalance(connection, baseTokenAccount.address, 0);
        await checks.assertBalance(connection, poolDataBefore.treasury, 11_000);

        const poolDataAfter = await program.account.lendingPool.fetch(state.pool);
        checks.checkDepositTokens(poolDataBefore, poolDataAfter, utils.bn(11_000, DD));

    });

    it('withdraw tokens', async () => {
        const poolDataBefore = await program.account.lendingPool.fetch(state.pool);
        const baseTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, state.baseMint, payer.publicKey);
        const creditTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, poolDataBefore.creditMint, payer.publicKey);
        
        await checks.assertBalance(connection, creditTokenAccount.address, 11_000);
        await checks.assertBalance(connection, baseTokenAccount.address, 0);
        await checks.assertBalance(connection, poolDataBefore.treasury, 11_000);

        await instructions.lending.withdrawTokens(
            program,
            payer,
            state.pool,
            baseTokenAccount.address,
            creditTokenAccount.address,
            utils.bn(1_000, DD)
        );

        await checks.assertBalance(connection, creditTokenAccount.address, 10_000);
        await checks.assertBalance(connection, baseTokenAccount.address, 1_000);
        await checks.assertBalance(connection, poolDataBefore.treasury, 10_000);

        const poolDataAfter = await program.account.lendingPool.fetch(state.pool);
        checks.checkWithdrawTokens(poolDataBefore, poolDataAfter, utils.bn(1_000, DD));

    });

    it('init borrow account', async () => {
        const collateralCapacity = utils.bn(20);
        await instructions.borrowing.initBorrowAccount(program, payer, state.node, collateralCapacity);
        await checks.checkInitialisedBorrowAccount(program, state.node, payer.publicKey, collateralCapacity);
    });

    it('deposit collateral', async () => {

        const collectionDataBefore = await program.account.collectionAccount.fetch(state.collection);
        const mint = state.collaterals[0];
        const userTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, mint, payer.publicKey);
        await utils.mint(connection, mint, payer, userTokenAccount.address, 1, NFTD);

        await checks.assertBalance(connection, userTokenAccount.address, 1);

        await instructions.borrowing.depositCollateral(
            program,
            whitelistingProgram,
            payer,
            state.pool,
            state.collection,
            mint,
            userTokenAccount.address
        );
        
        await checks.checkInitialisedCollateralAccount(program, state.node, mint, payer.publicKey);

        const [collateralAccount, ] = await pda.findCollateral(program, state.collection, mint);
        const collateralData = await program.account.collateralAccount.fetch(collateralAccount);

        // Make sure the NFT got moved into the collateral account
        await checks.assertBalance(connection, userTokenAccount.address, 0);
        await checks.assertBalance(connection, collateralData.collateralTokenAccount, 1);

        await checks.checkDepositedCollateral(program, state.node, mint, payer.publicKey);

        const collectionDataAfter = await program.account.collectionAccount.fetch(state.collection);
        await checks.checkCollectionAddedCollateral(collectionDataBefore, collectionDataAfter);
    });

    it('borrow', async () => {

        const [borrowAccount, ] = await pda.findBorrowAccount(program, state.node, payer.publicKey);
        const poolDataBefore = await program.account.lendingPool.fetch(state.pool);
        const borrowDataBefore = await program.account.borrowAccount.fetch(borrowAccount);

        const baseTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, state.baseMint, payer.publicKey);
        
        await checks.assertBalance(connection, baseTokenAccount.address, 1_000);
        await checks.assertBalance(connection, poolDataBefore.treasury, 10_000);

        const borrowAmount = utils.bn(5_000, DD)
        await instructions.borrowing.borrow(
            program,
            payer,
            state.pool,
            state.collection,
            baseTokenAccount.address,
            borrowAmount
        );

        await checks.assertBalance(connection, baseTokenAccount.address, 6_000);
        await checks.assertBalance(connection, poolDataBefore.treasury, 5_000);

        const poolDataAfter = await program.account.lendingPool.fetch(state.pool);
        const borrowDataAfter = await program.account.borrowAccount.fetch(borrowAccount);

        // Check pool + borrow account
        await checks.checkBorrow(poolDataBefore, poolDataAfter, borrowDataBefore, borrowDataAfter, borrowAmount);


    });

    it('repay', async () => {

        const [borrowAccount, ] = await pda.findBorrowAccount(program, state.node, payer.publicKey);
        const poolDataBefore = await program.account.lendingPool.fetch(state.pool);
        const borrowDataBefore = await program.account.borrowAccount.fetch(borrowAccount);

        const baseTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, state.baseMint, payer.publicKey);
        
        await checks.assertBalance(connection, baseTokenAccount.address, 6_000);
        await checks.assertBalance(connection, poolDataBefore.treasury, 5_000);
        
        const repaymentAmount = utils.bn(6_000, DD);
        await instructions.borrowing.repay(
            program,
            payer,
            state.pool,
            state.collection,
            baseTokenAccount.address,
            repaymentAmount
        );

        await checks.assertApproxBalance(connection, baseTokenAccount.address, 1_000);
        await checks.assertApproxBalance(connection, poolDataBefore.treasury, 10_000);

        const poolDataAfter = await program.account.lendingPool.fetch(state.pool);
        const borrowDataAfter = await program.account.borrowAccount.fetch(borrowAccount);

        // Check pool + borrow account
        await checks.checkRepay(poolDataBefore, poolDataAfter, borrowDataBefore, borrowDataAfter, repaymentAmount);

    });

    it('withdraw collateral', async () => {
       
        const collectionDataBefore = await program.account.collectionAccount.fetch(state.collection);
        const mint = state.collaterals[0];
        const [collateralAccount, ] = await pda.findCollateral(program, state.collection, mint);
        const collateralData = await program.account.collateralAccount.fetch(collateralAccount);
        const userTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, mint, payer.publicKey);

        await checks.assertBalance(connection, userTokenAccount.address, 0);
        await checks.assertBalance(connection, collateralData.collateralTokenAccount, 1);

        await instructions.borrowing.withdrawCollateral(
            program,
            payer,
            state.pool,
            state.collection,
            mint,
            userTokenAccount.address
        );

        // Make sure the collateral was withdrawn 
        await checks.assertBalance(connection, userTokenAccount.address, 1);

        await checks.checkWithdrewCollateral(program, state.node, mint, payer.publicKey);

        const collectionDataAfter = await program.account.collectionAccount.fetch(state.collection);
        await checks.checkCollectionRemovedCollateral(collectionDataBefore, collectionDataAfter);

    });

    it('close borrow account', async () => {
        await instructions.borrowing.closeBorrowAccount(program, payer, state.node);
    });

    it('liquidate', async () => {
        const mint = state.collaterals[0];
        const assetTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, mint, payer.publicKey);
        const baseTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, state.baseMint, payer.publicKey);

        // Initialise borrow account
        const capacity = utils.bn(20);
        await instructions.borrowing.initBorrowAccount(program, payer, state.node, capacity);

        // Deposit collateral
        await instructions.borrowing.depositCollateral(
            program,
            whitelistingProgram,
            payer,
            state.pool,
            state.collection,
            mint,
            assetTokenAccount.address
        );

        // Borrow max
        await instructions.borrowing.borrow(
            program,
            payer,
            state.pool,
            state.collection,
            baseTokenAccount.address,
            utils.bn(5_000, DD)
        );

        // Lower oracle price
        await instructions.admin.updateOracle(
            program, payer, state.collectionOracle, utils.bn(5_000, DD)
        );

        const poolDataBefore = await program.account.lendingPool.fetch(state.pool);
        const collectionDataBefore = await program.account.collectionAccount.fetch(state.collection);
        const [borrowAccount, ] = await pda.findBorrowAccount(program, state.node, payer.publicKey);
        const borrowData = await program.account.borrowAccount.fetch(borrowAccount);

        // Liquidate
        await instructions.liquidation.liquidate(program, payer, state.pool, state.collection, mint);

        const collectionDataAfter = await program.account.collectionAccount.fetch(state.collection);
        const poolDataAfter = await program.account.lendingPool.fetch(state.pool);

        await checks.checkAddedToSlice(program, state.node, borrowData, 0);
        await checks.checkCollateralsInfoAfterLiquidation(program, poolDataBefore, borrowData, 0);
        await checks.checkCollectionInfoAfterLiquidation(collectionDataBefore, collectionDataAfter, 1);
        await checks.checkPoolAfterLiquidation(poolDataBefore, poolDataAfter, borrowData);
    });
    
    it('purchase', async () => {

        const locking = utils.getLockingProgram();
        await lockingInstructions.initAuthority(locking, payer);

        const goldPerks = lockingInstructions.liquidationPerks(
            utils.bn(2_000), // 20% discount
            utils.bn(5), // 5 uses
            utils.bn(0), // 1 day warmup 
            utils.bn(60 * 60 * 24 * 7), // Refresh weekly
        )

        await lockingInstructions.initMembership(locking, payer, 2, goldPerks);
        state.goldMint = await utils.newMint(connection, payer, 0);
        await lockingInstructions.initCube(locking, payer, state.goldMint, 2);
        const goldTA = await getOrCreateAssociatedTokenAccount(connection, payer, state.goldMint, payer.publicKey);
        await utils.mint(connection, state.goldMint, payer, goldTA.address, 1, 0);

        // User locks cube for discount
        await lockingInstructions.initLockingAccount(locking, payer);
        await lockingInstructions.lock(locking, payer, state.goldMint, goldTA.address);

        const assetMint = state.collaterals[0]
        const assetTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, assetMint, payer.publicKey);
        const baseTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, state.baseMint, payer.publicKey);

        const poolDataBefore = await program.account.lendingPool.fetch(state.pool);
        const collectionDataBefore = await program.account.collectionAccount.fetch(state.collection);
        const [collateralAccount, ] = await pda.findCollateral(program, state.collection, assetMint);
        const collateralData = await program.account.collateralAccount.fetch(collateralAccount);
        const collateralsBefore = await program.account.collateralList.fetch(collateralData.collaterals);

        await checks.assertBalance(connection, poolDataBefore.reserve, 0);

        // await instructions.admin.updateOracle(program, payer, state.collectionOracle, utils.bn(10_000, DD));
        // await utils.mint(connection, state.baseMint, payer, baseTokenAccount.address, 10_000, DD);

        await instructions.liquidation.purchase(
            program,
            locking,
            payer,
            state.pool,
            state.collection,
            assetMint,
            baseTokenAccount.address,
            assetTokenAccount.address
        );

        const oracleData = await program.account.oracleAccount.fetch(state.collectionOracle);
        const salePrice = oracleData.price.mul(utils.bn(8_000)).div(utils.bn(10_000)); // 20% discount

        const poolDataAfter = await program.account.lendingPool.fetch(state.pool);
        const collectionDataAfter = await program.account.collectionAccount.fetch(state.collection);

        await checks.checkCollectionAfterPurchase(collectionDataBefore, collectionDataAfter);
        const reserveIncrease = await checks.checkPoolAfterPurchase(poolDataBefore, poolDataAfter, collateralsBefore, salePrice);

        await checks.assertBalance(connection, poolDataBefore.reserve, reserveIncrease.toNumber() / 1e9);
        await checks.checkRemovedFromSlice(program, state.node, collateralData.collaterals, 0);
        
    });

});



