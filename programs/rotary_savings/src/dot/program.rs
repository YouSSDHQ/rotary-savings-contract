#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Collection {
    pub admin: Pubkey,
    pub name: String,
    pub duration: i64,
    pub period: i64,
    pub amount_per_period: u64,
    pub total_members: u8,
    pub active_members: u8,
    pub total_balance: u64,
    pub is_active: bool,
    pub early_withdrawal_penalty_rate: u8,
}

impl<'info, 'entrypoint> Collection {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedCollection<'info, 'entrypoint>> {
        let admin = account.admin.clone();
        let name = account.name.clone();
        let duration = account.duration;
        let period = account.period;
        let amount_per_period = account.amount_per_period;
        let total_members = account.total_members;
        let active_members = account.active_members;
        let total_balance = account.total_balance;
        let is_active = account.is_active.clone();
        let early_withdrawal_penalty_rate = account.early_withdrawal_penalty_rate;

        Mutable::new(LoadedCollection {
            __account__: account,
            __programs__: programs_map,
            admin,
            name,
            duration,
            period,
            amount_per_period,
            total_members,
            active_members,
            total_balance,
            is_active,
            early_withdrawal_penalty_rate,
        })
    }

    pub fn store(loaded: Mutable<LoadedCollection>) {
        let mut loaded = loaded.borrow_mut();
        let admin = loaded.admin.clone();

        loaded.__account__.admin = admin;

        let name = loaded.name.clone();

        loaded.__account__.name = name;

        let duration = loaded.duration;

        loaded.__account__.duration = duration;

        let period = loaded.period;

        loaded.__account__.period = period;

        let amount_per_period = loaded.amount_per_period;

        loaded.__account__.amount_per_period = amount_per_period;

        let total_members = loaded.total_members;

        loaded.__account__.total_members = total_members;

        let active_members = loaded.active_members;

        loaded.__account__.active_members = active_members;

        let total_balance = loaded.total_balance;

        loaded.__account__.total_balance = total_balance;

        let is_active = loaded.is_active.clone();

        loaded.__account__.is_active = is_active;

        let early_withdrawal_penalty_rate = loaded.early_withdrawal_penalty_rate;

        loaded.__account__.early_withdrawal_penalty_rate = early_withdrawal_penalty_rate;
    }
}

#[derive(Debug)]
pub struct LoadedCollection<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Collection>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub admin: Pubkey,
    pub name: String,
    pub duration: i64,
    pub period: i64,
    pub amount_per_period: u64,
    pub total_members: u8,
    pub active_members: u8,
    pub total_balance: u64,
    pub is_active: bool,
    pub early_withdrawal_penalty_rate: u8,
}

#[account]
#[derive(Debug)]
pub struct Multisig {
    pub collection: Pubkey,
    pub signers: Vec<Pubkey>,
    pub threshold: u8,
    pub nonce: u64,
}

impl<'info, 'entrypoint> Multisig {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedMultisig<'info, 'entrypoint>> {
        let collection = account.collection.clone();
        let signers = Mutable::new(
            account
                .signers
                .clone()
                .into_iter()
                .map(|element| element)
                .collect(),
        );

        let threshold = account.threshold;
        let nonce = account.nonce;

        Mutable::new(LoadedMultisig {
            __account__: account,
            __programs__: programs_map,
            collection,
            signers,
            threshold,
            nonce,
        })
    }

    pub fn store(loaded: Mutable<LoadedMultisig>) {
        let mut loaded = loaded.borrow_mut();
        let collection = loaded.collection.clone();

        loaded.__account__.collection = collection;

        let signers = loaded
            .signers
            .clone()
            .borrow()
            .clone()
            .into_iter()
            .map(|element| element)
            .collect();

        loaded.__account__.signers = signers;

        let threshold = loaded.threshold;

        loaded.__account__.threshold = threshold;

        let nonce = loaded.nonce;

        loaded.__account__.nonce = nonce;
    }
}

#[derive(Debug)]
pub struct LoadedMultisig<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Multisig>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub collection: Pubkey,
    pub signers: Mutable<Vec<Pubkey>>,
    pub threshold: u8,
    pub nonce: u64,
}

