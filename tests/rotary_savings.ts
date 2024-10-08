import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { expect } from 'chai';
import { RotarySavings } from '../target/types/rotary_savings';

describe('rotary_savings', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RotarySavings as Program<RotarySavings>;

  const admin = Keypair.generate();
  const user1 = Keypair.generate();
  const user2 = Keypair.generate();
  const multisigSigner1 = Keypair.generate();
  const multisigSigner2 = Keypair.generate();
  const multisigSigner3 = Keypair.generate();

  let collectionPDA: PublicKey;
  let multisigPDA: PublicKey;

  const collectionName = 'Test Collection';
  const duration = 30 * 24 * 60 * 60; // 30 days in seconds
  const period = 7 * 24 * 60 * 60; // 7 days in seconds
  const amountPerPeriod = new anchor.BN(100000000); // 0.1 SOL
  const totalMembers = 5;
  const earlyWithdrawalPenaltyRate = 5; // 5%

  before(async () => {
    // Airdrop SOL to admin and users
    await provider.connection.requestAirdrop(
      admin.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      user1.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      user2.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    // Find PDAs
    [collectionPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('collection'),
        admin.publicKey.toBuffer(),
        Buffer.from(collectionName),
      ],
      program.programId
    );

    [multisigPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('multisig'), collectionPDA.toBuffer()],
      program.programId
    );
  });

  it('Creates a collection', async () => {
    const tx = await program.methods
      .createCollection(
        collectionName,
        new anchor.BN(duration),
        new anchor.BN(period),
        amountPerPeriod,
        totalMembers,
        earlyWithdrawalPenaltyRate
      )
      .accounts({
        admin: admin.publicKey,
        collection: collectionPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    const collectionAccount = await program.account.collection.fetch(
      collectionPDA
    );
    expect(collectionAccount.admin.toString()).to.equal(
      admin.publicKey.toString()
    );
    expect(collectionAccount.name).to.equal(collectionName);
    expect(collectionAccount.duration.toNumber()).to.equal(duration);
    expect(collectionAccount.period.toNumber()).to.equal(period);
    expect(collectionAccount.amountPerPeriod.toNumber()).to.equal(
      amountPerPeriod.toNumber()
    );
    expect(collectionAccount.totalMembers).to.equal(totalMembers);
    expect(collectionAccount.activeMembers).to.equal(0);
    expect(collectionAccount.totalBalance.toNumber()).to.equal(0);
    expect(collectionAccount.isActive).to.be.true;
    expect(collectionAccount.earlyWithdrawalPenaltyRate).to.equal(
      earlyWithdrawalPenaltyRate
    );
  });

  it('Creates a multisig', async () => {
    const signers = [
      multisigSigner1.publicKey,
      multisigSigner2.publicKey,
      multisigSigner3.publicKey,
    ];
    const threshold = 2;

    const tx = await program.methods
      .createMultisig(signers, threshold)
      .accounts({
        admin: admin.publicKey,
        collection: collectionPDA,
        multisig: multisigPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    const multisigAccount = await program.account.multisig.fetch(multisigPDA);
    expect(multisigAccount.collection.toString()).to.equal(
      collectionPDA.toString()
    );
    expect(multisigAccount.signers.map((s) => s.toString())).to.deep.equal(
      signers.map((s) => s.toString())
    );
    expect(multisigAccount.threshold).to.equal(threshold);
    expect(multisigAccount.nonce.toNumber()).to.equal(0);
  });

  it('Adds a user to the collection', async () => {
    const [userPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('user'),
        collectionPDA.toBuffer(),
        user1.publicKey.toBuffer(),
      ],
      program.programId
    );

    const tx = await program.methods
      .addUser(user1.publicKey)
      .accounts({
        admin: admin.publicKey,
        collection: collectionPDA,
        user: userPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    const userAccount = await program.account.user.fetch(userPDA);
    expect(userAccount.collection.toString()).to.equal(
      collectionPDA.toString()
    );
    expect(userAccount.user.toString()).to.equal(user1.publicKey.toString());
    expect(userAccount.paidPeriods).to.equal(0);
    expect(userAccount.lastPaid.toNumber()).to.equal(0);
    expect(userAccount.totalPaid.toNumber()).to.equal(0);
    expect(userAccount.canWithdraw).to.be.false;
    expect(userAccount.earlyWithdrawalRequested).to.be.false;

    const collectionAccount = await program.account.collection.fetch(
      collectionPDA
    );
    expect(collectionAccount.activeMembers).to.equal(1);
  });

  it('Allows a user to pay into the collection', async () => {
    const [userPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('user'),
        collectionPDA.toBuffer(),
        user1.publicKey.toBuffer(),
      ],
      program.programId
    );

    const tx = await program.methods
      .pay(amountPerPeriod)
      .accounts({
        user: user1.publicKey,
        collection: collectionPDA,
        userAccount: userPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([user1])
      .rpc();

    const userAccount = await program.account.user.fetch(userPDA);
    expect(userAccount.paidPeriods).to.equal(1);
    expect(userAccount.totalPaid.toNumber()).to.equal(
      amountPerPeriod.toNumber()
    );

    const collectionAccount = await program.account.collection.fetch(
      collectionPDA
    );
    expect(collectionAccount.totalBalance.toNumber()).to.equal(
      amountPerPeriod.toNumber()
    );
  });

  it('Prevents a user from paying twice in the same period', async () => {
    const [userPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('user'),
        collectionPDA.toBuffer(),
        user1.publicKey.toBuffer(),
      ],
      program.programId
    );

    try {
      await program.methods
        .pay(amountPerPeriod)
        .accounts({
          user: user1.publicKey,
          collection: collectionPDA,
          userAccount: userPDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();
      expect.fail('Should have thrown an error');
    } catch (error) {
      expect(error.message).to.include('Too early for next payment');
    }
  });

  it('Creates a proposal for early withdrawal', async () => {
    const [userPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('user'),
        collectionPDA.toBuffer(),
        user1.publicKey.toBuffer(),
      ],
      program.programId
    );

    const [proposalPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('proposal'), collectionPDA.toBuffer(), Buffer.from([0])],
      program.programId
    );

    const tx = await program.methods
      .createProposal(
        3,
        user1.publicKey,
        amountPerPeriod,
        new anchor.BN(0),
        new anchor.BN(0),
        new anchor.BN(0),
        0
      )
      .accounts({
        proposer: user1.publicKey,
        collection: collectionPDA,
        multisig: multisigPDA,
        proposal: proposalPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([user1])
      .rpc();

    const proposalAccount = await program.account.proposal.fetch(proposalPDA);
    expect(proposalAccount.proposalType).to.equal(3);
    expect(proposalAccount.withdrawUser.toString()).to.equal(
      user1.publicKey.toString()
    );
    expect(proposalAccount.withdrawAmount.toNumber()).to.equal(
      amountPerPeriod.toNumber()
    );

    const userAccount = await program.account.user.fetch(userPDA);
    expect(userAccount.earlyWithdrawalRequested).to.be.true;
  });

  it('Allows multisig signers to vote on the proposal', async () => {
    const [proposalPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('proposal'), collectionPDA.toBuffer(), Buffer.from([0])],
      program.programId
    );

    await program.methods
      .voteOnProposal(true)
      .accounts({
        signer: multisigSigner1.publicKey,
        collection: collectionPDA,
        multisig: multisigPDA,
        proposal: proposalPDA,
      })
      .signers([multisigSigner1])
      .rpc();

    await program.methods
      .voteOnProposal(true)
      .accounts({
        signer: multisigSigner2.publicKey,
        collection: collectionPDA,
        multisig: multisigPDA,
        proposal: proposalPDA,
      })
      .signers([multisigSigner2])
      .rpc();

    const proposalAccount = await program.account.proposal.fetch(proposalPDA);
    expect(proposalAccount.approvals[0]).to.be.true;
    expect(proposalAccount.approvals[1]).to.be.true;
    expect(proposalAccount.approvals[2]).to.be.false;
  });

  it('Executes the early withdrawal proposal', async () => {
    const [proposalPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('proposal'), collectionPDA.toBuffer(), Buffer.from([0])],
      program.programId
    );

    const [userPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('user'),
        collectionPDA.toBuffer(),
        user1.publicKey.toBuffer(),
      ],
      program.programId
    );

    const userBalanceBefore = await provider.connection.getBalance(
      user1.publicKey
    );

    const tx = await program.methods
      .executeProposal()
      .accounts({
        executor: admin.publicKey,
        collection: collectionPDA,
        multisig: multisigPDA,
        proposal: proposalPDA,
        withdrawUser: user1.publicKey,
        userAccount: userPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    const userBalanceAfter = await provider.connection.getBalance(
      user1.publicKey
    );
    const expectedWithdrawalAmount =
      (amountPerPeriod.toNumber() * (100 - earlyWithdrawalPenaltyRate)) / 100;

    expect(userBalanceAfter - userBalanceBefore).to.be.closeTo(
      expectedWithdrawalAmount,
      10000
    ); // Allow for small difference due to transaction fees

    const userAccount = await program.account.user.fetch(userPDA);
    expect(userAccount.totalPaid.toNumber()).to.equal(0);
    expect(userAccount.earlyWithdrawalRequested).to.be.false;

    const collectionAccount = await program.account.collection.fetch(
      collectionPDA
    );
    const expectedPenaltyAmount =
      (amountPerPeriod.toNumber() * earlyWithdrawalPenaltyRate) / 100;
    expect(collectionAccount.totalBalance.toNumber()).to.equal(
      expectedPenaltyAmount
    );

    const proposalAccount = await program.account.proposal.fetch(proposalPDA);
    expect(proposalAccount.executed).to.be.true;
  });
});
