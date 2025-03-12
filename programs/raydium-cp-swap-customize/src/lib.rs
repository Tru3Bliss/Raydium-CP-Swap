use anchor_lang::prelude::*;

declare_id!("F4tHwxNDizBtYz4s5rSukS32RY4VA1AYSHjBmywRCcTp");

#[program]
pub mod raydium_cp_swap_customize {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
