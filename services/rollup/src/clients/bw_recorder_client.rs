use std::{collections::HashMap, time::Duration};

use super::{client::Client, result::Result};
use anchor_lang::{prelude::Pubkey, system_program, Id, InstructionData, ToAccountMetas};
use bw_recorder::{accounts, instruction, program, RecordBandwidthParams, Recorder, ID};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    native_token::LAMPORTS_PER_SOL,
    signature::Keypair,
    signer::{EncodableKey, Signer},
};
use tracing::info;

pub struct BwRecorderClient {
    rpc_client: RpcClient,
    payer: Keypair,
}

impl Client for BwRecorderClient {
    fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    fn payer(&self) -> &Keypair {
        &self.payer
    }

    fn from_parts(rpc_client: RpcClient, payer: Keypair) -> Self {
        Self { rpc_client, payer }
    }
}

impl BwRecorderClient {
    pub fn new(rpc_url: String, payer_keypair_filename: String) -> Self {
        BwRecorderClient::from_parts(
            RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed()),
            Keypair::read_from_file(payer_keypair_filename).unwrap(),
        )
    }

    pub async fn initialize_contract(&self) -> Result<String> {
        let ix: Instruction = self.create_initialize_ix();

        let tx = self.process_instruction(ix, &vec![]).await?;

        Ok(tx.to_string())
    }

    pub async fn airdrop_payer(&self) -> Result<()> {
        let balance = self.rpc_client().get_balance(&self.payer.pubkey()).await?;
        info!("Payer balance: {}", balance);
        if balance > LAMPORTS_PER_SOL {
            return Ok(());
        }

        self.rpc_client()
            .request_airdrop(&self.payer.pubkey(), 3 * LAMPORTS_PER_SOL)
            .await?;

        let mut new_balance = self.rpc_client().get_balance(&self.payer.pubkey()).await?;
        while new_balance <= balance {
            std::thread::sleep(Duration::from_millis(1000));
            new_balance = self.rpc_client().get_balance(&self.payer.pubkey()).await?;
        }

        info!("Payer new balance: {}", new_balance);

        Ok(())
    }

    pub async fn record_bandwidth(
        &self,
        records: HashMap<u64, u64>,
        timestamp: u64,
    ) -> Result<String> {
        let ixs: Vec<Instruction> = records
            .iter()
            .map(|(node_id, bandwidth)| {
                self.create_record_bandwidth_ix(*node_id, *bandwidth, timestamp)
            })
            .collect();

        let tx = self.process_instructions(&ixs, &[]).await?;

        Ok(tx.to_string())
    }

    pub async fn get_recorder(&self) -> Result<Recorder> {
        let recorder = self.get_account_data(&self.get_recorder_pubkey()).await?;

        Ok(recorder)
    }

    pub async fn is_initialized(&self) -> Result<bool> {
        let ret = self.get_recorder().await;

        Ok(ret.is_ok())
    }

    pub fn get_program_id(&self) -> Pubkey {
        program::BwRecorder::id()
    }

    pub fn get_recorder_pubkey(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"state"], &self.get_program_id()).0
    }

    pub fn get_contributor_pubkey(&self, node_id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[b"contributor", bytemuck::bytes_of(&node_id)],
            &self.get_program_id(),
        )
        .0
    }

    fn create_record_bandwidth_ix(
        &self,
        node_id: u64,
        bandwidth: u64,
        timestamp: u64,
    ) -> Instruction {
        let params = RecordBandwidthParams {
            node_id,
            bandwidth,
            timestamp,
        };

        let accounts = accounts::RecordBandwidth {
            recorder: self.get_recorder_pubkey(),
            contributor: self.get_contributor_pubkey(node_id),
            payer: self.payer().pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None);

        Instruction {
            program_id: ID,
            accounts,
            data: instruction::RecordBandwidth { params }.data(),
        }
    }

    fn create_initialize_ix(&self) -> Instruction {
        let accounts = accounts::Initialize {
            recorder: self.get_recorder_pubkey(),
            payer: self.payer().pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None);

        Instruction {
            program_id: ID,
            accounts,
            data: instruction::Initialize {}.data(),
        }
    }
}