#[account]
#[derive(Debug)]
pub struct Proposal {
    pub collection: Pubkey,
    pub proposal_type: u8,
    pub proposer: Pubkey,
    pub created_at: i64,
    pub expires_at: i64,
    pub executed: bool,
    pub approvals: Vec<bool>,
    pub disapprovals: Vec<bool>,
    pub withdraw_user: Pubkey,
    pub withdraw_amount: u64,
    pub new_duration: i64,
    pub new_period: i64,
    pub new_amount_per_period: u64,
    pub new_early_withdrawal_penalty_rate: u8,
}

impl<'info, 'entrypoint> Proposal {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedProposal<'info, 'entrypoint>> {
        let collection = account.collection.clone();
        let proposal_type = account.proposal_type;
        let proposer = account.proposer.clone();
        let created_at = account.created_at;
        let expires_at = account.expires_at;
        let executed = account.executed.clone();
        let approvals = Mutable::new(
            account
                .approvals
                .clone()
                .into_iter()
                .map(|element| element)
                .collect(),
        );

        let disapprovals = Mutable::new(
            account
                .disapprovals
                .clone()
                .into_iter()
                .map(|element| element)
                .collect(),
        );

        let withdraw_user = account.withdraw_user.clone();
        let withdraw_amount = account.withdraw_amount;
        let new_duration = account.new_duration;
        let new_period = account.new_period;
        let new_amount_per_period = account.new_amount_per_period;
        let new_early_withdrawal_penalty_rate = account.new_early_withdrawal_penalty_rate;

        Mutable::new(LoadedProposal {
            __account__: account,
            __programs__: programs_map,
            collection,
            proposal_type,
            proposer,
            created_at,
            expires_at,
            executed,
            approvals,
            disapprovals,
            withdraw_user,
            withdraw_amount,
            new_duration,
            new_period,
            new_amount_per_period,
            new_early_withdrawal_penalty_rate,
        })
    }

    pub fn store(loaded: Mutable<LoadedProposal>) {
        let mut loaded = loaded.borrow_mut();
        let collection = loaded.collection.clone();

        loaded.__account__.collection = collection;

        let proposal_type = loaded.proposal_type;

        loaded.__account__.proposal_type = proposal_type;

        let proposer = loaded.proposer.clone();

        loaded.__account__.proposer = proposer;

        let created_at = loaded.created_at;

        loaded.__account__.created_at = created_at;

        let expires_at = loaded.expires_at;

        loaded.__account__.expires_at = expires_at;

        let executed = loaded.executed.clone();

        loaded.__account__.executed = executed;

        let approvals = loaded
            .approvals
            .clone()
            .borrow()
            .clone()
            .into_iter()
            .map(|element| element)
            .collect();

        loaded.__account__.approvals = approvals;

        let disapprovals = loaded
            .disapprovals
            .clone()
            .borrow()
            .clone()
            .into_iter()
            .map(|element| element)
            .collect();

        loaded.__account__.disapprovals = disapprovals;

        let withdraw_user = loaded.withdraw_user.clone();

        loaded.__account__.withdraw_user = withdraw_user;

        let withdraw_amount = loaded.withdraw_amount;

        loaded.__account__.withdraw_amount = withdraw_amount;

        let new_duration = loaded.new_duration;

        loaded.__account__.new_duration = new_duration;

        let new_period = loaded.new_period;

        loaded.__account__.new_period = new_period;

        let new_amount_per_period = loaded.new_amount_per_period;

        loaded.__account__.new_amount_per_period = new_amount_per_period;

        let new_early_withdrawal_penalty_rate = loaded.new_early_withdrawal_penalty_rate;

        loaded.__account__.new_early_withdrawal_penalty_rate = new_early_withdrawal_penalty_rate;
    }
}

