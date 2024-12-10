use core::fmt;
use std::{env, fs::File, io::Write};

use alloy_primitives::U256;
use integration_tests::{
    calls::{
        deposit_native::{
            invoke_call as deposit_native_call, prepare_call as deposit_native_calldata,
        },
        new_account_native::{
            invoke_call as new_account_native_call, prepare_call as new_account_native_calldata,
        },
        withdraw_native::{
            invoke_call as withdraw_native_call, prepare_args,
            prepare_call as withdraw_native_calldata,
        },
    },
    deploy::{deployment, Deployment},
    deposit_native_proving_params, new_account_native_proving_params,
    withdraw_native_proving_params,
};
use shielder_rust_sdk::{
    account::ShielderAccount,
    contract::ShielderContract::{depositNativeCall, newAccountNativeCall, withdrawNativeCall},
};

#[derive(Debug)]
enum Calldata {
    NewAccount(newAccountNativeCall),
    Deposit(depositNativeCall),
    Withdraw(withdrawNativeCall),
}

impl fmt::Display for Calldata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Calldata::NewAccount(_) => write!(f, "NewAccountNative"),
            Calldata::Deposit(_) => write!(f, "DepositNative"),
            Calldata::Withdraw(_) => write!(f, "WithdrawNative"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::create(filename).unwrap();

    let mut deployment = deployment(
        &new_account_native_proving_params(),
        &deposit_native_proving_params(),
        &withdraw_native_proving_params(),
    );

    let mut shielder_account = ShielderAccount::new(U256::from(1));
    let amount = U256::from(10);
    let calldata = Calldata::NewAccount(new_account_native_calldata(
        &mut deployment,
        &mut shielder_account,
        amount,
    ));

    let gas_used = measure_gas(&calldata, &mut deployment, &mut shielder_account);
    let mut content: Vec<u8> = vec![];

    content.extend(&mut format!("{}: {gas_used}\n", &calldata).as_bytes().iter());

    let calldata = Calldata::Deposit(
        deposit_native_calldata(&mut deployment, &mut shielder_account, amount).0,
    );

    let gas_used = measure_gas(&calldata, &mut deployment, &mut shielder_account);

    content.extend(&mut format!("{}: {gas_used}\n", &calldata).as_bytes().iter());

    let calldata = Calldata::Withdraw(
        withdraw_native_calldata(
            &mut deployment,
            &mut shielder_account,
            prepare_args(amount, U256::from(1)),
        )
        .0,
    );

    let gas_used = measure_gas(&calldata, &mut deployment, &mut shielder_account);

    content.extend(&mut format!("{}: {gas_used}\n", &calldata).as_bytes().iter());

    file.write_all(&content).unwrap();
}

fn measure_gas(
    calldata: &Calldata,
    deployment: &mut Deployment,
    shielder_account: &mut ShielderAccount,
) -> u64 {
    match calldata {
        Calldata::NewAccount(calldata) => {
            new_account_native_call(deployment, shielder_account, U256::from(10), calldata)
                .unwrap()
                .1
                .gas_used
        }
        Calldata::Deposit(calldata) => {
            deposit_native_call(deployment, shielder_account, U256::from(10), calldata)
                .unwrap()
                .1
                .gas_used
        }
        Calldata::Withdraw(calldata) => {
            withdraw_native_call(deployment, shielder_account, calldata)
                .unwrap()
                .1
                .gas_used
        }
    }
}
