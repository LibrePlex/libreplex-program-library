import * as anchor from '@coral-xyz/anchor';
import { AnchorProvider, Idl, Program } from '@coral-xyz/anchor';
import { Connection, Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { AccountUtils, isKp} from '../prog-common';
import { Libreplex } from '../types/libreplex';
import { findCollectionDataPDA, findMetadataPDA } from './metadata.pda';

export interface RoyaltyShare {
    royaltyAddress: PublicKey;
    royaltyShare: number;
}

export interface NftCollectionData {
    royaltyBps: number;
    royaltyShares: RoyaltyShare[];
    permittedSigners: PublicKey[];
}

export interface CollectionDataInput {
    name: string;
    symbol: string;
    collectionUrl: string;
    nftCollectionData: NftCollectionData | null;
}

export interface Attribute {
    traitType: string;
    attribute: string;
}

export interface NftMetadata {
    attributes: Attribute[];
    signers: PublicKey[];
}

export interface MetadataInput {
    name: string;
    symbol: string;
    metadataUrl: string;
    nftMetadata: NftMetadata | null;
}

export class LibreplexClient extends AccountUtils {
    wallet: anchor.Wallet;
    provider!: anchor.Provider;
    libreplexProgram!: anchor.Program<Libreplex>;

    constructor(
        conn: Connection,
        wallet: anchor.Wallet,
        idl?: Idl,
        programId?: PublicKey
    ) {
        super(conn);
        this.wallet = wallet;
        this.setProvider();
        this.setLibreplexProgram(idl, programId);
    }

    setProvider() {
        this.provider = new AnchorProvider(
            this.conn,
            this.wallet,
            AnchorProvider.defaultOptions()
        );
        anchor.setProvider(this.provider);
    }

    setLibreplexProgram(idl?: Idl, programId?: PublicKey) {
        //instantiating program depends on the environment
        if (idl && programId) {
            //means running in prod
            this.libreplexProgram = new anchor.Program<Libreplex>(
                idl as any,
                programId,
                this.provider
            );
        } else {
            //means running inside test suite
            this.libreplexProgram = anchor.workspace.LibrePlex as Program<Libreplex>;
        }
    }

    // -------------------------------------------------------- fetch deserialized accounts

    async fetchCollectionDataAccount(collectionData: PublicKey) {
        return this.libreplexProgram.account.collectionData.fetch(collectionData);
    }

    async fetchMetadataAccount(metadata: PublicKey) {
        return this.libreplexProgram.account.metadata.fetch(metadata);
    }

    // -------------------------------------------------------- get all PDAs by type

    async fetchAllCollectionDataPDAs(authority?: PublicKey) {
        const filter = authority
            ? [
                {
                    memcmp: {
                        offset: 8, // need to prepend 8 bytes for anchor's disc
                        bytes: authority.toBase58(),
                    },
                },
            ]
            : [];
        const pdas = await this.libreplexProgram.account.collectionData.all(filter);
        if (authority) {
            console.log('Found a total of', pdas.length, 'collection data PDAs for authority with address', authority.toBase58());
        }
        else {
            console.log('Found a total of', pdas.length, 'collection data PDAs');
        }
        return pdas;
    }

    async fetchAllMetadataPDAs(collectionData: PublicKey) {
        const filter = collectionData
            ? [
                {
                    memcmp: {
                        offset: 8, // need to prepend 8 bytes for anchor's disc
                        bytes: collectionData.toBase58(),
                    },
                },
            ]
            : [];
        const pdas = await this.libreplexProgram.account.metadata.all(filter);
        if (collectionData) {
            console.log('Found a total of', pdas.length, 'metadata PDAs for collection data with address', collectionData.toBase58());
        }
        else {
            console.log('Found a total of', pdas.length, 'metadata PDAs');
        }
        return pdas;
    }

    // -------------------------------------------------------- execute ixs

    async createCollectionData(
        authority: PublicKey | Keypair,
        collectionDataInput: CollectionDataInput | null,
    ) {
        const collectionDataSeedKeypair = Keypair.generate();
        const collectionDataSeed: PublicKey = collectionDataSeedKeypair.publicKey;

        // Derive PDAs
        const [collectionData, collectionDataBump] = await findCollectionDataPDA(collectionDataSeed);

        // Create Signers Array
        const signers = [];
        if (isKp(authority)) signers.push(<Keypair>authority);

        console.log('creating collection data with pubkey:' , collectionData.toBase58());

        // Transaction
        const txSig = await this.libreplexProgram.methods
            .createCollectionData(
                collectionDataInput
            )
            .accounts({
                authority: isKp(authority) ? (<Keypair>authority).publicKey : <PublicKey>authority,
                collectionData: collectionData,
                collectionSeed: collectionDataSeed,
                systemProgram: SystemProgram.programId
            })
            .signers(signers)
            .rpc();

        return {
            collectionData,
            collectionDataBump,
            txSig
        }
    }

    async deleteCollectionData(
        authority: PublicKey | Keypair,
        collectionDataSeed: PublicKey,
        receiver: PublicKey
    ) {
        // Derive PDAs
        const [collectionData, collectionDataBump] = await findCollectionDataPDA(collectionDataSeed);

        // Create Signers Array
        const signers = [];
        if (isKp(authority)) signers.push(<Keypair>authority);

        console.log('deleting collection data with pubkey:', collectionData.toBase58());

        // Transaction
        const txSig = await this.libreplexProgram.methods
            .deleteCollectionData(
                collectionDataBump
            )
            .accounts({
                authority: isKp(authority) ? (<Keypair>authority).publicKey : <PublicKey>authority,
                collectionData: collectionData,
                collectionSeed: collectionDataSeed,
                receiver: receiver,
                systemProgram: SystemProgram.programId
            })
            .signers(signers)
            .rpc();

        return {
            collectionData,
            collectionDataBump,
            txSig
        }
    }

    async createMetadata(
        authority: PublicKey | Keypair,
        mint: PublicKey | Keypair,
        collectionDataSeed: PublicKey,
        metadataInput: MetadataInput,
    ) {
        const mintKey: PublicKey = isKp(mint) ? (<Keypair>mint).publicKey : <PublicKey>mint;

        // Derive PDAs
        const [collectionData, collectionDataBump] = await findCollectionDataPDA(collectionDataSeed);
        const [metadata, metadataBump] = await findMetadataPDA(mintKey);

        // Create Signers Array
        const signers = [];
        if (isKp(authority)) signers.push(<Keypair>authority);
        if (isKp(mint)) signers.push(<Keypair>mint);

        console.log('creating metadata with mint with pubkey:' , mintKey.toBase58());

        // Transaction
        const txSig = await this.libreplexProgram.methods
            .createMetadata(
                metadataInput,
                collectionDataBump,
            )
            .accounts({
                authority: isKp(authority) ? (<Keypair>authority).publicKey : <PublicKey>authority,
                collectionData: collectionData,
                collectionSeed: collectionDataSeed,
                metadata: metadata,
                mint: isKp(mint) ? (<Keypair>mint).publicKey : <PublicKey>mint,
                systemProgram: SystemProgram.programId
            })
            .signers(signers)
            .rpc();

        return {
            collectionData,
            collectionDataBump,
            metadata,
            metadataBump,
            txSig
        }
    }

    async deleteMetadata(
        authority: PublicKey | Keypair,
        mint: PublicKey,
        collectionDataSeed: PublicKey,
        receiver: PublicKey,
    ) {
        // Derive PDAs
        const [collectionData, collectionDataBump] = await findCollectionDataPDA(collectionDataSeed);
        const [metadata, metadataBump] = await findMetadataPDA(mint);

        // Create Signers Array
        const signers = [];
        if (isKp(authority)) signers.push(<Keypair>authority);

        console.log('creating metadata with mint with pubkey:' , mint.toBase58());

        // Transaction
        const txSig = await this.libreplexProgram.methods
            .deleteMetadata(
                collectionDataBump,
                metadataBump
            )
            .accounts({
                authority: isKp(authority) ? (<Keypair>authority).publicKey : <PublicKey>authority,
                collectionData: collectionData,
                collectionSeed: collectionDataSeed,
                metadata: metadata,
                mint: mint,
                receiver: receiver,
                systemProgram: SystemProgram.programId
            })
            .signers(signers)
            .rpc();

        return {
            collectionData,
            collectionDataBump,
            metadata,
            metadataBump,
            txSig
        }
    }

}
