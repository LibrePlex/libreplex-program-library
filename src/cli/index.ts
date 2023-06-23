const dotenv = require('dotenv');
dotenv.config({ path: `.env.${process.env.NODE_ENV}` });
import { Keypair, SystemProgram, sendAndConfirmRawTransaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { Connection, Transaction } from '@solana/web3.js';
import { getProgramInstance } from '../program/getProgramInstance';
import fs from 'fs';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';
import { getGroupPda } from '../pdas/getCollectionPda';

const { program } = require('commander');

program
  .name('libre-cli')
  .description('CLI to some JavaScript string utilities')
  .version('0.1.0');

const group = program.command('group');

group
  .command('create')
  .description('Creates a libre metadata group')
  .argument('<mintId>', 'mintId to migrate')
  .option(
    '-k, --keypair <keypair>',
    'keypair to use, defaults to ~/.config/solana/id.json',
    '~/.config/solana/id.json'
  )
  .requiredOption(
    '-n, --name <name>',
    'name of the group'
  )
  .requiredOption(
    '-s, --symbol <symbol>',
    'symbol of the group'
  )
  .requiredOption(
    '-u, --url <url>',
    'url of the group'
  )
  .requiredOption(
    '-d, --description <description>',
    'description of the group'
  )
   
  .action(async (mintId: string, options: { 
    keypair: string | undefined,
    name: string,
    symbol: string,
    description: string,
    url: string
}) => {
    const keyfile = JSON.parse(fs.readFileSync(options.keypair, 'utf8'));

    const updateAuthKeypair = Keypair.fromSecretKey(new Uint8Array(keyfile));

    console.log({uauth: updateAuthKeypair.publicKey.toBase58()})

    const connection = new Connection(process.env.RPC_ENDPOINT);

    const wallet = new NodeWallet(Keypair.generate());

    const program = getProgramInstance(connection, wallet);

    const seed = Keypair.generate().publicKey;

    const [group] = getGroupPda(seed);

    const {keypair, name, symbol, description, url} = options

    const instruction = await program.methods
      .createGroup({
        name,
        symbol,
        url,
        description,
        templateConfiguration: {none: {}},
        royalties: null,
        attributeTypes: [],
        permittedSigners: []
      })
      .accounts({
        authority: updateAuthKeypair.publicKey,
        group,
        seed,
        systemProgram: SystemProgram.programId,
      })
      .instruction();

    const transaction = new Transaction().add(instruction);
    transaction.feePayer = updateAuthKeypair.publicKey;


    await sendAndConfirmTransaction(connection, transaction, [updateAuthKeypair])
  });

program.parse();
