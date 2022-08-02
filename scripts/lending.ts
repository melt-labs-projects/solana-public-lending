import 'dotenv/config';
import * as anchorUtils from './utils';
import * as utils from '../tests/utils';
import * as model from '../tests/lending/model';
import * as lendingIxs from '../tests/lending/instructions';
import * as whitelisting from '../tests/whitelisting';
import { AnchorProvider } from '@project-serum/anchor';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import { instructions } from '../tests/whitelisting';

const DEFAULT_DECIMALS = 9;

const lendingProgram = anchorUtils.getLendingProgram("mainnet");
const whitelistingProgram = anchorUtils.getWhitelistingProgram();
const { connection, wallet } = lendingProgram.provider as AnchorProvider;
const payer = (wallet as NodeWallet).payer;

const send_sol = async (to) => {
    await connection.requestAirdrop(to, LAMPORTS_PER_SOL);
}

const create_base_mint = async () => {
    return await utils.newMint(connection, payer, DEFAULT_DECIMALS);
}

const mint_tokens = async (mint, to, amount) => {
    const tokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, mint, to);
    await utils.mint(connection, mint, payer, tokenAccount.address, amount, DEFAULT_DECIMALS);
}

const create_oracle = async (price) => {
    return await lendingIxs.admin.initOracle(
        lendingProgram, payer, utils.bn(DEFAULT_DECIMALS), utils.bn(price, DEFAULT_DECIMALS)
    );
}

const update_oracle = async (oracle, price) => {
    await lendingIxs.admin.updateOracle(lendingProgram, payer, oracle, utils.bn(price, DEFAULT_DECIMALS))
}

const create_n_slope_model = async () => {
    const x = await lendingIxs.admin.initNSlopeModel(lendingProgram, payer,
        model.from([
            model.point(0, 0.3),
            model.point(0.8, 0.8),
            model.point(1, 5),
        ])
    );
    console.log(x.toBase58());
    return x;
}

const create_pool = async (baseMint, baseOracle, interestModel) => {
    return await lendingIxs.admin.initPool(
        lendingProgram,
        payer,
        interestModel,
        baseMint,
        baseOracle,
        utils.bn(0)
    );
}

const update_pool = async (pool, baseOracle, interestModel) => {
    await lendingIxs.admin.updatePool(
        lendingProgram,
        payer,
        pool,
        baseOracle,
        interestModel,
        utils.bn(0)
    );
}

const create_collection = async (collectionOracle) => {
    return await lendingIxs.admin.initCollection(lendingProgram, payer, collectionOracle, utils.bn(50));
}

const create_collateral = async (collection, to) => {
    const mint = await utils.newMint(connection, payer, 0);
    console.log(mint.toString());
    const ta = await getOrCreateAssociatedTokenAccount(connection, payer, mint, to);
    await utils.mint(connection, mint, payer, ta.address, 1, 0);
    return await lendingIxs.admin.initCollateral(lendingProgram, payer, collection, mint);
}

const create_node = async (pool, collection) => {
    const nodeParams = lendingIxs.admin.nodeParams(true, utils.bn(4_000), utils.bn(5_000));
    return await lendingIxs.admin.initNode(
        lendingProgram, payer, pool, collection, nodeParams
    );
}

const createBaseToken = async () => {
    const baseMint = await create_base_mint();
    const baseOracle = await create_oracle(1);
    return [baseMint, baseOracle];
}

const createInterestModel = async () => {
    return await create_n_slope_model();
}

const createPool = async (params) => {
    return await create_pool(
        params.baseMint, 
        params.baseOracle, 
        params.interestModel
    );
}

// await update_oracle(COLLECTION_ORACLE, 800);
// await lendingIxs.liquidation.liquidate(
//     lendingProgram,
//     payer,
//     POOL,
//     COLLECTION,
//     new PublicKey("4cQ1EPagCTySMiznrTmahVv5qkvtF2xT1kJa4SdDg767")
// )

const createCollection = async (price) => {
    const collectionOracle = await create_oracle(price);
    // const collectionOracle = new PublicKey("BBDUC1gZG5Zy8pTBkxgnCXEfB3LDnBtWqWoCKA87rnp1");
    const collection = await create_collection(collectionOracle);
    let slices = [];
    for (let i = 0; i < 2; i++) 
        slices.push(lendingIxs.admin.initSlice(lendingProgram, payer, collection, i));
    await Promise.all(slices);
    console.log(collection.toString(), collectionOracle.toString())
    return [collection, collectionOracle];
}

// const createCollectionCollateral = async (params) => {
//     await create_collateral(params.COLLECTION, LOCAL);
// }

