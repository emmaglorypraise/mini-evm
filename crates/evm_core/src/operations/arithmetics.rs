use alloy::primitives::{I256, U256};

use crate::{Evm, ProgramExitStatus};

pub fn stop(evm: &mut Evm) {
    evm.status = ProgramExitStatus::Success;
}

pub fn add(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a + b).unwrap();
}

pub fn mul(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a * b).unwrap();
}

pub fn sub(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a - b).unwrap();
}

pub fn div(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a / b).unwrap();
    }
}

pub fn sdiv(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());

    if b_int == I256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = a_int / b_int;
        let result_unsigned = U256::from_limbs(*result.as_limbs());
        evm.stack.push(result_unsigned).unwrap();
    }
}

pub fn modulo(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a % b).unwrap();
    }
}

pub fn smod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());

    if b_int == I256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = a_int % b_int;
        let result_unsigned = U256::from_limbs(*result.as_limbs());
        evm.stack.push(result_unsigned).unwrap();
    }
}

pub fn addmod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let c = evm.stack.pop().unwrap();

    if c == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = (a + b) % c;
        evm.stack.push(result).unwrap();
    }
}

pub fn mulmod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let c = evm.stack.pop().unwrap();

    if c == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = (a * b) % c;
        evm.stack.push(result).unwrap();
    }
}

pub fn exp(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    
    evm.stack.push(a.pow(b)).unwrap();
}

pub fn signextend(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = if a.to_be_bytes::<32>()[0] == 0 {
            a
        } else {
            a & U256::MAX // << (256 - b.as_u64()) // TODO: Implement sign extension completely
        };
        evm.stack.push(result).unwrap();
    }
}


pub fn and(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    
    evm.stack.push(a & b).unwrap();
}

pub fn byte(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    
    if a.as_limbs()[0] > 32 {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let retrived = b.to_be_bytes::<32>()[a.as_limbs()[0] as usize];
        evm.stack.push(U256::from(retrived)).unwrap();
    }
}

pub fn mstore(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();

    evm.memory.store_word(offset.as_limbs()[0] as usize, value);
}
