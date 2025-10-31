//! This is an impl of the "Memory" DS

use alloy::primitives::U256;

#[derive(Debug, Default, Clone)]
pub struct Memory {
    /// Data string representing the memory of the EVM
    data: Vec<u8>,
}

impl Memory {
    /// Create a new memory instance.
    pub fn new() -> Self {
        Memory { data: Vec::new() }
    }

    /// Create new memory instance with data
    pub fn new_with_data(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Store 32bytes (word) to memory given an offset
    pub fn store_word(&mut self, offset: usize, word: U256) {
        let word_as_bytes = word.to_be_bytes::<32>(); // converts U256 to bytes
        self.data[offset..offset + 32].copy_from_slice(&word_as_bytes); // copies bytes to memory
    }

    /// Load 32bytes (word) from memory given an offset
    pub fn load_word(&self, offset: usize) -> U256 {
        let word_as_bytes: [u8; 32] = self.data[offset..offset + 32].try_into().unwrap();
        U256::from_be_bytes(word_as_bytes)
    }

    /// Function store a single bytes to memory given and offset
    pub fn store_byte(&mut self, offset: usize, byte: u8) {
        self.data[offset] = byte;
    }

    /// Function reads a single byte from memory given an offset
    pub fn load_byte(&self, offset: usize) -> u8 {
        self.data[offset]
    }
}

// /// Represents the EVM's linear memory.
// #[derive(Debug, Default)]
// pub struct Memory {
//     data: Vec<u8>,
// }

// impl Memory {
//     /// Creates a new, empty Memory instance.
//     pub fn new() -> Self {
//         Memory { data: Vec::new() }
//     }

//     /// Returns the current length of the memory in bytes.
//     pub fn len(&self) -> usize {
//         self.data.len()
//     }

//     /// Returns true if the memory is empty.
//     pub fn is_empty(&self) -> bool {
//         self.data.is_empty()
//     }

//     /// Returns a slice of the raw memory data.
//     pub fn data(&self) -> &[u8] {
//         &self.data
//     }

//     /// Accesses a slice of memory, returning a *new* Vec<u8>.
//     /// This mimics the Go implementation's `Access` function, which reads
//     /// with zero-padding *without* modifying the underlying memory.
//     /// This is typically used for read-only operations like `MLOAD`.
//     pub fn access(&self, offset: u64, size: u64) -> Vec<u8> {
//         if size == 0 {
//             return Vec::new();
//         }

//         let offset = offset as usize;
//         let size = size as usize;
//         let end = offset + size;

//         if self.len() < end {
//             // We are reading past the end of memory.
//             // Create a new buffer, copy what we have, and the rest will be zeros.
//             let mut cpy = vec![0; end];
//             let current_len = self.len();

//             if current_len > 0 {
//                 // Determine the portion of existing data to copy
//                 let copy_len = std::cmp::min(current_len, end);
//                 if offset < copy_len {
//                      let real_copy_len = std::cmp::min(current_len - offset, size);
//                      cpy[offset..offset + real_copy_len].copy_from_slice(&self.data[offset..offset + real_copy_len]);
//                 }
//             }
//             // Return the requested slice from the zero-padded copy
//             cpy[offset..end].to_vec()
//         } else {
//             // We have enough data, just return a copy of the slice
//             self.data[offset..end].to_vec()
//         }
//     }

//     /// Loads a 32-byte word from memory.
//     /// This is a convenience wrapper for `access(offset, 32)`.
//     pub fn load(&self, offset: u64) -> Vec<u8> {
//         self.access(offset, 32)
//     }

//     /// Stores a slice of bytes into memory at a given offset.
//     /// This function *will* expand memory if necessary and returns the gas cost
//     /// for any expansion.
//     ///
//     /// NOTE: This implementation corrects a bug in the provided Go code
//     /// where storing to an empty memory incorrectly handled the offset.
//     pub fn store(&mut self, offset: u64, value: &[u8]) -> u64 {
//         if value.is_empty() {
//             return 0; // No cost for storing nothing
//         }

//         let current_mem_size = self.len() as u64;
//         let current_cost = calc_memory_gas_cost(current_mem_size);

//         let new_mem_size = offset + value.len() as u64;
//         let mut expansion_cost = 0;

//         if current_mem_size < new_mem_size {
//             // Need to expand memory
//             self.data.resize(new_mem_size as usize, 0);

//             // Calculate new cost based on the *new* memory length
//             let new_cost = calc_memory_gas_cost(self.len() as u64);
//             expansion_cost = new_cost - current_cost;
//         }

//         // Copy the value into the memory
//         let offset = offset as usize;
//         self.data[offset..offset + value.len()].copy_from_slice(value);

//         expansion_cost
//     }

//     /// Stores a 32-byte word (or a slice up to 32 bytes) into memory.
//     /// This function *will* expand memory if necessary and returns the gas cost
//     /// for any expansion.
//     ///
//     /// NOTE: This implementation corrects a bug in the provided Go code
//     /// where storing to an empty memory incorrectly handled the offset.
//     pub fn store32(&mut self, offset: u64, value: &[u8]) -> u64 {
//         let current_mem_size = self.len() as u64;
//         let current_cost = calc_memory_gas_cost(current_mem_size);

//         let new_mem_size = offset + 32;
//         let mut expansion_cost = 0;

//         if current_mem_size < new_mem_size {
//             // Need to expand memory
//             self.data.resize(new_mem_size as usize, 0);

//             // Calculate new cost based on the *new* memory length
//             let new_cost = calc_memory_gas_cost(self.len() as u64);
//             expansion_cost = new_cost - current_cost;
//         }

//         // Copy the value. Go's `copy(dst, src)` copies `min(len(dst), len(src))`.
//         // We do the same: copy up to 32 bytes from `value`.
//         let offset = offset as usize;
//         let len_to_copy = std::cmp::min(value.len(), 32);

//         self.data[offset..offset + len_to_copy].copy_from_slice(&value[..len_to_copy]);
//         // Any remaining bytes in the 32-byte slot (from `offset + len_to_copy`
//         // to `offset + 32`) will remain 0 from the `resize`.

//         expansion_cost
//     }
// }
