import { AnchorProvider, setProvider } from '@coral-xyz/anchor'

before(async () => {
  const provider = AnchorProvider.local(undefined, {
    commitment: 'confirmed',
    preflightCommitment: 'confirmed',
  })
  setProvider(provider)
})
