#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod seahorse_util {
    use super::*;
    use std::{
        collections::HashMap,
        fmt::Debug,
        ops::{Deref, Index, IndexMut},
    };

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    pub trait IndexWrapped {
        type Output;

        fn index_wrapped(&self, index: i128) -> &Self::Output;
    }

    pub trait IndexWrappedMut: IndexWrapped {
        fn index_wrapped_mut(&mut self, index: i128) -> &mut <Self as IndexWrapped>::Output;
    }

    impl<T> IndexWrapped for Vec<T> {
        type Output = T;

        fn index_wrapped(&self, mut index: i128) -> &Self::Output {
            if index < 0 {
                index += self.len() as i128;
            }

            let index: usize = index.try_into().unwrap();

            self.index(index)
        }
    }

    impl<T> IndexWrappedMut for Vec<T> {
        fn index_wrapped_mut(&mut self, mut index: i128) -> &mut <Self as IndexWrapped>::Output {
            if index < 0 {
                index += self.len() as i128;
            }

            let index: usize = index.try_into().unwrap();

            self.index_mut(index)
        }
    }

    impl<T, const N: usize> IndexWrapped for [T; N] {
        type Output = T;

        fn index_wrapped(&self, mut index: i128) -> &Self::Output {
            if index < 0 {
                index += N as i128;
            }

            let index: usize = index.try_into().unwrap();

            self.index(index)
        }
    }

    impl<T, const N: usize> IndexWrappedMut for [T; N] {
        fn index_wrapped_mut(&mut self, mut index: i128) -> &mut <Self as IndexWrapped>::Output {
            if index < 0 {
                index += N as i128;
            }

            let index: usize = index.try_into().unwrap();

            self.index_mut(index)
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! seahorse_const {
        ($ name : ident , $ value : expr) => {
            macro_rules! $name {
                () => {
                    $value
                };
            }

            pub(crate) use $name;
        };
    }

    pub trait Loadable {
        type Loaded;

        fn load(stored: Self) -> Self::Loaded;

        fn store(loaded: Self::Loaded) -> Self;
    }

    macro_rules! Loaded {
        ($ name : ty) => {
            <$name as Loadable>::Loaded
        };
    }

    pub(crate) use Loaded;

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }

    pub(crate) use assign;

    pub(crate) use index_assign;

    pub(crate) use seahorse_const;
}

