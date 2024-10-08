# rotary_savings
# Built with Seahorse v0.2.0

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Collection(Account):
    admin: Pubkey
    name: str
    duration: i64
    period: i64
    amount_per_period: u64
    total_members: u8
    active_members: u8
    total_balance: u64
    is_active: bool
    early_withdrawal_penalty_rate: u8


class User(Account):
    collection: Pubkey
    user: Pubkey
    paid_periods: u8
    last_paid: i64
    total_paid: u64
    can_withdraw: bool
    early_withdrawal_requested: bool


class Multisig(Account):
    collection: Pubkey
    signers: List[Pubkey]
    threshold: u8
    nonce: u64


class Proposal(Account):
    collection: Pubkey
    proposal_type: u8  # 0: Withdraw, 1: Close Collection, 2: Adjust Settings
    proposer: Pubkey
    created_at: i64
    expires_at: i64
    executed: bool
    approvals: List[bool]
    disapprovals: List[bool]
    # Additional fields based on proposal type
    withdraw_user: Pubkey
    withdraw_amount: u64
    new_duration: i64
    new_period: i64
    new_amount_per_period: u64
    new_early_withdrawal_penalty_rate: u8


@instruction
def create_collection(
    admin: Signer,
    collection: Empty[Collection],
    name: str,
    duration: i64,
    period: i64,
    amount_per_period: u64,
    total_members: u8,
    early_withdrawal_penalty_rate: u8
):
    collection = collection.init(
        payer=admin,
        seeds=['collection', admin.key(), name]
    )
    collection.admin = admin.key()
    collection.name = name
    collection.duration = duration
    collection.period = period
    collection.amount_per_period = amount_per_period
    collection.total_members = total_members
    collection.active_members = 0
    collection.total_balance = 0
    collection.is_active = True
    collection.early_withdrawal_penalty_rate = early_withdrawal_penalty_rate


@instruction
def add_user(
    admin: Signer,
    collection: Collection,
    user: Empty[User],
    new_user: Pubkey
):
    assert admin.key() == collection.admin, 'Only admin can add users'
    assert collection.active_members < collection.total_members, 'Collection is full'
    assert collection.is_active, 'Collection is not active'

    user = user.init(
        payer=admin,
        seeds=['user', collection.key(), new_user]
    )
    user.collection = collection.key()
    user.user = new_user
    user.paid_periods = 0
    user.last_paid = 0
    user.total_paid = 0
    user.can_withdraw = False

    collection.active_members += 1


@instruction
def adjust_settings(
    admin: Signer,
    collection: Collection,
    multisig: Multisig,
    proposal: Empty[Proposal],
    clock: Clock,
    new_duration: i64,
    new_period: i64,
    new_amount_per_period: u64,
):
    assert admin.key() == collection.admin, 'Only admin can propose setting adjustments'
    assert collection.is_active, 'Collection is not active'

    proposal = proposal.init(
        payer=admin,
        seeds=['proposal', collection.key(), str(multisig.nonce)]
    )
    proposal.collection = collection.key()
    proposal.proposal_type = 2  # Adjust Settings
    proposal.proposer = admin.key()
    proposal.created_at = clock.unix_timestamp()
    proposal.expires_at = proposal.created_at + \
        7 * 24 * 60 * 60  # 7 days expiration
    proposal.executed = False

    index: i64 = 0
    for index in range(len(multisig.signers)):
        proposal.approvals.append(False)
        proposal.disapprovals.append(False)
    proposal.new_duration = new_duration
    proposal.new_period = new_period
    proposal.new_amount_per_period = new_amount_per_period

    multisig.nonce += 1


