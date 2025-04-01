import { Program } from '@coral-xyz/anchor'
import * as anchor from '@coral-xyz/anchor'
import { BwRecorder } from '../../target/types/bw_recorder'

describe('initialize', () => {
  const program = anchor.workspace.BwRecorder as Program<BwRecorder>

  it('Should run successfully', async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({}).rpc()
    console.log('Your transaction signature', tx)
  })
})
