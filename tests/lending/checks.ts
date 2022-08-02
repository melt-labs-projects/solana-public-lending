import assert from 'node:assert/strict';
import { PublicKey } from '@solana/web3.js';
import * as pda from './pda';
import { BN } from '@project-serum/anchor';

const ZERO = new BN(0);


export const checkAuthority = async (program, authority: PublicKey) => {
    const [admin, ] = await pda.findAdmin(program);
    const adminData = await program.account.adminAccount.fetch(admin);
    assert(authority.equals(adminData.authority));
}

export const checkOracle = async (program, oracle: PublicKey, price: BN) => {
    const oracleData = await program.account.oracleAccount.fetch(oracle);
    assert(oracleData.price.eq(price));
}

export const checkInitialisedCollection = async (
    program, 
    collection: PublicKey, 
    oracle: PublicKey,
    maxCollaterals
) => {
    const collectionData = await program.account.collectionAccount.fetch(collection);
    assert(collectionData.collectionOracle.equals(oracle));
    assert(collectionData.maxCollaterals.eq(maxCollaterals));
    assert(collectionData.collateralsInUse.eq(ZERO));
    assert(collectionData.pendingLiquidations.eq(ZERO));
    assert(collectionData.totalLiquidations.eq(ZERO));
    assert(collectionData.totalSlicesLength.eq(ZERO));
}

export const checkInitialisedPool = async (
    program, 
    pool: PublicKey, 
    mint: PublicKey,
    interestRateModel: PublicKey,
    oracle: PublicKey,   
    facilitatorFee,
) => {
    const poolData = await program.account.lendingPool.fetch(pool);
    assert(poolData.baseMint.equals(mint));
    assert(!poolData.creditMint.equals(PublicKey.default));
    assert(!poolData.treasury.equals(PublicKey.default));
    assert(!poolData.reserve.equals(PublicKey.default));
    assert(!poolData.feeTokenAccount.equals(PublicKey.default));
    assert(poolData.availableTokens.eq(ZERO));
    assert(poolData.utilisedTokens.eq(ZERO));
    assert(poolData.pendingTokens.eq(ZERO));
    assert(poolData.lostTokens.eq(ZERO));
    assert(poolData.creditShares.eq(ZERO));
    assert(poolData.debtShares.eq(ZERO));
    assert(poolData.lastUpdateTimestamp.eq(ZERO));
    assert(poolData.interestRate.eq(ZERO));
    assert(poolData.interestRateModel.equals(interestRateModel));
    assert(poolData.baseOracle.equals(oracle));
    assert(poolData.params.facilitatorFee.eq(facilitatorFee));
}

export const checkInitialisedNode = async (
    program, 
    pool: PublicKey, 
    collection: PublicKey,
    params,
) => {
    const [node, ] = await pda.findNode(program, pool, collection);
    const nodeData = await program.account.borrowingNode.fetch(node);
    assert(nodeData.pool.equals(pool));
    assert(nodeData.collection.equals(collection));
    assert(nodeData.params.enabled == params.enabled);
    assert(nodeData.params.maxBorrowRatio.eq(params.maxBorrowRatio));
    assert(nodeData.params.liquidationRatio.eq(params.liquidationRatio));
}

export const assertBalance = async (connection, account, expected) => {
    const balance = (await connection.getTokenAccountBalance(account)).value.uiAmount;
    assert(balance === expected);
}

export const assertApproxBalance = async (connection, account, expected, eps=0.001) => {
    const balance = (await connection.getTokenAccountBalance(account)).value.uiAmount;
    assert(Math.abs(balance - expected) < eps);
}

const amountToCreditShares = (poolData, amount) => {
    const total = poolData.availableTokens.add(poolData.utilisedTokens).add(poolData.pendingTokens);
    if (total.eq(ZERO)) return amount;
    return (poolData.creditShares.mul(amount)).div(total);
}

const amountToDebtShares = (poolData, amount) => {
    const total = poolData.utilisedTokens;
    if (total.eq(ZERO)) return amount;
    return (poolData.debtShares.mul(amount)).div(total);
}

const debtSharesToAmount = (poolData, shares) => {
    if (shares.eq(ZERO)) return shares;
    return (poolData.utilisedTokens.mul(shares)).div(poolData.debtShares);
}

export const checkDepositTokens = (poolDataBefore, poolDataAfter, amount) => {
    const newCreditShares = amountToCreditShares(poolDataBefore, amount);
    assert(poolDataAfter.availableTokens.eq(poolDataBefore.availableTokens.add(amount)));
    assert(poolDataAfter.creditShares.eq(poolDataBefore.creditShares.add(newCreditShares)));
}