const addCollaterals = async (params) => {
    await whitelisting.instructions.initAuthority(whitelistingProgram, payer);
    for (let mint of params.collaterals)
        await whitelisting.instructions.whitelist(whitelistingProgram, payer, params.collection, new PublicKey(mint));
        // await lendingIxs.admin.initCollateral(lendingProgram, payer, params.collection, new PublicKey(mint));
}

const createNode = async (params) => {
    await create_node(params.pool, params.collection);
}


const LOCAL = new PublicKey("8SSovXRiS532H9tjqkmrvECEZWVK4gTGs8kq6pH1nCX2");

const DEVNET_PARAMS_NEW = {
    baseMint: 'DXV8ZpPEKXaAAucQCijQMmi2aRe2cnX6xBiY9PPwdHnw',
    baseOracle: '2wxueqktLxGVEdjQRwvf4HxDxQcniJG1Rpb6VqWhiovh',
    interestModel: '74cZvXQCmKscJRWVF9D4Ph4hUPg4EQbwnY4EwreG6ws4',
    pool: '3LjikETmXVsYD3VcZd5K4fKeDromrnPE8VopvRwB8fvk',
    collection: '3L4ThTiRm2aHH3m13AFqDHJg44hFg482k8ZZiLHyeq8F',
    collectionOracle: 'J4o8bii4dvPs4VBGWrYZLWPmoKz55kKRZ2RqFFhAGyAt',
    collaterals: [
      'Bk6oLspmAwDq8psdVmuxEvQT8372bB2KcY61Q4oeVYsY',
      '6kCbN2z5GRtXLJ289rjq34iuhM9yN6VbFECUEMSnE8jg',
      '7rYU7oC9HYeprLEWvrFBoSSVEV2F8EYNXiXabUq2HLWC',
      'Bp4NWCGx96QBHSskZpHsDqoXSHTu4BBNQN1qmmJNMHe9',
      '8MyCY8KyNnyUmkT3FLAMv2oJs8KHrDbCu22xf9sNq4qk',
      'FGDaZMFz1KC7C3cLhaVmcAv5J4TtvmrfsQXDTEGEtASy',
      'GrL8j8m43r3ZQipBhtyJ5UtANyCShNRtMupuo8i8zEGp',
      'HV5mSmKoiBFhdtmN94mbC9EsMcq2GTaQtzCs88234zRE',
      'HAr2TL6mKiYqiPqyVDn5dsQ2N22sN4Y5HeTqBQjKdc5p'
    ]
  }



// const COLLECTION_STONED_FROGS = new PublicKey("");

const parseParams = (params) => {
    for (let key of Object.keys(params)) {
        if (key == 'collaterals') {
            for (let i = 0; i < params[key].length; i++) 
                params[key][i] = new PublicKey(params[key][i]);
        }
        else params[key] = new PublicKey(params[key]);
    }
    return params
}

const createDevnetPool = async (params) => {

    // await lendingIxs.admin.initAuthority(lendingProgram, payer);

    if (!params.baseMint) {
        console.log("Creating base mint + oracle");
        const [baseMint, baseOracle] = await createBaseToken();
        params.baseMint = baseMint;
        params.baseOracle = baseOracle;
    }
    
    if (!params.interestModel) {
        console.log("Creating interest rate model");
        params.interestModel = await createInterestModel();
    }

    console.log("Creating pool");
    params.pool = await createPool(params);

    console.log("Creating collection");
    const [collection, collectionOracle] = await createCollection(1000);
    params.collection = collection;
    params.collectionOracle = collectionOracle;

    console.log("Creating node")
    await createNode(params);

    console.log("Adding collaterals");
    params.collaterals = [
        "Bk6oLspmAwDq8psdVmuxEvQT8372bB2KcY61Q4oeVYsY",
        "6kCbN2z5GRtXLJ289rjq34iuhM9yN6VbFECUEMSnE8jg",
        "7rYU7oC9HYeprLEWvrFBoSSVEV2F8EYNXiXabUq2HLWC",
        "Bp4NWCGx96QBHSskZpHsDqoXSHTu4BBNQN1qmmJNMHe9",
        "8MyCY8KyNnyUmkT3FLAMv2oJs8KHrDbCu22xf9sNq4qk",
        "FGDaZMFz1KC7C3cLhaVmcAv5J4TtvmrfsQXDTEGEtASy",
        "GrL8j8m43r3ZQipBhtyJ5UtANyCShNRtMupuo8i8zEGp",
        "HV5mSmKoiBFhdtmN94mbC9EsMcq2GTaQtzCs88234zRE",
        "HAr2TL6mKiYqiPqyVDn5dsQ2N22sN4Y5HeTqBQjKdc5p",
    ];
    await addCollaterals(params);

    console.log({
        "baseMint": params.baseMint.toString(),
        "baseOracle": params.baseOracle.toString(),
        "interestModel": params.interestModel.toString(),
        "pool": params.pool.toString(),
        "collection": params.collection.toString(),
        "collectionOracle": params.collectionOracle.toString(),
        collaterals: params.collaterals
    })
    
}