@instruction
def pay(
    user: Signer,
    collection: Collection,
    user_account: User,
    clock: Clock,
    amount: u64,
):
    assert user.key() == user_account.user, 'Invalid user'
    assert collection.is_active, 'Collection is not active'
    assert amount == collection.amount_per_period, 'Invalid payment amount'

    current_time = clock.unix_timestamp()
    assert current_time - \
        user_account.last_paid >= collection.period, 'Too early for next payment'

    user_account.paid_periods += 1
    user_account.last_paid = current_time
    user_account.total_paid += amount
    collection.total_balance += amount

    if user_account.paid_periods * collection.period >= collection.duration:
        user_account.can_withdraw = True


@instruction
def propose_withdraw(
    user: Signer,
    collection: Collection,
    user_account: User,
    multisig: Multisig,
    proposal: Empty[Proposal],
    clock: Clock,
):
    assert user.key() == user_account.user, 'Invalid user'
    assert collection.is_active, 'Collection is not active'
    assert user_account.can_withdraw, 'User cannot withdraw yet'

    proposal = proposal.init(
        payer=user,
        seeds=['proposal', collection.key(), str(multisig.nonce)]
    )
    proposal.collection = collection.key()
    proposal.proposal_type = 0  # Withdraw
    proposal.proposer = user.key()
    proposal.created_at = clock.unix_timestamp()
    proposal.expires_at = proposal.created_at + \
        7 * 24 * 60 * 60  # 7 days expiration
    proposal.executed = False

    index: i64 = 0
    for index in range(len(multisig.signers)):
        proposal.approvals.append(False)
        proposal.disapprovals.append(False)
    proposal.withdraw_user = user.key()
    proposal.withdraw_amount = user_account.total_paid

    multisig.nonce += 1


@instruction
def create_proposal(
    proposer: Signer,
    collection: Collection,
    multisig: Multisig,
    proposal: Empty[Proposal],
    user_account: User,
    clock: Clock,
    proposal_type: u8,
    withdraw_user: Pubkey,
    withdraw_amount: u64,
    new_duration: i64,
    new_period: i64,
    new_amount_per_period: u64,
    new_early_withdrawal_penalty_rate: u8,
):
    assert collection.is_active, 'Collection is not active'

    if proposal_type == 0:  # Regular Withdraw
        assert user_account.can_withdraw, 'User cannot withdraw yet'
    elif proposal_type == 1:  # Close Collection
        assert proposer.key() == collection.admin, 'Only admin can propose to close collection'
    elif proposal_type == 2:  # Adjust Settings
        assert proposer.key() == collection.admin, 'Only admin can propose setting adjustments'
    elif proposal_type == 3:  # Early Withdraw
        assert not user_account.can_withdraw, 'User is eligible for regular withdrawal'
        assert not user_account.early_withdrawal_requested, 'Early withdrawal already requested'
    else:
        assert False, 'Invalid proposal type'

    proposal = proposal.init(
        payer=proposer,
        seeds=['proposal', collection.key(), str(multisig.nonce)]
    )
    proposal.collection = collection.key()
    proposal.proposal_type = proposal_type
    proposal.proposer = proposer.key()
    proposal.created_at = clock.unix_timestamp()
    proposal.expires_at = proposal.created_at + \
        7 * 24 * 60 * 60  # 7 days expiration
    proposal.executed = False

    index: i64 = 0
    for index in range(len(multisig.signers)):
        proposal.approvals.append(False)
        proposal.disapprovals.append(False)
    proposal.withdraw_user = withdraw_user
    proposal.withdraw_amount = withdraw_amount
    proposal.new_duration = new_duration
    proposal.new_period = new_period
    proposal.new_amount_per_period = new_amount_per_period
    proposal.new_early_withdrawal_penalty_rate = new_early_withdrawal_penalty_rate

    if proposal_type == 3:  # Early Withdraw
        user_account.early_withdrawal_requested = True

    multisig.nonce += 1


