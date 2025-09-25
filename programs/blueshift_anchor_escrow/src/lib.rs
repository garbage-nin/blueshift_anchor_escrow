#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub use instructions::*;
declare_id!("3WUvx61qwcLZM46TFLH1Gqn7CYQVC9PYbve3D7X42XzM");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        instructions::make::handler(ctx, seed, receive, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