// createCollection(0)

// createDevnetPool({});
// mint_tokens(parseParams(DEVNET_PARAMS_NEW).baseMint, LOCAL, 1_000_000);
// addCollaterals(parseParams(DEVNET_PARAMS_NEW));

// const PARAMS = parseParams(MAINNET_PARAMS_NEW);
// const nodeParams = lendingIxs.admin.nodeParams(
//     true,
//     utils.bn(4_000),
//     utils.bn(5_000),
// );

// lendingIxs.admin.updateNode(
//     lendingProgram,
//     payer,
//     PARAMS.pool,
//     PARAMS.collection,
//     nodeParams
// );

// lendingIxs.liquidation.liquidate(
//     lendingProgram,
//     payer,
//     PARAMS.pool,
//     PARAMS.collection,
//     PARAMS.collaterals[5]
// )


// update_pool(PARAMS.pool, PARAMS.baseOracle, new PublicKey("HjLXKTttHNDgrc7cVCFqPsVfngsPBg9DdAvBXkUdEwXA"));
// lendingIxs.admin.initAuthority(lendingProgram, payer);
// createCollection(0).then(x => console.log(x[0].toString(), x[1].toString()))
// create_collection(new PublicKey(parseParams(MAINNET_PARAMS_NEW).collectionOracle));

const MAINNET_PARAMS_NEW = {
    baseMint: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
    baseOracle: 'AXMNMoiSmxwfAJ48juQEK7FfGCr12PTRTwLUJNnpVEgw',
    interestModel: 'Ed3js3QDHWT3tsaCkTaaYopvUUDnGnmynoMmaRsvFpJx',
    pool: '6339Zm4AEZKNBacfLYoxpVoCX7CqZGnhRCQnhcLn7ss7',
    collection: '7iW361tfcsUw4PdATgYL79Mq4HXXrbz3aR3kKAm8Dejv',
    collectionOracle: 'FXVU7p3fSGHfh7TvXzPfKd3yhCMHu25mad8hG25n6YPY',
}

// createPool(parseParams(MAINNET_PARAMS_NEW)).then(x => console.log(x.toString()))
// createNode(parseParams(MAINNET_PARAMS_NEW))

// lendingIxs.admin.initOracle(
//     lendingProgram, payer, utils.bn(6), utils.bn(2, 6)
// ).then(x => console.log(x.toBase58()));

// const PARAMS = parseParams(MAINNET_PARAMS_NEW);
// update_pool(PARAMS.pool, PARAMS.baseOracle, PARAMS.interestModel);

// lendingIxs.admin.updateCollection(lendingProgram, payer, PARAMS.collection, PARAMS.collectionOracle, utils.bn(20))

// lendingIxs.admin.updateOracle(lendingProgram, payer, new PublicKey("DJtD7UuqNL8yaqrwn9CA1mMYLB3gR1aYBBtJEFjLj6sW"), utils.bn(5, 6))
// lendingIxs.admin.initSlice(lendingProgram, payer, PARAMS.collection, 1)



// lendingIxs.admin.initOracle(lendingProgram, payer, utils.bn(6), utils.bn(0, 6)).then(x => console.log(x.toString()))

// const COLLECTION_TO_UPDATE = new PublicKey("7iW361tfcsUw4PdATgYL79Mq4HXXrbz3aR3kKAm8Dejv");
// const NEW_ORACLE = new PublicKey("FXVU7p3fSGHfh7TvXzPfKd3yhCMHu25mad8hG25n6YPY");
// lendingIxs.admin.updateCollection(lendingProgram, payer, COLLECTION_TO_UPDATE, NEW_ORACLE, utils.bn(20))
// lendingProgram.account.collectionAccount.fetch(COLLECTION_TO_UPDATE).then(x => console.log(x.))

let collection = new PublicKey("7iW361tfcsUw4PdATgYL79Mq4HXXrbz3aR3kKAm8Dejv")
let slices = [];
for (let i = 0; i < 2; i++) 
    slices.push(lendingIxs.admin.initSlice(lendingProgram, payer, collection, i));
Promise.all(slices).then(() => console.log("we good son"));