#[derive(Debug)]
pub struct LoadedProposal<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Proposal>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub collection: Pubkey,
    pub proposal_type: u8,
    pub proposer: Pubkey,
    pub created_at: i64,
    pub expires_at: i64,
    pub executed: bool,
    pub approvals: Mutable<Vec<bool>>,
    pub disapprovals: Mutable<Vec<bool>>,
    pub withdraw_user: Pubkey,
    pub withdraw_amount: u64,
    pub new_duration: i64,
    pub new_period: i64,
    pub new_amount_per_period: u64,
    pub new_early_withdrawal_penalty_rate: u8,
}

#[account]
#[derive(Debug)]
pub struct User {
    pub collection: Pubkey,
    pub user: Pubkey,
    pub paid_periods: u8,
    pub last_paid: i64,
    pub total_paid: u64,
    pub can_withdraw: bool,
    pub early_withdrawal_requested: bool,
}

impl<'info, 'entrypoint> User {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedUser<'info, 'entrypoint>> {
        let collection = account.collection.clone();
        let user = account.user.clone();
        let paid_periods = account.paid_periods;
        let last_paid = account.last_paid;
        let total_paid = account.total_paid;
        let can_withdraw = account.can_withdraw.clone();
        let early_withdrawal_requested = account.early_withdrawal_requested.clone();

        Mutable::new(LoadedUser {
            __account__: account,
            __programs__: programs_map,
            collection,
            user,
            paid_periods,
            last_paid,
            total_paid,
            can_withdraw,
            early_withdrawal_requested,
        })
    }

    pub fn store(loaded: Mutable<LoadedUser>) {
        let mut loaded = loaded.borrow_mut();
        let collection = loaded.collection.clone();

        loaded.__account__.collection = collection;

        let user = loaded.user.clone();

        loaded.__account__.user = user;

        let paid_periods = loaded.paid_periods;

        loaded.__account__.paid_periods = paid_periods;

        let last_paid = loaded.last_paid;

        loaded.__account__.last_paid = last_paid;

        let total_paid = loaded.total_paid;

        loaded.__account__.total_paid = total_paid;

        let can_withdraw = loaded.can_withdraw.clone();

        loaded.__account__.can_withdraw = can_withdraw;

        let early_withdrawal_requested = loaded.early_withdrawal_requested.clone();

        loaded.__account__.early_withdrawal_requested = early_withdrawal_requested;
    }
}

#[derive(Debug)]
pub struct LoadedUser<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, User>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub collection: Pubkey,
    pub user: Pubkey,
    pub paid_periods: u8,
    pub last_paid: i64,
    pub total_paid: u64,
    pub can_withdraw: bool,
    pub early_withdrawal_requested: bool,
}

pub fn add_user_handler<'info>(
    mut admin: SeahorseSigner<'info, '_>,
    mut collection: Mutable<LoadedCollection<'info, '_>>,
    mut user: Empty<Mutable<LoadedUser<'info, '_>>>,
    mut new_user: Pubkey,
) -> () {
    if !(admin.key() == collection.borrow().admin) {
        panic!("Only admin can add users");
    }

    if !(collection.borrow().active_members < collection.borrow().total_members) {
        panic!("Collection is full");
    }

    if !collection.borrow().is_active {
        panic!("Collection is not active");
    }

    let mut user = user.account.clone();

    assign!(
        user.borrow_mut().collection,
        collection.borrow().__account__.key()
    );

    assign!(user.borrow_mut().user, new_user);

    assign!(user.borrow_mut().paid_periods, 0);

    assign!(user.borrow_mut().last_paid, 0);

    assign!(user.borrow_mut().total_paid, 0);

    assign!(user.borrow_mut().can_withdraw, false);

    assign!(
        collection.borrow_mut().active_members,
        collection.borrow().active_members + 1
    );
}

