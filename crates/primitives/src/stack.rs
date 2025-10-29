//! This is the implementation of the Stack
//! 
//! 

use alloy::primitives::B256;

use crate::{constants::MAX_STACK_DEPTH, errors::EvmErrors};

#[derive(Debug, Default, Clone)]
pub struct Stack {
  data: Vec<B256>
}

impl Stack {
    /// Function introduces a new item to the top of the stack
    pub fn push( &mut self, item: B256) -> Result<(), EvmErrors> {
      if self.data.len() >= MAX_STACK_DEPTH as usize {
        return Err(EvmErrors::StackOverFlow);
      }

      Ok(self.data.push(item))

    }

    /// Function removes the last item in the stack and return this item 
    pub fn pop(&mut self) -> Option<B256> {
      return self.data.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::MAX_STACK_DEPTH;
    use crate::errors::EvmErrors;
    use alloy::primitives::B256;

    fn sample_b256(val: u8) -> B256 {
        B256::from_slice(&[val])
    }

    #[test]
    fn test_push_and_pop_single_item() {
        let mut stack = Stack::default();
        let item = sample_b256(1);
        assert!(stack.push(item).is_ok());
        assert_eq!(stack.pop(), Some(item));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_underflow() {
        let mut stack = Stack::default();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::default();
        let item = sample_b256(2);
        for _ in 0..MAX_STACK_DEPTH {
            assert!(stack.push(item).is_ok());
        }
        // Next push should fail
        let result = stack.push(item);
        assert!(matches!(result, Err(EvmErrors::StackOverFlow)));
    }

    #[test]
    fn test_lifo_order() {
        let mut stack = Stack::default();
        let first = sample_b256(10);
        let second = sample_b256(20);
        stack.push(first).unwrap();
        stack.push(second).unwrap();
        assert_eq!(stack.pop(), Some(second));
        assert_eq!(stack.pop(), Some(first));
        assert_eq!(stack.pop(), None);
    }
}