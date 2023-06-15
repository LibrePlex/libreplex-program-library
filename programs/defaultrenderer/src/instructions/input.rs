use anchor_lang::prelude::*;



#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct RenderInput {
    
}

#[derive(Accounts)]
pub struct RenderContext<'info> {

    #[account()]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: Can be empty
    #[account()]
    pub metadata_extension: UncheckedAccount<'info>,

    /// CHECK: Can be empty
    #[account()]
    pub group: UncheckedAccount<'info>,

}