use anchor_lang::prelude::*; //导入Anchor框架的预导入模块

declare_id!(""); //声明程序的ID，被导入到链上时的唯一标识符，空字符串表示自动生成，Anchorbuild会
               //自动生成

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; //定义8字节的公共常量,用于账户数据的鉴别器。

#[program] //标记Solana程序的主模块，识别程序入口
pub mod favorites{

    use super::*;  //导入父模块的所有内容 其中::是路径分隔符，super指的是父模块

    pub fn set_favorites(//指令函数，用于接收上下文和用户数据

        context: Context<SetFavorites>,
        number: u64, 
        color: String, 
        hobbies: Vec<String>,

        ) -> Result<()>{
        msg!("Greetings from {}", context.program_id);
        let user_public_key = context.accounts.user.key();

        msg!(
            "User {}'s favorite number is {}, favorite color is {}, and their hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
            );

        context.accounts.favorites.set_inner( Favorites{
            number,
            color,
            hobbies,
        }); //set_inner将Favorites结构体数据写入账户

        Ok(())
    }

}

#[account] //属性标记，用于指导4编译，标记结构体为Anchor账户类型
#[derive(InitSpace)] //自动计算账户初始化所需空间
pub struct Favorites{

    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5,50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info>{

    #[account(mut)]    
    pub user: Signer<'info>,   //user：交易的签名者，标记为可变的mut，需要支付租金

    #[account(
        init_if_needed,  //账户不存在即初始化
        payer = user,    //租金由用户支付
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,  //账户空间等于鉴别器＋fa结构体所需空间
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,    //存储用户的PDA，程序派生地址

    pub system_program: Program<'info, System>,