pub fn adjust_settings_handler<'info>(
    mut admin: SeahorseSigner<'info, '_>,
    mut collection: Mutable<LoadedCollection<'info, '_>>,
    mut multisig: Mutable<LoadedMultisig<'info, '_>>,
    mut proposal: Empty<Mutable<LoadedProposal<'info, '_>>>,
    mut clock: Sysvar<'info, Clock>,
    mut new_duration: i64,
    mut new_period: i64,
    mut new_amount_per_period: u64,
) -> () {
    if !(admin.key() == collection.borrow().admin) {
        panic!("Only admin can propose setting adjustments");
    }

    if !collection.borrow().is_active {
        panic!("Collection is not active");
    }

    let mut proposal = proposal.account.clone();

    assign!(
        proposal.borrow_mut().collection,
        collection.borrow().__account__.key()
    );

    assign!(proposal.borrow_mut().proposal_type, 2);

    assign!(proposal.borrow_mut().proposer, admin.key());

    assign!(proposal.borrow_mut().created_at, clock.unix_timestamp);

    assign!(
        proposal.borrow_mut().expires_at,
        proposal.borrow().created_at + (((7 * 24) * 60) * 60)
    );

    assign!(proposal.borrow_mut().executed, false);

    let mut index = 0;

    for mut index in 0..(multisig.borrow().signers.borrow().len() as u64) {
        proposal.borrow().approvals.borrow_mut().push(false);

        proposal.borrow().disapprovals.borrow_mut().push(false);
    }

    assign!(proposal.borrow_mut().new_duration, new_duration);

    assign!(proposal.borrow_mut().new_period, new_period);

    assign!(
        proposal.borrow_mut().new_amount_per_period,
        new_amount_per_period
    );

    assign!(multisig.borrow_mut().nonce, multisig.borrow().nonce + 1);
}

pub fn close_collection_handler<'info>(
    mut admin: SeahorseSigner<'info, '_>,
    mut collection: Mutable<LoadedCollection<'info, '_>>,
    mut multisig: Mutable<LoadedMultisig<'info, '_>>,
    mut proposal: Empty<Mutable<LoadedProposal<'info, '_>>>,
    mut clock: Sysvar<'info, Clock>,
) -> () {
    if !(admin.key() == collection.borrow().admin) {
        panic!("Only admin can propose to close collection");
    }

    if !collection.borrow().is_active {
        panic!("Collection is already closed");
    }

    let mut proposal = proposal.account.clone();

    assign!(
        proposal.borrow_mut().collection,
        collection.borrow().__account__.key()
    );

    assign!(proposal.borrow_mut().proposal_type, 1);

    assign!(proposal.borrow_mut().proposer, admin.key());

    assign!(proposal.borrow_mut().created_at, clock.unix_timestamp);

    assign!(
        proposal.borrow_mut().expires_at,
        proposal.borrow().created_at + (((7 * 24) * 60) * 60)
    );

    assign!(proposal.borrow_mut().executed, false);

    let mut index = 0;

    for mut index in 0..(multisig.borrow().signers.borrow().len() as u64) {
        proposal.borrow().approvals.borrow_mut().push(false);

        proposal.borrow().disapprovals.borrow_mut().push(false);
    }

    assign!(multisig.borrow_mut().nonce, multisig.borrow().nonce + 1);
}

pub fn create_collection_handler<'info>(
    mut admin: SeahorseSigner<'info, '_>,
    mut collection: Empty<Mutable<LoadedCollection<'info, '_>>>,
    mut name: String,
    mut duration: i64,
    mut period: i64,
    mut amount_per_period: u64,
    mut total_members: u8,
    mut early_withdrawal_penalty_rate: u8,
) -> () {
    let mut collection = collection.account.clone();

    assign!(collection.borrow_mut().admin, admin.key());

    assign!(collection.borrow_mut().name, name);

    assign!(collection.borrow_mut().duration, duration);

    assign!(collection.borrow_mut().period, period);

    assign!(collection.borrow_mut().amount_per_period, amount_per_period);

    assign!(collection.borrow_mut().total_members, total_members);

    assign!(collection.borrow_mut().active_members, 0);

    assign!(collection.borrow_mut().total_balance, 0);

    assign!(collection.borrow_mut().is_active, true);

    assign!(
        collection.borrow_mut().early_withdrawal_penalty_rate,
        early_withdrawal_penalty_rate
    );
}

