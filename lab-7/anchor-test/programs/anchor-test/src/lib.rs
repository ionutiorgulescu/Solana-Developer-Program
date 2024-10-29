use anchor_lang::prelude::*;

declare_id!("CuNLHRXTbKDQHfHxkgZCmEWMwqeHwEotbRZG5yKpyQmv");

#[program]
pub mod anchor_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
