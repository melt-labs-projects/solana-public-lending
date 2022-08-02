import * as fs from 'fs'
import cubes from './cubes_done.json';
import * as locking from '../../../tests/locking';
import * as utils from '../../utils';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { AnchorProvider } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { rejects } from 'assert';
import { performance } from 'perf_hooks';

const MEMBERSHIP = {
    "Bronze": 2,
    "Silver": 1,
    "Gold": 0
};

const BATCH_SIZE = 8;
const PENDING = new Set();
var UNIQUE = 0;

const START_TIME = performance.now();
var ADDED = 0;

const program = utils.getLockingProgram("mainnet");
const { wallet } = program.provider as AnchorProvider;
const payer = (wallet as NodeWallet).payer;
const last = cubes.length - 1;

const writeJson = () => {
    const json = JSON.stringify(cubes, null, 4);
    fs.writeFileSync("./scripts/deployment/locking/cubes_done.json", json);
}

const isCubeInUse = async (index) => {
    const mint = new PublicKey(cubes[index].address);
    const [cube, ] = await locking.pda.findCube(program, mint);
    try {
        await program.account.cubeAccount.fetch(cube);
        return true;
    } catch (e) {
        return false;
    }
}

const wrappedInitCube = async (index) => {
    try {
        await locking.instructions.initCube(
            program, 
            payer, 
            new PublicKey(cubes[index].address),
            MEMBERSHIP[cubes[index].membership]
        )
    } catch (e) {
        // console.log("WRAPPED ERROR", e)
        return await Promise.reject(new Error("failed"))
    }
}

const getBatch = async () => {
    let index = cubes.findIndex(item => !item['done'])
    console.log("Starting at", index);
    const batch = [];
    let count = 0;
    while (true) {
        if (!('done' in cubes[index]) && !PENDING.has(index)) {

            // Make sure it doesn't already exist
            const existsAlready = await isCubeInUse(index);
            if (existsAlready) {
                console.log(index, "exists already...");
                cubes[index]['index'] = index;
                cubes[index]['done'] = true;
                ADDED += 1;
                continue;
            }
            writeJson();

            batch.push(index);
            PENDING.add(index)
            count += 1;
        }
        if (count === BATCH_SIZE || index === last) break;
        index += 1;
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
            cubes[cubeIndex]['index'] = cubeIndex;
            cubes[cubeIndex]['done'] = true;
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