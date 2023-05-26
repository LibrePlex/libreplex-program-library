import { LibreplexClient } from '../metadata';
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { promises as fs } from 'fs';
import { default as yargs } from 'yargs';
import * as anchor from '@coral-xyz/anchor';
import { IDL as LibreplexIDL } from '../types/libreplex';
import { METADATA_PROGRAM_ID } from '../index';
import { stringifyPKsAndBNs } from '../prog-common';

import { networkConfig } from '../cli/config_devnet/networkConfig';
import { collectionDataConfig } from '../cli/config_devnet/collectionDataConfig';
import { metadataInputCli } from '../cli/config_devnet/metadataConfig';

// ----------------------------------------------- Legend ---------------------------------------------------------

// -a authority account address (authority)
// -c collection data account address (comment)
// -k pubkey of account being fetched (key)
// -m metadata account address (metadata)
// -r receiver account address (receiver)
// -t token mint address (minT)
// -u unix timestamp (unix)
// -z dryRun



const parser = yargs(process.argv.slice(2)).options({
    dryRun: {
        alias: 'z',
        type: 'boolean',
        default: false,
        description: 'set Dry Run flag'
    },
})



// --------------------------------------------- authority instructions ---------------------------------------------



// Create collection data account (payer = authority)
// Must config collection data inputs in collectionDataConfig
    .command('create-collection-data', 'Create a collection data PDA account', {
    },
             async (argv) => {
                 const rpcConn = new Connection(networkConfig.clusterApiUrl, { confirmTransactionInitialTimeout: 91000 });
                 const wallet: anchor.Wallet = new anchor.Wallet(await loadWallet(networkConfig.signerKeypair));
                 const libreplexClient: LibreplexClient = new LibreplexClient(
                     rpcConn,
                     wallet,
                     LibreplexIDL,
                     METADATA_PROGRAM_ID,
                 );

                 const collectionDataInput = collectionDataConfig.collectionDataInput;

                 if (!argv.dryRun) {
                     const createCollectionDataInstance = await libreplexClient.createCollectionData(
                         wallet.payer,
                         collectionDataInput
                     );
                     console.log(stringifyPKsAndBNs(createCollectionDataInstance));
                 } else {
                     //console.log('Creating collection data account for authority with pubkey', stringifyPKsAndBNs(wallet.publicKey));
                 }
             })



// Delete collection data account
    .command('delete-collection-data', 'Delete a collection data PDA account', {
        collectionDataPubkey: {
            alias: 'c',
            type: 'string',
            demandOption: true,
            description: 'collection data PDA account pubkey'
        },
        receiverPubkey: {
            alias: 'r',
            type: 'string',
            demandOption: false,
            description: 'receiver account pubkey for reclaimed rent lamports'
        }
    },
             async (argv) => {
                 const rpcConn = new Connection(networkConfig.clusterApiUrl, { confirmTransactionInitialTimeout: 91000 });
                 const wallet: anchor.Wallet = new anchor.Wallet(await loadWallet(networkConfig.signerKeypair));
                 const libreplexClient: LibreplexClient = new LibreplexClient(
                     rpcConn,
                     wallet,
                     LibreplexIDL,
                     METADATA_PROGRAM_ID,
                 );

                 const collectionDataKey: PublicKey = new PublicKey(argv.collectionDataPubkey);
                 const collectionDataAcct = await libreplexClient.fetchCollectionDataAccount(collectionDataKey);
                 const collectionDataSeed = collectionDataAcct.collectionSeed;

                 const receiverKey: PublicKey = argv.receiverPubkey? new PublicKey(argv.receiverPubkey) : wallet.publicKey;

                 if (!argv.dryRun) {
                     const deleteCollectionDataInstance = await libreplexClient.deleteCollectionData(
                         wallet.payer,
                         collectionDataSeed,
                         receiverKey,
                     );
                     console.log(stringifyPKsAndBNs(deleteCollectionDataInstance));
                 } else {
                     //console.log('Deleting collection data account for authority with pubkey', stringifyPKsAndBNs(wallet.publicKey));
                 }
             })



