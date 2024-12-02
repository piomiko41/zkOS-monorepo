use std::assert_matches::assert_matches;

use alloy_primitives::{Address, U256};
use alloy_sol_types::{SolCall, SolValue};
use evm_utils::EvmRunner;
use rstest::rstest;
use shielder_circuits::consts::merkle_constants::{ARITY, NOTE_TREE_HEIGHT};
use shielder_rust_sdk::contract::ShielderContract::getMerklePathCall;

use crate::shielder::{
    calls::new_account_native,
    deploy::{deployment, Deployment},
    invoke_shielder_call,
};

pub fn get_merkle_args(
    shielder_address: Address,
    note_index: U256,
    evm: &mut EvmRunner,
) -> (U256, [[U256; ARITY]; NOTE_TREE_HEIGHT]) {
    let calldata = getMerklePathCall { id: note_index }.abi_encode();
    let result = evm
        .call(shielder_address, calldata, None, None)
        .expect("Call failed")
        .output;
    let decoded = <Vec<U256>>::abi_decode(&result, true).expect("Decoding failed");
    reorganize_merkle_path(decoded)
}

fn reorganize_merkle_path(merkle_path: Vec<U256>) -> (U256, [[U256; ARITY]; NOTE_TREE_HEIGHT]) {
    assert_eq!(merkle_path.len(), ARITY * NOTE_TREE_HEIGHT + 1);

    let root = *merkle_path.last().expect("Empty merkle path");

    let mut result = [[U256::ZERO; ARITY]; NOTE_TREE_HEIGHT];
    for (i, element) in merkle_path
        .into_iter()
        .enumerate()
        .take(ARITY * NOTE_TREE_HEIGHT)
    {
        result[i / ARITY][i % ARITY] = element;
    }

    (root, result)
}

#[rstest]
fn succeeds(mut deployment: Deployment) {
    assert!(new_account_native::create_account_and_call(
        &mut deployment,
        U256::from(1),
        U256::from(10)
    )
    .is_ok());

    let calldata = getMerklePathCall { id: U256::ZERO };
    let result = invoke_shielder_call(&mut deployment, &calldata, None);

    assert_matches!(result, Ok(_));
    assert!(result.unwrap().is_empty())
}
