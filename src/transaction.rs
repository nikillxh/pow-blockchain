#[derive(Debug)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[derive(Debug)]
pub struct TxInput {
    pub previous_output: String,
}

#[derive(Debug)]
pub struct TxOutput {
    pub recipient: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Transaction { inputs, outputs }
    }

    pub fn new_coinbase(miner_address: String, reward: u64) -> Self {
        Transaction {
            inputs: vec![],
            outputs: vec![TxOutput{
                recipient: miner_address,
                amount: reward,
            }]
        }
    }

    pub fn calculate_fee(&self, input_total: u64) -> u64 {
        let output_total: u64 = self.outputs.iter().map(|o| o.amount).sum();
        input_total.saturating_sub(output_total)
    }
}