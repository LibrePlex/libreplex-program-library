import { PublicKey } from '@solana/web3.js';
import { METADATA_PROGRAM_ID } from '../index';

export const findCollectionDataPDA = async (collection_seed: PublicKey) => {
    return PublicKey.findProgramAddressSync(
        [Buffer.from('collection_data'), collection_seed.toBytes()],
        METADATA_PROGRAM_ID
    );
};

export const findMetadataPDA = async (mint: PublicKey) => {
    return PublicKey.findProgramAddressSync(
        [Buffer.from('metadata'), mint.toBytes()],
        METADATA_PROGRAM_ID
    );
};