// Create metadata account (payer = authority)
// Must config collection data inputs in collectionDataConfig
    .command('create-metadata', 'Create a metadata PDA account', {
    },
             async (argv) => {
                 const rpcConn = new Connection(networkConfig.clusterApiUrl, { confirmTransactionInitialTimeout: 91000 });
                 const wallet: anchor.Wallet = new anchor.Wallet(await loadWallet(networkConfig.signerKeypair));
                 const libreplexClient: LibreplexClient = new LibreplexClient(
                     rpcConn,
                     wallet,
                     LibreplexIDL,
                     METADATA_PROGRAM_ID,
                 );

                 const collectionDataKey = metadataInputCli.collectionDataPubkey;
                 const collectionDataAcct = await libreplexClient.fetchCollectionDataAccount(collectionDataKey);
                 const collectionDataSeed = collectionDataAcct.collectionSeed;

                 const mintWallet: anchor.Wallet = new anchor.Wallet(await loadWallet(metadataInputCli.mint));
                 const metadataInput = metadataInputCli.metadataInput;

                 if (!argv.dryRun) {
                     const createMetadataInstance = await libreplexClient.createMetadata(
                         wallet.payer,
                         mintWallet.payer,
                         collectionDataSeed,
                         metadataInput,
                     );
                     console.log(stringifyPKsAndBNs(createMetadataInstance));
                 } else {
                     console.log('Creating metadata account for mint with pubkey', stringifyPKsAndBNs(mintWallet.publicKey));
                 }
             })



// Delete metadata account
    .command('delete-metadata', 'Delete a metadata PDA account', {
        collectionDataPubkey: {
            alias: 'c',
            type: 'string',
            demandOption: true,
            description: 'collection data PDA account pubkey'
        },
        mintPubkey: {
            alias: 't',
            type: 'string',
            demandOption: true,
            description: 'mint account pubkey'
        },
        receiverPubkey: {
            alias: 'r',
            type: 'string',
            demandOption: false,
            description: 'receiver account pubkey for reclaimed rent lamports'
        }
    },
             async (argv) => {
                 const rpcConn = new Connection(networkConfig.clusterApiUrl, { confirmTransactionInitialTimeout: 91000 });
                 const wallet: anchor.Wallet = new anchor.Wallet(await loadWallet(networkConfig.signerKeypair));
                 const libreplexClient: LibreplexClient = new LibreplexClient(
                     rpcConn,
                     wallet,
                     LibreplexIDL,
                     METADATA_PROGRAM_ID,
                 );

                 const collectionDataKey = new PublicKey(argv.collectionDataPubkey);
                 const collectionDataAcct = await libreplexClient.fetchCollectionDataAccount(collectionDataKey);
                 const collectionDataSeed = collectionDataAcct.collectionSeed;

                 const mintKey = new PublicKey(argv.mintPubkey);
                 const receiverKey: PublicKey = argv.receiverPubkey? new PublicKey(argv.receiverPubkey) : wallet.publicKey;

                 if (!argv.dryRun) {
                     const deleteMetadataInstance = await libreplexClient.deleteMetadata(
                         wallet.payer,
                         mintKey,
                         collectionDataSeed,
                         receiverKey,
                     );
                     console.log(stringifyPKsAndBNs(deleteMetadataInstance));
                 } else {
                     console.log('Creating metadata account for mint with pubkey', stringifyPKsAndBNs(mintKey));
                 }
             })



// -------------------------------------------------- PDA account fetching instructions ------------------------------------------



