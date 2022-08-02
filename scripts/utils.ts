import 'dotenv/config'
import { AnchorProvider, Program, Wallet, Idl } from '@project-serum/anchor'
import { PublicKey, Connection } from '@solana/web3.js';
import { lendingIDL, lockingIDL, whitelistingIDL } from './idl';

export const LENDING_PROGRAM_PUBKEY = new PublicKey("4JM47H9t82cK6qtSdZLmnWwbTss6JPH7csJBZz5Qe68o");
export const LOCKING_PROGRAM_PUBKEY = new PublicKey("5DkgLNzMMMdxoEUrS2ZwVWxcsW7E5LAsCLsp25WX6jyE");
export const WHITELISTING_PROGRAM_PUBKEY = new PublicKey("GwUMfoPtUC4EgTtKePwuVmfifpU5zFGmR6mPPVAtthKx");
const ENV = 'devnet';

export const getConnection = (env=ENV) => {
    if (env === "mainnet") return new Connection("https://api.mainnet-beta.solana.com");
    // if (env === "mainnet") return new Connection("https://solana-api.projectserum.com");
    // if (env === "mainnet") return new Connection("https://rpc.ankr.com/solana");
    // if (env === "mainnet") return new Connection("https://ssc-dao.genesysgo.net/");
    if (env === "testnet") return new Connection("https://api.testnet.solana.com");
    if (env === "devnet") return new Connection("https://api.devnet.solana.com");
    if (env === "local") return new Connection("http://127.0.0.1:8899");
}

export const getWallet = () => Wallet.local();

export const getAnchorProvider = (env) => {
    const wallet = getWallet();
    const connection = getConnection(env);
    return new AnchorProvider(connection, wallet, {});
}

export const getLendingProgram = (env=ENV) => {
    return new Program(
        lendingIDL as Idl, 
        LENDING_PROGRAM_PUBKEY, 
        getAnchorProvider(env)
    );
}

export const getLockingProgram = (env=ENV) => {
    return new Program(
        lockingIDL as Idl, 
        LOCKING_PROGRAM_PUBKEY, 
        getAnchorProvider(env)
    );
}

export const getWhitelistingProgram = (env=ENV) => {
    return new Program(
        whitelistingIDL as Idl, 
        WHITELISTING_PROGRAM_PUBKEY, 
        getAnchorProvider(env)
    );
}
