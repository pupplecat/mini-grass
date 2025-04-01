import { Program } from '@coral-xyz/anchor'
import * as anchor from '@coral-xyz/anchor'
import { BwRecorder } from '../../target/types/bw_recorder'
import { PublicKey, SystemProgram } from '@solana/web3.js'
import { expect } from 'chai'
import { BN } from 'bn.js'

describe('initialize', () => {
  const program = anchor.workspace.BwRecorder as Program<BwRecorder>

  const recorderPubkey = () => {
    return PublicKey.findProgramAddressSync(
      [Buffer.from('state')],
      program.programId
    )[0]
  }
  const getContributorPubkey = (nodeId: number) => {
    return PublicKey.findProgramAddressSync(
      [Buffer.from('contributor'), new BN(nodeId).toArrayLike(Buffer, 'le', 8)],
      program.programId
    )[0]
  }

  it('Should initialize state successfully', async () => {
    // Add your test here.
    const recorder = PublicKey.findProgramAddressSync(
      [Buffer.from('state')],
      program.programId
    )[0]

    const payer = program.provider.publicKey
    const systemProgram = SystemProgram.programId

    const accounts = { recorder, payer, systemProgram }
    const tx = await program.methods.initialize().accounts(accounts).rpc()
    console.log('tx: ', tx)

    const recorderAccount = await program.account.recorder.fetch(recorder)

    expect(recorderAccount.totalBandwidth.toNumber()).to.eq(0)
  })

  it('Should record bandwidth first time', async () => {
    // Add your test here.
    const recorder = PublicKey.findProgramAddressSync(
      [Buffer.from('state')],
      program.programId
    )[0]

    const contributor = getContributorPubkey(10)

    const payer = program.provider.publicKey
    const systemProgram = SystemProgram.programId

    const recordBandwidthAccounts = {
      recorder,
      contributor,
      payer,
      systemProgram,
    }

    const params = {
      nodeId: new BN(10),
      bandwidth: new BN(11),
      timestamp: new BN(12),
    }
    const tx = await program.methods
      .recordBandwidth(params)
      .accounts(recordBandwidthAccounts)
      .rpc()

    console.log('tx: ', tx)

    const recorderAccount = await program.account.recorder.fetch(recorder)
    expect(recorderAccount.totalBandwidth.toNumber()).to.eq(11)

    const contributorAccount = await program.account.contributor.fetch(
      contributor
    )
    expect(contributorAccount.totalBandwidth.toNumber()).to.eq(11)
    expect(contributorAccount.lastTimestamp.toNumber()).to.eq(12)
  })

  it('Should record bandwidth second time', async () => {
    // Add your test here.
    const recorder = PublicKey.findProgramAddressSync(
      [Buffer.from('state')],
      program.programId
    )[0]

    const contributor = getContributorPubkey(20)

    const payer = program.provider.publicKey
    const systemProgram = SystemProgram.programId

    const recordBandwidthAccounts = {
      recorder,
      contributor,
      payer,
      systemProgram,
    }

    const params1 = {
      nodeId: new BN(20),
      bandwidth: new BN(11),
      timestamp: new BN(22),
    }
    const params2 = {
      nodeId: new BN(20),
      bandwidth: new BN(22),
      timestamp: new BN(33),
    }
    await program.methods
      .recordBandwidth(params1)
      .accounts(recordBandwidthAccounts)
      .rpc()
    const tx = await program.methods
      .recordBandwidth(params2)
      .accounts(recordBandwidthAccounts)
      .rpc()

    console.log('tx: ', tx)

    const recorderAccount = await program.account.recorder.fetch(recorder)
    expect(recorderAccount.totalBandwidth.toNumber()).to.eq(44) // 11 + 11 + 22

    const contributorAccount = await program.account.contributor.fetch(
      contributor
    )
    expect(contributorAccount.totalBandwidth.toNumber()).to.eq(33) // 11 + 22
    expect(contributorAccount.lastTimestamp.toNumber()).to.eq(33)
  })
})
