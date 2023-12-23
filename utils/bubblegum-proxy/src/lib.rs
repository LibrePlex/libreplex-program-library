use anchor_lang::{prelude::*, solana_program::keccak};

anchor_gen::generate_cpi_crate!("bubblegum.json");

declare_id!("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY");



pub fn hash_metadata(metadata: &MetadataArgs) -> Result<[u8; 32]> {
    let metadata_args_hash = keccak::hashv(&[metadata.try_to_vec()?.as_slice()]);
    // Calculate new data hash.
    Ok(keccak::hashv(&[
        &metadata_args_hash.to_bytes(),
        &metadata.seller_fee_basis_points.to_le_bytes(),
    ])
    .to_bytes())
}