export const checkWithdrawTokens = (poolDataBefore, poolDataAfter, amount) => {
    const creditShares = amountToCreditShares(poolDataBefore, amount);
    assert(poolDataAfter.availableTokens.eq(poolDataBefore.availableTokens.sub(amount)));
    assert(poolDataAfter.creditShares.eq(poolDataBefore.creditShares.sub(creditShares)));
}

export const checkInitialisedBorrowAccount = async (program, node, borrower, capacity) => {
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, borrower);
    const borrowData = await program.account.borrowAccount.fetch(borrowAccount);
    assert(borrowData.node.equals(node));
    assert(!borrowData.collaterals.equals(PublicKey.default));
    assert(borrowData.shares.eq(ZERO));
    assert(borrowData.outstanding.eq(ZERO));
    const collaterals = await program.account.collateralList.fetch(borrowData.collaterals);
    assert(collaterals.borrower.equals(borrower));
    assert(collaterals.size.eq(capacity));
    assert(collaterals.pendingTokensPerCollateral.eq(ZERO));
    assert(collaterals.sliceNumber.eq(ZERO));
    assert(!collaterals.isPendingLiquidation);
    assert(collaterals.vector.length === 0);
}

export const checkInitialisedCollateralAccount = async (program, node, mint, borrower) => {
    const nodeData = await program.account.borrowingNode.fetch(node);
    const [collateralAccount, ] = await pda.findCollateral(program, nodeData.collection, mint);
    const collateralData = await program.account.collateralAccount.fetch(collateralAccount);
    assert(collateralData.mint.equals(mint));
    assert(collateralData.borrower.equals(borrower));
    assert(!collateralData.collateralTokenAccount.equals(PublicKey.default));

    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, borrower);
    const borrowData = await program.account.borrowAccount.fetch(borrowAccount);
    assert(collateralData.collaterals.equals(borrowData.collaterals));
}

export const checkDepositedCollateral = async (program, node, mint, borrower) => {
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, borrower);
    const borrowData = await program.account.borrowAccount.fetch(borrowAccount);
    const collaterals = await program.account.collateralList.fetch(borrowData.collaterals);
    const last = collaterals.vector.length - 1;
    assert(collaterals.vector[last].equals(mint));
}

export const checkCollectionAddedCollateral = async (collectionDataBefore, collectionDataAfter) => {
    assert(collectionDataAfter.collateralsInUse.eq(collectionDataBefore.collateralsInUse.add(new BN(1))));
}

export const checkBorrow = async (poolDataBefore, poolDataAfter, borrowAccountBefore, borrowAccountAfter, amount) => {
    const debtShares = amountToDebtShares(poolDataBefore, amount);
    assert(poolDataAfter.debtShares.eq(poolDataBefore.debtShares.add(debtShares)));
    assert(poolDataAfter.availableTokens.eq(poolDataBefore.availableTokens.sub(amount)));
    assert(poolDataAfter.utilisedTokens.eq(poolDataBefore.utilisedTokens.add(amount)));
    assert(borrowAccountAfter.outstanding.eq(borrowAccountBefore.outstanding.add(amount)));
    assert(borrowAccountAfter.shares.eq(borrowAccountBefore.shares.add(debtShares)));
}

const assertApproxEqual = (a, b) => {
    const difference = (b.sub(a)).abs();
    assert(difference.lt(new BN(1_000_000)));
}

export const checkRepay = async (poolDataBefore, poolDataAfter, borrowAccountBefore, borrowAccountAfter, amount) => {
    const debtAmount = debtSharesToAmount(poolDataBefore, borrowAccountBefore.shares);
    const amountToPay = BN.min(amount, debtAmount);
    const debtShares = amountToDebtShares(poolDataBefore, amountToPay);
    assert(poolDataAfter.debtShares.eq(poolDataBefore.debtShares.sub(debtShares)));

    // console.log(
    //     poolDataBefore.availableTokens.toString(), 
    //     poolDataAfter.availableTokens.toString(),
    //     amountToPay.toString()
    // );
    
    assertApproxEqual(poolDataAfter.availableTokens, poolDataBefore.availableTokens.add(amountToPay));
    assertApproxEqual(poolDataAfter.utilisedTokens, poolDataBefore.utilisedTokens.sub(amountToPay));
    const amountOffOutstanding = BN.min(borrowAccountBefore.outstanding, amountToPay);
    assertApproxEqual(borrowAccountAfter.outstanding, borrowAccountBefore.outstanding.sub(amountOffOutstanding));
    assertApproxEqual(borrowAccountAfter.shares, borrowAccountBefore.shares.sub(debtShares));
}

export const checkWithdrewCollateral = async (program, node, mint, borrower) => {
    const [borrowAccount, ] = await pda.findBorrowAccount(program, node, borrower);
    const borrowData = await program.account.borrowAccount.fetch(borrowAccount);
    const collaterals = await program.account.collateralList.fetch(borrowData.collaterals);
    if (collaterals.vector.length > 0) {
        const last = collaterals.vector.length - 1;
        assert(!collaterals.vector[last].equals(mint));
    }
}