#[program]
mod rotary_savings {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    # [instruction (new_user : Pubkey)]
    pub struct AddUser<'info> {
        #[account(mut)]
        pub admin: Signer<'info>,
        #[account(mut)]
        pub collection: Box<Account<'info, dot::program::Collection>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: User > () + 8 , payer = admin , seeds = ["user" . as_bytes () . as_ref () , collection . key () . as_ref () , new_user . as_ref ()] , bump)]
        pub user: Box<Account<'info, dot::program::User>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn add_user(ctx: Context<AddUser>, new_user: Pubkey) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let admin = SeahorseSigner {
            account: &ctx.accounts.admin,
            programs: &programs_map,
        };

        let collection =
            dot::program::Collection::load(&mut ctx.accounts.collection, &programs_map);

        let user = Empty {
            account: dot::program::User::load(&mut ctx.accounts.user, &programs_map),
            bump: Some(ctx.bumps.user),
        };

        add_user_handler(admin.clone(), collection.clone(), user.clone(), new_user);

        dot::program::Collection::store(collection);

        dot::program::User::store(user.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (new_duration : i64 , new_period : i64 , new_amount_per_period : u64)]
    pub struct AdjustSettings<'info> {
        #[account(mut)]
        pub admin: Signer<'info>,
        #[account(mut)]
        pub collection: Box<Account<'info, dot::program::Collection>>,
        #[account(mut)]
        pub multisig: Box<Account<'info, dot::program::Multisig>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Proposal > () + 8 , payer = admin , seeds = ["proposal" . as_bytes () . as_ref () , collection . key () . as_ref () , format ! ("{:?}" , multisig . borrow () . nonce) . as_bytes () . as_ref ()] , bump)]
        pub proposal: Box<Account<'info, dot::program::Proposal>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn adjust_settings(
        ctx: Context<AdjustSettings>,
        new_duration: i64,
        new_period: i64,
        new_amount_per_period: u64,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let admin = SeahorseSigner {
            account: &ctx.accounts.admin,
            programs: &programs_map,
        };

        let collection =
            dot::program::Collection::load(&mut ctx.accounts.collection, &programs_map);

        let multisig = dot::program::Multisig::load(&mut ctx.accounts.multisig, &programs_map);
        let proposal = Empty {
            account: dot::program::Proposal::load(&mut ctx.accounts.proposal, &programs_map),
            bump: Some(ctx.bumps.proposal),
        };

        let clock = &ctx.accounts.clock.clone();

        adjust_settings_handler(
            admin.clone(),
            collection.clone(),
            multisig.clone(),
            proposal.clone(),
            clock.clone(),
            new_duration,
            new_period,
            new_amount_per_period,
        );

        dot::program::Collection::store(collection);

        dot::program::Multisig::store(multisig);

        dot::program::Proposal::store(proposal.account);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct CloseCollection<'info> {
        #[account(mut)]
        pub admin: Signer<'info>,
        #[account(mut)]
        pub collection: Box<Account<'info, dot::program::Collection>>,
        #[account(mut)]
        pub multisig: Box<Account<'info, dot::program::Multisig>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Proposal > () + 8 , payer = admin , seeds = ["proposal" . as_bytes () . as_ref () , collection . key () . as_ref () , format ! ("{:?}" , multisig . borrow () . nonce) . as_bytes () . as_ref ()] , bump)]
        pub proposal: Box<Account<'info, dot::program::Proposal>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn close_collection(ctx: Context<CloseCollection>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let admin = SeahorseSigner {
            account: &ctx.accounts.admin,
            programs: &programs_map,
        };

        let collection =
            dot::program::Collection::load(&mut ctx.accounts.collection, &programs_map);

        let multisig = dot::program::Multisig::load(&mut ctx.accounts.multisig, &programs_map);
        let proposal = Empty {
            account: dot::program::Proposal::load(&mut ctx.accounts.proposal, &programs_map),
            bump: Some(ctx.bumps.proposal),
        };

        let clock = &ctx.accounts.clock.clone();

        close_collection_handler(
            admin.clone(),
            collection.clone(),
            multisig.clone(),
            proposal.clone(),
            clock.clone(),
        );

        dot::program::Collection::store(collection);

        dot::program::Multisig::store(multisig);

        dot::program::Proposal::store(proposal.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (name : String , duration : i64 , period : i64 , amount_per_period : u64 , total_members : u8 , early_withdrawal_penalty_rate : u8)]
    pub struct CreateCollection<'info> {
        #[account(mut)]
        pub admin: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Collection > () + 8 , payer = admin , seeds = ["collection" . as_bytes () . as_ref () , admin . key () . as_ref () , name . as_bytes () . as_ref ()] , bump)]
        pub collection: Box<Account<'info, dot::program::Collection>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        name: String,
        duration: i64,
        period: i64,
        amount_per_period: u64,
        total_members: u8,
        early_withdrawal_penalty_rate: u8,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let admin = SeahorseSigner {
            account: &ctx.accounts.admin,
            programs: &programs_map,
        };

        let collection = Empty {
            account: dot::program::Collection::load(&mut ctx.accounts.collection, &programs_map),
            bump: Some(ctx.bumps.collection),
        };

        create_collection_handler(
            admin.clone(),
            collection.clone(),
            name,
            duration,
            period,
            amount_per_period,
            total_members,
            early_withdrawal_penalty_rate,
        );

        dot::program::Collection::store(collection.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (signers : Vec < Pubkey > , threshold : u8)]
    pub struct CreateMultisig<'info> {
        #[account(mut)]
        pub admin: Signer<'info>,
        #[account(mut)]
        pub collection: Box<Account<'info, dot::program::Collection>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Multisig > () + 8 , payer = admin , seeds = ["multisig" . as_bytes () . as_ref () , collection . key () . as_ref ()] , bump)]
        pub multisig: Box<Account<'info, dot::program::Multisig>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        signers: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let admin = SeahorseSigner {
            account: &ctx.accounts.admin,
            programs: &programs_map,
        };

        let collection =
            dot::program::Collection::load(&mut ctx.accounts.collection, &programs_map);

        let multisig = Empty {
            account: dot::program::Multisig::load(&mut ctx.accounts.multisig, &programs_map),
            bump: Some(ctx.bumps.multisig),
        };

        create_multisig_handler(
            admin.clone(),
            collection.clone(),
            multisig.clone(),
            signers,
            threshold,
        );

        dot::program::Collection::store(collection);

        dot::program::Multisig::store(multisig.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (proposal_type : u8 , withdraw_user : Pubkey , withdraw_amount : u64 , new_duration : i64 , new_period : i64 , new_amount_per_period : u64 , new_early_withdrawal_penalty_rate : u8)]
    pub struct CreateProposal<'info> {
        #[account(mut)]
        pub proposer: Signer<'info>,
        #[account(mut)]
        pub collection: Box<Account<'info, dot::program::Collection>>,
        #[account(mut)]
        pub multisig: Box<Account<'info, dot::program::Multisig>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Proposal > () + 8 , payer = proposer , seeds = ["proposal" . as_bytes () . as_ref () , collection . key () . as_ref () , format ! ("{:?}" , multisig . borrow () . nonce) . as_bytes () . as_ref ()] , bump)]
        pub proposal: Box<Account<'info, dot::program::Proposal>>,
        #[account(mut)]
        pub user_account: Box<Account<'info, dot::program::User>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal_type: u8,
        withdraw_user: Pubkey,
        withdraw_amount: u64,
        new_duration: i64,
        new_period: i64,
        new_amount_per_period: u64,
        new_early_withdrawal_penalty_rate: u8,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let proposer = SeahorseSigner {
            account: &ctx.accounts.proposer,
            programs: &programs_map,
        };

        let collection =
            dot::program::Collection::load(&mut ctx.accounts.collection, &programs_map);

        let multisig = dot::program::Multisig::load(&mut ctx.accounts.multisig, &programs_map);
        let proposal = Empty {
            account: dot::program::Proposal::load(&mut ctx.accounts.proposal, &programs_map),
            bump: Some(ctx.bumps.proposal),
        };

        let user_account = dot::program::User::load(&mut ctx.accounts.user_account, &programs_map);
        let clock = &ctx.accounts.clock.clone();

        create_proposal_handler(
            proposer.clone(),
            collection.clone(),
            multisig.clone(),
            proposal.clone(),
            user_account.clone(),
            clock.clone(),
            proposal_type,
            withdraw_user,
            withdraw_amount,
            new_duration,
            new_period,
            new_amount_per_period,
            new_early_withdrawal_penalty_rate,
        );

        dot::program::Collection::store(collection);

        dot::program::Multisig::store(multisig);

        dot::program::Proposal::store(proposal.account);

        dot::program::User::store(user_account);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct ExecuteProposal<'info> {
        #[account(mut)]
        pub collection: Box<Account<'info, dot::program::Collection>>,
        #[account(mut)]
        pub multisig: Box<Account<'info, dot::program::Multisig>>,
        #[account(mut)]
        pub proposal: Box<Account<'info, dot::program::Proposal>>,
        #[account(mut)]
        pub user_account: Box<Account<'info, dot::program::User>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let collection =
            dot::program::Collection::load(&mut ctx.accounts.collection, &programs_map);

        let multisig = dot::program::Multisig::load(&mut ctx.accounts.multisig, &programs_map);
        let proposal = dot::program::Proposal::load(&mut ctx.accounts.proposal, &programs_map);
        let user_account = dot::program::User::load(&mut ctx.accounts.user_account, &programs_map);
        let clock = &ctx.accounts.clock.clone();

        execute_proposal_handler(
            collection.clone(),
            multisig.clone(),
            proposal.clone(),
            user_account.clone(),
            clock.clone(),
        );

        dot::program::Collection::store(collection);

        dot::program::Multisig::store(multisig);

        dot::program::Proposal::store(proposal);

        dot::program::User::store(user_account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (amount : u64)]
    pub struct Pay<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(mut)]
        pub collection: Box<Account<'info, dot::program::Collection>>,
        #[account(mut)]
        pub user_account: Box<Account<'info, dot::program::User>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
    }

    pub fn pay(ctx: Context<Pay>, amount: u64) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let user = SeahorseSigner {
            account: &ctx.accounts.user,
            programs: &programs_map,
        };

        let collection =
            dot::program::Collection::load(&mut ctx.accounts.collection, &programs_map);

        let user_account = dot::program::User::load(&mut ctx.accounts.user_account, &programs_map);
        let clock = &ctx.accounts.clock.clone();

        pay_handler(
            user.clone(),
            collection.clone(),
            user_account.clone(),
            clock.clone(),
            amount,
        );

        dot::program::Collection::store(collection);

        dot::program::User::store(user_account);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct ProposeWithdraw<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(mut)]
        pub collection: Box<Account<'info, dot::program::Collection>>,
        #[account(mut)]
        pub user_account: Box<Account<'info, dot::program::User>>,
        #[account(mut)]
        pub multisig: Box<Account<'info, dot::program::Multisig>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Proposal > () + 8 , payer = user , seeds = ["proposal" . as_bytes () . as_ref () , collection . key () . as_ref () , format ! ("{:?}" , multisig . borrow () . nonce) . as_bytes () . as_ref ()] , bump)]
        pub proposal: Box<Account<'info, dot::program::Proposal>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn propose_withdraw(ctx: Context<ProposeWithdraw>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let user = SeahorseSigner {
            account: &ctx.accounts.user,
            programs: &programs_map,
        };

        let collection =
            dot::program::Collection::load(&mut ctx.accounts.collection, &programs_map);

        let user_account = dot::program::User::load(&mut ctx.accounts.user_account, &programs_map);
        let multisig = dot::program::Multisig::load(&mut ctx.accounts.multisig, &programs_map);
        let proposal = Empty {
            account: dot::program::Proposal::load(&mut ctx.accounts.proposal, &programs_map),
            bump: Some(ctx.bumps.proposal),
        };

        let clock = &ctx.accounts.clock.clone();

        propose_withdraw_handler(
            user.clone(),
            collection.clone(),
            user_account.clone(),
            multisig.clone(),
            proposal.clone(),
            clock.clone(),
        );

        dot::program::Collection::store(collection);

        dot::program::User::store(user_account);

        dot::program::Multisig::store(multisig);

        dot::program::Proposal::store(proposal.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (approve : bool)]
    pub struct VoteOnProposal<'info> {
        #[account(mut)]
        pub signer: Signer<'info>,
        #[account(mut)]
        pub multisig: Box<Account<'info, dot::program::Multisig>>,
        #[account(mut)]
        pub proposal: Box<Account<'info, dot::program::Proposal>>,
        #[account()]
        pub clock: Sysvar<'info, Clock>,
    }

    pub fn vote_on_proposal(ctx: Context<VoteOnProposal>, approve: bool) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let signer = SeahorseSigner {
            account: &ctx.accounts.signer,
            programs: &programs_map,
        };

        let multisig = dot::program::Multisig::load(&mut ctx.accounts.multisig, &programs_map);
        let proposal = dot::program::Proposal::load(&mut ctx.accounts.proposal, &programs_map);
        let clock = &ctx.accounts.clock.clone();

        vote_on_proposal_handler(
            signer.clone(),
            multisig.clone(),
            proposal.clone(),
            clock.clone(),
            approve,
        );

        dot::program::Multisig::store(multisig);

        dot::program::Proposal::store(proposal);

        return Ok(());
    }
}