@instruction
def vote_on_proposal(
    signer: Signer,
    multisig: Multisig,
    proposal: Proposal,
    clock: Clock,
    approve: bool
):
    is_valid_signer = False
    for valid_signer in multisig.signers:
        if valid_signer == signer.key():
            is_valid_signer = True
            break
    assert is_valid_signer, 'Not a valid multisig signer'

    assert not proposal.executed, 'Proposal already executed'
    assert clock.unix_timestamp() < proposal.expires_at, 'Proposal expired'

    signer_index = 0
    index: i64 = 0
    for index in range(len(multisig.signers)):
        if multisig.signers[signer_index] == signer.key():
            break
        signer_index += 1

    if approve:
        proposal.approvals[signer_index] = True
        proposal.disapprovals[signer_index] = False
    else:
        proposal.approvals[signer_index] = False
        proposal.disapprovals[signer_index] = True


@instruction
def execute_proposal(
    collection: Collection,
    multisig: Multisig,
    proposal: Proposal,
    user_account: User,
    clock: Clock,
):
    assert not proposal.executed, 'Proposal already executed'
    assert clock.unix_timestamp() < proposal.expires_at, 'Proposal expired'

    approval_count = 0
    for approved in proposal.approvals:
        if approved:
            approval_count += 1
    assert approval_count >= multisig.threshold, 'Not enough approvals'

    amount_to_withdraw = 0
    if proposal.proposal_type == 0 or proposal.proposal_type == 3:  # Regular or Early Withdraw
        if proposal.proposal_type == 0:
            assert user_account.can_withdraw, 'User cannot withdraw'
            amount_to_withdraw = proposal.withdraw_amount
        else:  # Early Withdraw
            assert user_account.early_withdrawal_requested, 'Early withdrawal not requested'
            penalty = (proposal.withdraw_amount *
                       collection.early_withdrawal_penalty_rate) // 100
            amount_to_withdraw = proposal.withdraw_amount - penalty
            collection.total_balance -= penalty  # Penalty goes to the collection

        user_account.total_paid = 0
        user_account.can_withdraw = False
        user_account.early_withdrawal_requested = False
        collection.total_balance -= amount_to_withdraw

        # Transfer funds to user
        collection.transfer_lamports(
            user_account, amount_to_withdraw)

    elif proposal.proposal_type == 1:  # Close Collection
        collection.is_active = False

    elif proposal.proposal_type == 2:  # Adjust Settings
        collection.duration = proposal.new_duration
        collection.period = proposal.new_period
        collection.amount_per_period = proposal.new_amount_per_period
        collection.early_withdrawal_penalty_rate = proposal.new_early_withdrawal_penalty_rate

    proposal.executed = True


@instruction
def close_collection(
    admin: Signer,
    collection: Collection,
    multisig: Multisig,
    proposal: Empty[Proposal],
    clock: Clock,
):
    assert admin.key() == collection.admin, 'Only admin can propose to close collection'
    assert collection.is_active, 'Collection is already closed'

    proposal = proposal.init(
        payer=admin,
        seeds=['proposal', collection.key(), str(multisig.nonce)]
    )
    proposal.collection = collection.key()
    proposal.proposal_type = 1  # Close Collection
    proposal.proposer = admin.key()
    proposal.created_at = clock.unix_timestamp()
    proposal.expires_at = proposal.created_at + \
        7 * 24 * 60 * 60  # 7 days expiration
    proposal.executed = False

    index: i64 = 0
    for index in range(len(multisig.signers)):
        proposal.approvals.append(False)
        proposal.disapprovals.append(False)

    multisig.nonce += 1


@instruction
def create_multisig(
    admin: Signer,
    collection: Collection,
    multisig: Empty[Multisig],
    signers: List[Pubkey],
    threshold: u8
):
    assert admin.key() == collection.admin, 'Only admin can create multisig'
    assert len(signers) >= threshold, 'Invalid threshold'

    multisig = multisig.init(
        payer=admin,
        seeds=['multisig', collection.key()]
    )
    multisig.collection = collection.key()
    multisig.signers = signers
    multisig.threshold = threshold
    multisig.nonce = 0
