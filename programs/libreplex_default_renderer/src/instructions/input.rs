use anchor_lang::prelude::*;



#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct RenderInput {
    
}

#[derive(Accounts)]
pub struct RenderContext<'info> {
    /// CHECK: Can be empty
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: Can be empty
    pub mint: UncheckedAccount<'info>,

    /// CHECK: Can be empty
    pub group: UncheckedAccount<'info>,

    /// CHECK: Can be empty
    #[account(seeds = [mint.key.as_ref()], bump)]
    pub render_state: UncheckedAccount<'info>,
    
    /// CHECK: Can be empty
    pub output_account: UncheckedAccount<'info>,
}