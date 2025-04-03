use anchor_lang::AnchorDeserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    compute_budget,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::VersionedTransaction,
};

use super::result::Result;
use super::sorted_signers::SortedSigners;

#[async_trait::async_trait]
pub trait Client {
    fn rpc_client(&self) -> &RpcClient;
    fn payer(&self) -> &Keypair;

    // Helper function to create an instance from the parts.
    fn from_parts(rpc_client: RpcClient, payer: Keypair) -> Self;

    async fn process_instruction(
        &self,
        instruction: Instruction,
        signers: &Vec<&Keypair>,
    ) -> Result<Signature> {
        self.process_instructions(&[instruction], signers).await
    }

    async fn process_instructions(
        &self,
        instructions: &[Instruction],
        signers: &[&Keypair], // accept a slice for more flexibility
    ) -> Result<Signature> {
        // Create the compute budget instruction.
        let compute_units_ix =
            compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(2_000_000);

        // Combine the compute budget instruction with the provided instructions.
        let mut all_instructions = Vec::with_capacity(instructions.len() + 1);
        all_instructions.push(compute_units_ix);
        all_instructions.extend_from_slice(instructions);

        let recent_blockhash = self.rpc_client().get_latest_blockhash().await?;

        // Combine the payer with the provided signers.
        let mut signers_with_payer = vec![self.payer()];
        signers_with_payer.extend_from_slice(signers);

        // Create a versioned transaction.
        let compiled_message = Message::try_compile(
            &self.payer().pubkey(),
            &all_instructions,
            &[], // TODO: add option to apply address_lookup
            recent_blockhash,
        )?;
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(compiled_message),
            &SortedSigners(&signers_with_payer),
        )?;

        let signature = self.rpc_client().send_and_confirm_transaction(&tx).await?;
        Ok(signature)
    }

    async fn get_account(&self, pubkey: &Pubkey) -> Result<Account> {
        let account = self.rpc_client().get_account(pubkey).await?;
        Ok(account)
    }

    async fn get_account_data<T: AnchorDeserialize>(&self, pubkey: &Pubkey) -> Result<T> {
        let account = self.get_account(pubkey).await?;

        Ok(T::deserialize(&mut &account.data[..])?)
    }
}
