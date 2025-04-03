use std::env;

#[derive(Clone)]
pub struct Config {
    pub job_schedule: String,
    pub bw_filename: String,
    pub rpc_url: String,
    pub payer_keypair_filename: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            job_schedule: env::var("JOB_SCHEDULE").unwrap(),
            bw_filename: env::var("BW_FILENAME").unwrap_or("./bw_file.json".to_string()),
            rpc_url: env::var("RPC_URL").unwrap(),
            payer_keypair_filename: env::var("PAYER_KEYPAIR_FILENAME")
                .unwrap_or("./payer.json".to_string()),
        }
    }
}