pub fn create_multisig_handler<'info>(
    mut admin: SeahorseSigner<'info, '_>,
    mut collection: Mutable<LoadedCollection<'info, '_>>,
    mut multisig: Empty<Mutable<LoadedMultisig<'info, '_>>>,
    mut signers: Mutable<Vec<Pubkey>>,
    mut threshold: u8,
) -> () {
    if !(admin.key() == collection.borrow().admin) {
        panic!("Only admin can create multisig");
    }

    if !((signers.borrow().len() as u64) >= (threshold as u64)) {
        panic!("Invalid threshold");
    }

    let mut multisig = multisig.account.clone();

    assign!(
        multisig.borrow_mut().collection,
        collection.borrow().__account__.key()
    );

    assign!(multisig.borrow_mut().signers, signers);

    assign!(multisig.borrow_mut().threshold, threshold);

    assign!(multisig.borrow_mut().nonce, 0);
}

pub fn create_proposal_handler<'info>(
    mut proposer: SeahorseSigner<'info, '_>,
    mut collection: Mutable<LoadedCollection<'info, '_>>,
    mut multisig: Mutable<LoadedMultisig<'info, '_>>,
    mut proposal: Empty<Mutable<LoadedProposal<'info, '_>>>,
    mut user_account: Mutable<LoadedUser<'info, '_>>,
    mut clock: Sysvar<'info, Clock>,
    mut proposal_type: u8,
    mut withdraw_user: Pubkey,
    mut withdraw_amount: u64,
    mut new_duration: i64,
    mut new_period: i64,
    mut new_amount_per_period: u64,
    mut new_early_withdrawal_penalty_rate: u8,
) -> () {
    if !collection.borrow().is_active {
        panic!("Collection is not active");
    }

    if proposal_type == 0 {
        if !user_account.borrow().can_withdraw {
            panic!("User cannot withdraw yet");
        }
    } else {
        if proposal_type == 1 {
            if !(proposer.key() == collection.borrow().admin) {
                panic!("Only admin can propose to close collection");
            }
        } else {
            if proposal_type == 2 {
                if !(proposer.key() == collection.borrow().admin) {
                    panic!("Only admin can propose setting adjustments");
                }
            } else {
                if proposal_type == 3 {
                    if !(!user_account.borrow().can_withdraw) {
                        panic!("User is eligible for regular withdrawal");
                    }

                    if !(!user_account.borrow().early_withdrawal_requested) {
                        panic!("Early withdrawal already requested");
                    }
                } else {
                    if !false {
                        panic!("Invalid proposal type");
                    }
                }
            }
        }
    }

    let mut proposal = proposal.account.clone();

    assign!(
        proposal.borrow_mut().collection,
        collection.borrow().__account__.key()
    );

    assign!(proposal.borrow_mut().proposal_type, proposal_type);

    assign!(proposal.borrow_mut().proposer, proposer.key());

    assign!(proposal.borrow_mut().created_at, clock.unix_timestamp);

    assign!(
        proposal.borrow_mut().expires_at,
        proposal.borrow().created_at + (((7 * 24) * 60) * 60)
    );

    assign!(proposal.borrow_mut().executed, false);

    let mut index = 0;

    for mut index in 0..(multisig.borrow().signers.borrow().len() as u64) {
        proposal.borrow().approvals.borrow_mut().push(false);

        proposal.borrow().disapprovals.borrow_mut().push(false);
    }

    assign!(proposal.borrow_mut().withdraw_user, withdraw_user);

    assign!(proposal.borrow_mut().withdraw_amount, withdraw_amount);

    assign!(proposal.borrow_mut().new_duration, new_duration);

    assign!(proposal.borrow_mut().new_period, new_period);

    assign!(
        proposal.borrow_mut().new_amount_per_period,
        new_amount_per_period
    );

    assign!(
        proposal.borrow_mut().new_early_withdrawal_penalty_rate,
        new_early_withdrawal_penalty_rate
    );

    if proposal_type == 3 {
        assign!(user_account.borrow_mut().early_withdrawal_requested, true);
    }

    assign!(multisig.borrow_mut().nonce, multisig.borrow().nonce + 1);
}