// Fetch all collection data PDAs for a given authority and display their account info
// Pass in authority pubkey or will default to pubkey of keypair path in networkConfig.ts
    .command('fetch-all-collection-data', 'Fetch all collection data PDA accounts info', {
        authorityPubkey: {
            alias: 'a',
            type: 'string',
            demandOption: false,
            description: 'collection data authority pubkey'
        }
    },
             async (argv) => {
                 const rpcConn = new Connection(networkConfig.clusterApiUrl, { confirmTransactionInitialTimeout: 91000 });
                 const wallet: anchor.Wallet = new anchor.Wallet(await loadWallet(networkConfig.signerKeypair));
                 const libreplexClient: LibreplexClient = new LibreplexClient(
                     rpcConn,
                     wallet,
                     LibreplexIDL,
                     METADATA_PROGRAM_ID,
                 );

                 if (argv.managerPubkey) {

                     const authorityKey: PublicKey = new PublicKey(argv.authorityPubkey);

                     if (!argv.dryRun) {
                         console.log('Fetching all collection data PDAs for authority with pubkey:', authorityKey.toBase58());
                         const collectionDataPDAs = await libreplexClient.fetchAllCollectionDataPDAs(authorityKey);

                         // Loop over all PDAs and display account info
                         for (let num = 1; num <= collectionDataPDAs.length; num++) {
                             console.log('Collection data account', num, ':');
                             console.dir(stringifyPKsAndBNs(collectionDataPDAs[num - 1]), {depth: null});
                         }

                     } else {
                         console.log('Found a total of n collection data PDAs for manager pubkey:', authorityKey.toBase58());
                     }
                 } else {
                     if (!argv.dryRun) {
                         console.log('Fetching all collection data PDAs');
                         const collectionDataPDAs = await libreplexClient.fetchAllCollectionDataPDAs();

                         // Loop over all PDAs and display account info
                         for (let num = 1; num <= collectionDataPDAs.length; num++) {
                             console.log('Collection data account', num, ':');
                             console.dir(stringifyPKsAndBNs(collectionDataPDAs[num - 1]), {depth: null});
                         }

                     } else {
                         console.log('Found a total of n collection data PDAs');
                     }
                 }
             })



// Fetch collection data PDA by Pubkey
// Collection Data account pubkey required in command
    .command('fetch-collection-data-by-key', 'Fetch forum PDA account info by pubkey', {
        collectionDataPubkey: {
            alias: 'k',
            type: 'string',
            demandOption: true,
            description: 'collection data account pubkey'
        }
    },
             async (argv) => {
                 const rpcConn = new Connection(networkConfig.clusterApiUrl, { confirmTransactionInitialTimeout: 91000 });
                 const wallet: anchor.Wallet = new anchor.Wallet(await loadWallet(networkConfig.signerKeypair));
                 const libreplexClient: LibreplexClient = new LibreplexClient(
                     rpcConn,
                     wallet,
                     LibreplexIDL,
                     METADATA_PROGRAM_ID,
                 );

                 const collectionDataKey: PublicKey = new PublicKey(argv.collectionDataPubkey);

                 if (!argv.dryRun) {

                     const collectionDataPDA = await libreplexClient.fetchCollectionDataAccount(collectionDataKey);

                     console.log('Displaying account info for collection data with pubkey: ', collectionDataKey.toBase58());
                     console.dir(stringifyPKsAndBNs(collectionDataPDA), {depth: null});

                 } else {
                     console.log('Found collection data PDA for pubkey:', collectionDataKey.toBase58());
                 }
             })



// ------------------------------------------------ misc ----------------------------------------------------------
    .usage('Usage: $0 [-d] -c [config_file] <command> <options>')
    .help();



async function loadWallet(fileName: string): Promise<Keypair> {
    let walletBytes = JSON.parse((await fs.readFile(fileName)).toString());
    let privKeyBytes = walletBytes.slice(0,32);
    let keypair = Keypair.fromSeed(Uint8Array.from(privKeyBytes));
    return keypair
}



// Let's go!
(async() => {
    await parser.argv;
    process.exit();
})();
