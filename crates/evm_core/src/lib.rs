pub mod jump_table;
pub mod opcodes;
pub mod operations;

use alloy::primitives::Address;
use primitives::{
    evm_types::{BlockEnv, EvmStorage, Transaction},
    memory::Memory,
    stack::Stack,
};

use crate::{jump_table::build_jump_table, opcodes::Opcode};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ProgramExitStatus {
    Success,
    Failure,
    #[default]
    Default,
}

#[derive(Debug, Clone, Default)]
pub struct Evm {
    pub block_env: BlockEnv,
    pub tx: Transaction,
    pub memory: Memory,
    pub stack: Stack,
    pub storage: EvmStorage,
    pub pc: usize,
    pub status: ProgramExitStatus,
}

impl Evm {
    /// Function to create an Instance of the evm
    pub fn new(
        block_env: BlockEnv,
        tx: Transaction,
        memory: Memory,
        stack: Stack,
        storage: EvmStorage,
    ) -> Self {
        Self {
            block_env,
            tx,
            memory,
            stack,
            storage,
            pc: 0,
            status: ProgramExitStatus::default(),
        }
    }

    /// Function to setup evm for execution
    /// Note: This EVM to not work with smart contract using logic in the recieve function
    pub fn setup(&mut self) {
        // check is the is a deploy action (is to empty and )
        if self.tx.to == Address::ZERO && !self.tx.data.is_empty() {
            // load initcode from calldata to memory
            for (i, d) in self.tx.data.iter().enumerate() {
                self.memory.store_byte(i, *d);
            }

            self.run();
        } else if self.tx.to != Address::ZERO {
            // load smart contract code from storage to memory
            let touched_contract = self.tx.to;
            for (i, d) in self
                .storage
                .data
                .get(&touched_contract)
                .unwrap()
                .code
                .iter()
                .enumerate()
            {
                self.memory.store_byte(i, *d);
            }

            self.run();
        } else {
        }
    }

    /// step function for evm execution
    fn step(&mut self) {
        let raw_instruction = self.memory.load_byte(self.pc);
        let instruction = Opcode::from_u8(raw_instruction).unwrap(); // Handle this error later: (InvalidOpcode)

        let jump_table = build_jump_table();
        jump_table[instruction as usize](self);
    }

    /// Function to run stored bytecode in evm memory
    pub fn run(&mut self) {
        while self.status == ProgramExitStatus::Default {
            self.step();
        }
    }
}