pub fn execute_proposal_handler<'info>(
    mut collection: Mutable<LoadedCollection<'info, '_>>,
    mut multisig: Mutable<LoadedMultisig<'info, '_>>,
    mut proposal: Mutable<LoadedProposal<'info, '_>>,
    mut user_account: Mutable<LoadedUser<'info, '_>>,
    mut clock: Sysvar<'info, Clock>,
) -> () {
    if !(!proposal.borrow().executed) {
        panic!("Proposal already executed");
    }

    if !(clock.unix_timestamp < proposal.borrow().expires_at) {
        panic!("Proposal expired");
    }

    let mut approval_count = 0;

    for mut approved in proposal
        .borrow()
        .approvals
        .borrow()
        .iter()
        .map(|elem| elem.clone())
    {
        if approved {
            assign!(approval_count, approval_count + 1);
        }
    }

    if !(approval_count >= multisig.borrow().threshold) {
        panic!("Not enough approvals");
    }

    let mut amount_to_withdraw = 0;

    if (proposal.borrow().proposal_type == 0) || (proposal.borrow().proposal_type == 3) {
        if proposal.borrow().proposal_type == 0 {
            if !user_account.borrow().can_withdraw {
                panic!("User cannot withdraw");
            }

            amount_to_withdraw = proposal.borrow().withdraw_amount;
        } else {
            if !user_account.borrow().early_withdrawal_requested {
                panic!("Early withdrawal not requested");
            }

            let mut penalty = (proposal.borrow().withdraw_amount
                * (collection.borrow().early_withdrawal_penalty_rate as u64))
                / 100;

            amount_to_withdraw = proposal.borrow().withdraw_amount - penalty;

            assign!(
                collection.borrow_mut().total_balance,
                collection.borrow().total_balance - penalty
            );
        }

        assign!(user_account.borrow_mut().total_paid, 0);

        assign!(user_account.borrow_mut().can_withdraw, false);

        assign!(user_account.borrow_mut().early_withdrawal_requested, false);

        assign!(
            collection.borrow_mut().total_balance,
            collection.borrow().total_balance - amount_to_withdraw
        );

        {
            let amount = amount_to_withdraw.clone();

            **collection
                .borrow()
                .__account__
                .to_account_info()
                .try_borrow_mut_lamports()
                .unwrap() -= amount;

            **user_account
                .borrow()
                .__account__
                .to_account_info()
                .try_borrow_mut_lamports()
                .unwrap() += amount;
        };
    } else {
        if proposal.borrow().proposal_type == 1 {
            assign!(collection.borrow_mut().is_active, false);
        } else {
            if proposal.borrow().proposal_type == 2 {
                assign!(
                    collection.borrow_mut().duration,
                    proposal.borrow().new_duration
                );

                assign!(collection.borrow_mut().period, proposal.borrow().new_period);

                assign!(
                    collection.borrow_mut().amount_per_period,
                    proposal.borrow().new_amount_per_period
                );

                assign!(
                    collection.borrow_mut().early_withdrawal_penalty_rate,
                    proposal.borrow().new_early_withdrawal_penalty_rate
                );
            }
        }
    }

    assign!(proposal.borrow_mut().executed, true);
}

pub fn pay_handler<'info>(
    mut user: SeahorseSigner<'info, '_>,
    mut collection: Mutable<LoadedCollection<'info, '_>>,
    mut user_account: Mutable<LoadedUser<'info, '_>>,
    mut clock: Sysvar<'info, Clock>,
    mut amount: u64,
) -> () {
    if !(user.key() == user_account.borrow().user) {
        panic!("Invalid user");
    }

    if !collection.borrow().is_active {
        panic!("Collection is not active");
    }

    if !(amount == collection.borrow().amount_per_period) {
        panic!("Invalid payment amount");
    }

    let mut current_time = clock.unix_timestamp;

    if !((current_time - user_account.borrow().last_paid) >= collection.borrow().period) {
        panic!("Too early for next payment");
    }

    assign!(
        user_account.borrow_mut().paid_periods,
        user_account.borrow().paid_periods + 1
    );

    assign!(user_account.borrow_mut().last_paid, current_time);

    assign!(
        user_account.borrow_mut().total_paid,
        user_account.borrow().total_paid + amount
    );

    assign!(
        collection.borrow_mut().total_balance,
        collection.borrow().total_balance + amount
    );

    if ((user_account.borrow().paid_periods as i64) * collection.borrow().period)
        >= collection.borrow().duration
    {
        assign!(user_account.borrow_mut().can_withdraw, true);
    }
}

