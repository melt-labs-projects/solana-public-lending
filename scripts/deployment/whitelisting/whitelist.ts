import * as fs from 'fs'
import * as whitelisting from '../../../tests/whitelisting';
import * as utils from '../../utils';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { AnchorProvider } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { performance } from 'perf_hooks';

const COLLECTION_NAME = "stoned-frogs"
const COLLECTION_PUBKEY = new PublicKey("333Bcq2fZE5kjMwmV9gAJo9o3WgMFyKx3CHAH2zWa8eL");

const JSON_NAME = `./scripts/deployment/whitelisting/collections/${COLLECTION_NAME}/processed.json`
var nfts = JSON.parse(fs.readFileSync(JSON_NAME, 'utf-8'))

const BATCH_SIZE = 8;
const PENDING = new Set();
const START_TIME = performance.now();
var UNIQUE = 0;
var ADDED = 0;

const program = utils.getWhitelistingProgram("mainnet");
const { wallet } = program.provider as AnchorProvider;
const payer = (wallet as NodeWallet).payer;
const last = nfts.length;

const writeJson = () => {
    const json = JSON.stringify(nfts, null, 4);
    fs.writeFileSync(JSON_NAME, json);
}

const isInUse = async (index) => {
    const mint = new PublicKey(nfts[index].address);
    const [whitelisted, ] = await whitelisting.findWNFT(program, COLLECTION_PUBKEY, mint);
    try {
        await program.account.whitelisted.fetch(whitelisted);
        return true;
    } catch (e) {
        return false;
    }
}

const wrappedInitCube = async (index) => {
    try {
        await whitelisting.instructions.whitelist(
            program,
            payer,
            COLLECTION_PUBKEY,
            new PublicKey(nfts[index].address)
        )
    } catch (e) {
        // console.log("WRAPPED ERROR", e)
        return await Promise.reject(new Error("failed"))
    }
}

const getBatch = async () => {
    let index = nfts.findIndex(item => !item['done'])
    console.log("Starting at", index);
    const batch = [];
    let count = 0;
    while (true) {
        
        if (!('done' in nfts[index]) && !PENDING.has(index)) {

            // Make sure it doesn't already exist
            const existsAlready = await isInUse(index);
            if (existsAlready) {
                console.log(index, "exists already...");
                nfts[index]['index'] = index;
                nfts[index]['done'] = true;
                ADDED += 1;
                continue;
            }
            writeJson();

            batch.push(index);
            PENDING.add(index)
            count += 1;
            
        }
        index += 1;
        if (count === BATCH_SIZE || index === last) break;
    }
    return batch;
}



const main = async () => {

    const batch = await getBatch();

    console.log("Processing batch", batch)
    const currentUnique = UNIQUE;
    UNIQUE += 1;
    console.time(`settling-${currentUnique}`);

    const results = await Promise.allSettled(
        batch.map(x => wrappedInitCube(x))
    );

    for (let a = 0; a < results.length; a++) {
        const result = results[a];
        const cubeIndex = batch[a];
        console.log(cubeIndex, result.status);
        if (result.status === "fulfilled") {
            nfts[cubeIndex]['index'] = cubeIndex;
            nfts[cubeIndex]['done'] = true;
            ADDED += 1;
        }
        PENDING.delete(cubeIndex)
    }

    console.timeEnd(`settling-${currentUnique}`);
    console.log();
    writeJson();

    const NOW = performance.now();
    const past = (NOW - START_TIME) / 1000;
    console.log(ADDED, past, 60 / (past / ADDED))

}

main()
setInterval(main, 15000);
// whitelisting.instructions.initAuthority(program, payer);