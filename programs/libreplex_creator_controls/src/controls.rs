use anchor_lang::{prelude::*, system_program};
use arrayref::array_ref;
use solana_program::instruction::Instruction;
use solana_program::program::invoke;
use solana_program::keccak;
use crate::errors::ErrorCode;
use crate::state::{Accounts, ArgCtx};


pub trait Control {
    fn before_mint(&self,  _accounts: &mut Accounts, _arg_ctx: &mut ArgCtx) -> Result<()> {
        Ok(())
    }

    fn after_mint(&self,  _accounts: &mut Accounts, _arg_ctx: &mut ArgCtx) -> Result<()> {
        msg!("Default Post Mint");
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ControlType {
    AllowList(AllowList),
    Payment(Payment),
    SplPayment(SplPayment),
    MintLimit(MintLimit),
    CustomProgram(CustomProgram),
}

impl ControlType {
    pub fn get_size(&self) -> usize {
        1 + match &self {
            ControlType::AllowList(al) => {
                4 + al.label.len() + 32
            },
            ControlType::Payment(_) => {
                32 + 8
            },
            ControlType::SplPayment(_) => {
                8 + 32 + 32 + 32
            },
            ControlType::MintLimit(limit) => {
                4 + 4 + limit.account_key.len() * 32 + 1
            },
            ControlType::CustomProgram(custom_program) => {
                custom_program.get_size()
            },
        }
    }
}

pub const MAX_CONTROL_TYPE_SIZE: usize = 200;

impl Control for ControlType {
    fn before_mint(&self, accounts: &mut Accounts, arg_ctx: &mut ArgCtx) -> Result<()> {
        match self {
            ControlType::AllowList(allow_list) => allow_list.before_mint(accounts, arg_ctx),
            ControlType::Payment(payment) => payment.before_mint(accounts, arg_ctx),
            ControlType::SplPayment(spl_payment) => spl_payment.before_mint(accounts, arg_ctx),
            ControlType::MintLimit(mint_limit) => mint_limit.before_mint(accounts, arg_ctx),
            ControlType::CustomProgram(custom_program) => custom_program.before_mint(accounts, arg_ctx)
        }
    }

    fn after_mint(&self,  accounts: &mut Accounts, arg_ctx: &mut ArgCtx) -> Result<()> {
        match self {
            ControlType::AllowList(allow_list) => allow_list.after_mint(accounts, arg_ctx),
            ControlType::Payment(payment) => payment.after_mint(accounts, arg_ctx),
            ControlType::SplPayment(spl_payment) => spl_payment.after_mint(accounts, arg_ctx),
            ControlType::MintLimit(mint_limit) => mint_limit.after_mint(accounts, arg_ctx),
            ControlType::CustomProgram(custom_program) => custom_program.after_mint(accounts, arg_ctx)
        }
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AllowList {
    pub label: String,
    pub root: [u8; 32],
}


impl AllowList {
    pub fn verify(proof: &[u8], root: [u8; 32], leaf: [u8; 32]) -> Result<()> {
        if proof.len() % 32 != 0 {
            return Err(ErrorCode::InvalidProof.into())
        }


        let hash = proof.chunks(32).fold(leaf, |hash, proof_element| {
            if &hash[..] <= proof_element {
                keccak::hashv(&[&hash, proof_element]).0
            } else {
                keccak::hashv(&[proof_element, &hash]).0
            }
        });

        if hash != root { Err(ErrorCode::InvalidProof.into()) } else {Ok(())}
    }
}

impl Control for AllowList {
    fn before_mint(&self, accounts: &mut Accounts, arg_ctx: &mut ArgCtx) -> Result<()> {
        let current_arg 
            = arg_ctx.args.get(arg_ctx.current as usize).ok_or(ErrorCode::MissingArgument)?;

        arg_ctx.current += 1;

        let proof = current_arg.as_slice();
        let leaf = keccak::hash(&accounts.receiver.key.to_bytes()).to_bytes();
        let root = self.root;

        Self::verify(proof, root, leaf)
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Payment {
    pub amount: u64,
    pub recepient: Pubkey,
}

impl Control for Payment {
    fn before_mint(&self, accounts: &mut Accounts, _arg_ctx: &mut ArgCtx) -> Result<()> {
        let mint_funds_recepient = accounts.remaining_accounts.accounts
                                                            .get(accounts.remaining_accounts.current as usize)
                                                            .ok_or(ErrorCode::MissingAccount)?;
        accounts.remaining_accounts.current += 1;


        if &self.recepient != mint_funds_recepient.key {
            return Err(ErrorCode::InvalidMintFundsRecepient.into());
        }

        system_program::transfer(CpiContext::new(accounts.system_program.to_account_info(), system_program::Transfer {
            from: accounts.payer.to_account_info(),
            to: mint_funds_recepient.to_account_info()
        }), self.amount)
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MintLimit {
    pub limit: u32,
    pub account_key: Vec<Pubkey>,
    pub scoped_to_buyer: bool,
}

impl MintLimit {
    pub const MAX_ACCOUNT_KEY_SIZE: usize = 40;
}


impl Control for MintLimit {
    fn before_mint(&self,  accounts: &mut Accounts, _arg_ctx: &mut ArgCtx) -> Result<()> {
        let total_mints_account = accounts.remaining_accounts.accounts
            .get(accounts.remaining_accounts.current as usize)
            .ok_or(ErrorCode::MissingAccount)?;

        accounts.remaining_accounts.current += 1;

        let mut account_key_bytes: Vec<&[u8]> = self.account_key.iter().map(|c| {c.as_ref()}).collect();

        let mut expected_seeds = if self.scoped_to_buyer {
            vec!["mint_limit".as_bytes(), accounts.receiver.key.as_ref()]
        } else {
            vec!["mint_limit".as_bytes()]
        };

        expected_seeds.append(&mut account_key_bytes);

        let expected_key = Pubkey::find_program_address(&expected_seeds, &crate::id());

        if total_mints_account.key != &expected_key.0 {
            return Err(ErrorCode::InvalidTotalMintsAccount.into());
        }

        if total_mints_account.lamports() == 0 {
            let rent = Rent::get()?;

            let bump = [expected_key.1];
            expected_seeds.push(&bump);
       
            let total_mints_signer_seeds = expected_seeds.as_slice();

            anchor_lang::system_program::create_account(CpiContext::new_with_signer(accounts.system_program.to_account_info(), anchor_lang::system_program::CreateAccount {
                from: accounts.payer.to_account_info(),
                to: total_mints_account.to_account_info(),
            }, &[total_mints_signer_seeds]), rent.minimum_balance(4), 4, &crate::id())?;
        }

        let mut total_mints_data = total_mints_account.data.borrow_mut();
        let current_total_mints = u32::from_le_bytes(*array_ref![total_mints_data, 0, 4]);

        if current_total_mints > self.limit {
            return Err(ErrorCode::MintLimitExceeded.into());
        }

        let new_total_mints = (current_total_mints + 1).to_le_bytes();
        total_mints_data[0..4].copy_from_slice(&new_total_mints);

        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SplPayment {
    pub amount: u64,
    pub mint: Pubkey,
    pub recepient: Pubkey,
    pub token_program: Pubkey,
}

impl Control for SplPayment {
    fn before_mint(&self, accounts: &mut Accounts, _arg_ctx: &mut ArgCtx) -> Result<()> {
        let token_recepient = accounts.remaining_accounts.accounts
            .get(accounts.remaining_accounts.current as usize)
            .ok_or(ErrorCode::MissingAccount)?;

        accounts.remaining_accounts.current += 1;

        let buyer_token_wallet = accounts.remaining_accounts.accounts
            .get(accounts.remaining_accounts.current as usize)
            .ok_or(ErrorCode::MissingAccount)?;

        accounts.remaining_accounts.current += 1;

        let token_program = accounts.remaining_accounts.accounts
        .get(accounts.remaining_accounts.current as usize)
        .ok_or(ErrorCode::MissingAccount)?;

        accounts.remaining_accounts.current += 1;

        if token_program.key != &self.token_program {
            return Err(ErrorCode::InvalidTokenProgram.into());
        }

        if &self.recepient != token_recepient.key {
            return Err(ErrorCode::InvalidMintFundsRecepient.into());
        }
        
        // Do we even need to check the mint?
        anchor_spl::token::transfer(CpiContext::new(token_program.to_account_info(), anchor_spl::token::Transfer{
            from: buyer_token_wallet.to_account_info(),
            to: token_recepient.to_account_info(),
            authority: accounts.payer.to_account_info(),
        }), self.amount)
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CustomProgram {
    pub label: String,
    pub program_id: Pubkey,
    pub instruction_data: Vec<u8>,
    pub remaining_account_metas: Vec<CustomProgramAccountMeta>,
}

impl CustomProgram {
    fn get_size(&self) -> usize {
        4 + self.label.len() + 
        32 + 
        4 + self.instruction_data.len() + 
        4 + self.remaining_account_metas.iter().fold(0, |current, meta| {
            current + meta.get_size()
        })
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub enum Seed {
    Bytes(Vec<u8>),
    MintPlaceHolder,
    ReceiverPlaceHolder,
    PayerPlaceHolder,
}

impl Seed {
    fn get_size(&self) -> usize {
        1 + match &self {
            Seed::Bytes(bytes) => 4 + bytes.len(),
            _ => 0,
        }
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct KeySeedDerivation {
    program_id: Pubkey,
    seeds: Vec<Seed>,
}

impl KeySeedDerivation {
    fn get_size(&self) -> usize {
        32 + 4 + self.seeds.iter().fold(0, |total, seed|{
            total + seed.get_size()
        })
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub enum CustomProgramAcountMetaKey {
    Pubkey(Pubkey),
    DerivedFromSeeds(KeySeedDerivation),
}

impl CustomProgramAcountMetaKey {
    fn get_size(&self) -> usize {
        1 + match &self {
            CustomProgramAcountMetaKey::Pubkey(_) => 32,
            CustomProgramAcountMetaKey::DerivedFromSeeds(seed_derivation) => seed_derivation.get_size(),
        }
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct CustomProgramAccountMeta {
    pub key: CustomProgramAcountMetaKey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl CustomProgramAccountMeta {
    fn get_size(&self) -> usize {
        &self.key.get_size() + 1 + 1
    }
}


impl Control for CustomProgram {

    
    fn after_mint(&self,  accounts: &mut Accounts, _arg_ctx: &mut ArgCtx) -> Result<()> {
        msg!("CustomProgram control");
        let remaining_accounts = accounts.remaining_accounts.accounts;

        let maybe_custom_program_account = remaining_accounts.get(accounts.remaining_accounts.current as usize);

        accounts.remaining_accounts.current += 1;
        let current_remaining_accounts_pointer = accounts.remaining_accounts.current as usize;

        if maybe_custom_program_account.is_none() {
            return Err(ErrorCode::InvalidCustomProgram.into());
        };

        let custom_program_account = maybe_custom_program_account.unwrap();

        if custom_program_account.key != &self.program_id {
            return Err(ErrorCode::InvalidCustomProgram.into());
        }

        let target_remaining_accounts = &remaining_accounts[current_remaining_accounts_pointer..current_remaining_accounts_pointer + self.remaining_account_metas.len()];

        let remaining_accounts_are_invalid = self.remaining_account_metas.iter().enumerate().any(|(index, expected_meta)| {
            let actual_remaining_account = &target_remaining_accounts[index];

            let keys_match = match &expected_meta.key {
                CustomProgramAcountMetaKey::Pubkey(key) => key == actual_remaining_account.key,
                CustomProgramAcountMetaKey::DerivedFromSeeds(seeds) => {
                    let pda_seeds: Vec<&[u8]> = seeds.seeds.iter().map(|seed| {
                        match seed {
                            Seed::Bytes(bytes) => bytes.as_slice(),
                            Seed::MintPlaceHolder => accounts.mint.key.as_ref(),
                            Seed::ReceiverPlaceHolder => accounts.receiver.key.as_ref(),
                            Seed::PayerPlaceHolder => accounts.payer.key.as_ref(),
                        }
                    }).collect();

                    let expected_key = Pubkey::find_program_address(pda_seeds.as_slice(), &seeds.program_id);

                    &expected_key.0 == actual_remaining_account.key
                },
            };

            !(expected_meta.is_signer == actual_remaining_account.is_signer && expected_meta.is_writable == actual_remaining_account.is_writable && keys_match)
        });

        if remaining_accounts_are_invalid {
            return Err(ErrorCode::InvalidRemainingAccountsForCustomProgramControl.into())
        }

        let mut account_metas = vec![
            accounts.receiver.to_account_metas(None).pop().unwrap(), 
            accounts.mint.to_account_metas(None).pop().unwrap(), 
            accounts.metadata.to_account_metas(None).pop().unwrap(), 
            accounts.collection.to_account_metas(None).pop().unwrap(), 
            accounts.system_program.to_account_metas(None).pop().unwrap()];



        account_metas.extend(target_remaining_accounts.iter()
        .map(|acc| match acc.is_writable {
            false => AccountMeta::new_readonly(*acc.key, acc.is_signer),
            true => AccountMeta::new(*acc.key, acc.is_signer),
        }));

        let ix = Instruction {
            accounts: account_metas,
            data: self.instruction_data.clone(),
            program_id: self.program_id,
        };
        
        let mut infos = vec![ 
            accounts.receiver.to_account_info(), 
            accounts.mint.to_account_info(), 
            accounts.metadata.to_account_info(),
            accounts.collection.to_account_info(), 
            accounts.system_program.to_account_info(), 
            custom_program_account.to_account_info()];

        infos.extend(target_remaining_accounts.iter().map(|acc| acc.to_account_info()));


        invoke(&ix, 
            infos.as_slice())?;

        accounts.remaining_accounts.current += self.remaining_account_metas.len() as u32;

        Ok(())
    }
}