pub fn propose_withdraw_handler<'info>(
    mut user: SeahorseSigner<'info, '_>,
    mut collection: Mutable<LoadedCollection<'info, '_>>,
    mut user_account: Mutable<LoadedUser<'info, '_>>,
    mut multisig: Mutable<LoadedMultisig<'info, '_>>,
    mut proposal: Empty<Mutable<LoadedProposal<'info, '_>>>,
    mut clock: Sysvar<'info, Clock>,
) -> () {
    if !(user.key() == user_account.borrow().user) {
        panic!("Invalid user");
    }

    if !collection.borrow().is_active {
        panic!("Collection is not active");
    }

    if !user_account.borrow().can_withdraw {
        panic!("User cannot withdraw yet");
    }

    let mut proposal = proposal.account.clone();

    assign!(
        proposal.borrow_mut().collection,
        collection.borrow().__account__.key()
    );

    assign!(proposal.borrow_mut().proposal_type, 0);

    assign!(proposal.borrow_mut().proposer, user.key());

    assign!(proposal.borrow_mut().created_at, clock.unix_timestamp);

    assign!(
        proposal.borrow_mut().expires_at,
        proposal.borrow().created_at + (((7 * 24) * 60) * 60)
    );

    assign!(proposal.borrow_mut().executed, false);

    let mut index = 0;

    for mut index in 0..(multisig.borrow().signers.borrow().len() as u64) {
        proposal.borrow().approvals.borrow_mut().push(false);

        proposal.borrow().disapprovals.borrow_mut().push(false);
    }

    assign!(proposal.borrow_mut().withdraw_user, user.key());

    assign!(
        proposal.borrow_mut().withdraw_amount,
        user_account.borrow().total_paid
    );

    assign!(multisig.borrow_mut().nonce, multisig.borrow().nonce + 1);
}

pub fn vote_on_proposal_handler<'info>(
    mut signer: SeahorseSigner<'info, '_>,
    mut multisig: Mutable<LoadedMultisig<'info, '_>>,
    mut proposal: Mutable<LoadedProposal<'info, '_>>,
    mut clock: Sysvar<'info, Clock>,
    mut approve: bool,
) -> () {
    let mut is_valid_signer = false;

    for mut valid_signer in multisig
        .borrow()
        .signers
        .borrow()
        .iter()
        .map(|elem| elem.clone())
    {
        if valid_signer == signer.key() {
            is_valid_signer = true;

            break;
        }
    }

    if !is_valid_signer {
        panic!("Not a valid multisig signer");
    }

    if !(!proposal.borrow().executed) {
        panic!("Proposal already executed");
    }

    if !(clock.unix_timestamp < proposal.borrow().expires_at) {
        panic!("Proposal expired");
    }

    let mut signer_index = 0;
    let mut index = 0;

    for mut index in 0..(multisig.borrow().signers.borrow().len() as u64) {
        if (*multisig
            .borrow()
            .signers
            .borrow()
            .index_wrapped(signer_index.into()))
            == signer.key()
        {
            break;
        }

        assign!(signer_index, signer_index + 1);
    }

    if approve {
        assign!(
            (*proposal
                .borrow_mut()
                .approvals
                .borrow_mut()
                .index_wrapped_mut(signer_index.into())),
            true
        );

        assign!(
            (*proposal
                .borrow_mut()
                .disapprovals
                .borrow_mut()
                .index_wrapped_mut(signer_index.into())),
            false
        );
    } else {
        assign!(
            (*proposal
                .borrow_mut()
                .approvals
                .borrow_mut()
                .index_wrapped_mut(signer_index.into())),
            false
        );

        assign!(
            (*proposal
                .borrow_mut()
                .disapprovals
                .borrow_mut()
                .index_wrapped_mut(signer_index.into())),
            true
        );
    }
}