export const checkCollectionRemovedCollateral = async (collectionDataBefore, collectionDataAfter) => {
    assert(collectionDataAfter.collateralsInUse.eq(collectionDataBefore.collateralsInUse.sub(new BN(1))));
}

export const checkAddedToSlice = async (program, node, borrowData, sliceNumber) => {
    const nodeData = await program.account.borrowingNode.fetch(node);
    const [slice, ] = await pda.findSlice(program, nodeData.collection, sliceNumber);
    const sliceData = await program.account.lSlice.fetch(slice);
    const last = sliceData.vector.length - 1;
    assert(sliceData.vector[last].equals(borrowData.collaterals));
}

export const checkCollectionInfoAfterLiquidation = async (collectionDataBefore, collectionDataAfter, collaterals) => {
    assert(collectionDataAfter.pendingLiquidations.eq(collectionDataBefore.pendingLiquidations.add(new BN(collaterals))));
    assert(collectionDataAfter.totalSlicesLength.eq(collectionDataBefore.totalSlicesLength.add(new BN(1))));
}

export const checkCollateralsInfoAfterLiquidation = async (program, poolData, borrowData, sliceNumber) => {
    const pendingTokens = debtSharesToAmount(poolData, borrowData.shares);
    const collaterals = await program.account.collateralList.fetch(borrowData.collaterals);
    assert(collaterals.sliceNumber.toNumber() === sliceNumber);
    assert(collaterals.isPendingLiquidation);
    assertApproxEqual(collaterals.pendingTokensPerCollateral, pendingTokens.div(new BN(collaterals.vector.length)));
}

export const checkPoolAfterLiquidation = async (poolDataBefore, poolDataAfter, borrowData) => {
    const pendingTokens = debtSharesToAmount(poolDataBefore, borrowData.shares);
    assertApproxEqual(poolDataAfter.utilisedTokens, poolDataBefore.utilisedTokens.sub(pendingTokens));
    assertApproxEqual(poolDataAfter.pendingTokens, poolDataBefore.pendingTokens.add(pendingTokens));
    assertApproxEqual(poolDataAfter.debtShares, poolDataBefore.debtShares.sub(borrowData.shares));
}

export const checkPoolAfterPurchase = async (poolDataBefore, poolDataAfter, collaterals, salesPrice) => {
    const pendingAmount = collaterals.pendingTokensPerCollateral;
    assert(poolDataAfter.pendingTokens.eq(poolDataBefore.pendingTokens.sub(pendingAmount)));
    // console.log(
    //     salesPrice.toString(),
    //     pendingAmount.toString()
    // )
    if (salesPrice.gt(pendingAmount)) {
        assert(poolDataAfter.availableTokens.eq(poolDataBefore.availableTokens.add(pendingAmount)));
        assert(poolDataAfter.lostTokens.eq(poolDataBefore.lostTokens));
        const reserveAmount = salesPrice.sub(pendingAmount);
        return reserveAmount;
    } else {
        const lostAmount = pendingAmount.sub(salesPrice);
        // console.log(
        //     lostAmount.toString(),
        //     poolDataBefore.lostTokens.toString(),
        //     poolDataAfter.lostTokens.toString()
        // )
        assert(poolDataAfter.lostTokens.eq(poolDataBefore.lostTokens.add(lostAmount)));
        assert(poolDataAfter.availableTokens.eq(poolDataBefore.availableTokens.add(salesPrice)));
        return ZERO;
    }
}

export const checkCollectionAfterPurchase = async (collectionDataBefore, collectionDataAfter) => {
    assert(collectionDataAfter.totalLiquidations.eq(collectionDataBefore.totalLiquidations.add(new BN(1))));
    assert(collectionDataAfter.collateralsInUse.eq(collectionDataBefore.collateralsInUse.sub(new BN(1))));
    assert(collectionDataAfter.pendingLiquidations.eq(collectionDataBefore.pendingLiquidations.sub(new BN(1))));
    assert(collectionDataAfter.totalSlicesLength.eq(collectionDataBefore.totalSlicesLength.sub(new BN(1))));
}

export const checkRemovedFromSlice = async (program, node, collaterals, sliceNumber) => {
    const nodeData = await program.account.borrowingNode.fetch(node);
    const [slice, ] = await pda.findSlice(program, nodeData.collection, sliceNumber);
    const sliceData = await program.account.lSlice.fetch(slice);
    if (sliceData.vector.length > 0) {
        const last = sliceData.vector.length - 1;
        assert(!sliceData.vector[last].equals(collaterals));
    }
}