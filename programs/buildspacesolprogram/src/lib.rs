use anchor_lang::prelude::*;

declare_id!("2KUJfwMchPJYaT2yPrDmETtq1d77rPHsEX3rb7Je3DS9");

#[program]
pub mod buildspacesolprogram {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // Get a reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        // Get a reference to the account and incremental total_gifs
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct
        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            score: 0,
        };

        // Add the item to the gif_list vector
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn gif_vote(
        ctx: Context<GifVote>,
        gif_item_index: u32,
        gif_item_action: bool,
    ) -> ProgramResult {
        //
        let base_account = &mut ctx.accounts.base_account;
        let _gif_item_index_usize: usize = gif_item_index.to_string().parse().unwrap();
        let gif_item = &mut base_account.gif_list[_gif_item_index_usize];
        // gif_item_action == true 👍
        if gif_item_action {
            gif_item.score += 1;
        // gif_item_action == false 👎
        } else {
            gif_item.score -= 1;
        }
        Ok(())
    }

}

// STRUCTS

// Attach certain variables to the StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Specify what data to store in the AddGif Context
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

//
#[derive(Accounts)]
pub struct GifVote<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Custom Item Struct
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub score: i64,
}

// Tell Solana what we want to store on this account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type: ItemStruct to the account
    pub gif_list: Vec<ItemStruct>,
}
