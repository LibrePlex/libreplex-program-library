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

pub const MAX_CONTROL_TYPE_SIZE: usize = 150;

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
    pub program_id: Pubkey,
    pub instruction_data: Vec<u8>,
    pub remaining_accounts_to_use: u32,
}


impl Control for CustomProgram {

    
    fn after_mint(&self,  accounts: &mut Accounts, _arg_ctx: &mut ArgCtx) -> Result<()> {
        msg!("CustomProgram control");
        let remaining_accounts = accounts.remaining_accounts.accounts;

        let custom_program_account = remaining_accounts.get(accounts.remaining_accounts.current as usize).unwrap();
        accounts.remaining_accounts.current += 1;
        let current_remaining_accounts_pointer = accounts.remaining_accounts.current as usize;



        let target_remaining_accounts = &remaining_accounts[current_remaining_accounts_pointer..current_remaining_accounts_pointer + self.remaining_accounts_to_use as usize];

    
        let mut account_metas = vec![
            accounts.receiver.to_account_metas(None).pop().unwrap(), 
            accounts.mint.to_account_metas(None).pop().unwrap(), 
            accounts.metadata.to_account_metas(None).pop().unwrap(), 
            accounts.group.to_account_metas(None).pop().unwrap(), 
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
        accounts.group.to_account_info(), accounts.system_program.to_account_info(), custom_program_account.to_account_info()];

        infos.extend(target_remaining_accounts.iter().map(|acc| acc.to_account_info()));


        invoke(&ix, 
            infos.as_slice())?;

        accounts.remaining_accounts.current += self.remaining_accounts_to_use;

        Ok(())
    }
}
