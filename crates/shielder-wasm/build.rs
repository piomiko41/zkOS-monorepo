use rand::RngCore;
/// This script builds artifacts, which are later
/// embedded into the wasm binary.
/// To speedup the build process, we cache the artifacts after the first build.
///
/// When working locally, the `artifacts/` directory should be cleaned after the circuits are changed.
use shielder_circuits::circuits::merkle::MerkleCircuit;
use shielder_circuits::{circuits::Params, merkle::MerkleProverKnowledge, ProverKnowledge};
/// This script builds artifacts, which are later
/// embedded into the wasm binary.
/// To speedup the build process, we cache the artifacts after the first build.
///
/// When working locally, the `artifacts/` directory should be cleaned after the circuits are changed.
use shielder_circuits::{
    consts::{merkle_constants::NOTE_TREE_HEIGHT, RANGE_PROOF_CHUNK_SIZE},
    deposit::DepositCircuit,
    generate_keys_with_min_k,
    marshall::{marshall_params, marshall_path, marshall_pk},
    new_account::NewAccountCircuit,
    withdraw::WithdrawCircuit,
    Circuit, F, MAX_K,
};
use shielder_rust_sdk::{
    parameter_generation,
    powers_of_tau::{get_ptau_file_path, read as read_setup_parameters, Format},
};

/// This function is used to generate the artifacts for the circuit, i.e. hardcoded keys
/// and parameters. Saves results to `params.bin` and `pk.bin`.
fn gen_params_pk<C>(circuit_name: &str, full_params: &Params)
where
    C: Circuit<F> + Default,
{
    std::fs::create_dir_all(format!("artifacts/{}", circuit_name))
        .expect("Failed to create directory");
    let (params, k, pk, _) = generate_keys_with_min_k::<C>(full_params.clone())
        .expect("keys should not fail to generate");
    let params_bytes = marshall_params(&params).expect("Failed to marshall params");
    std::fs::write(
        format!("artifacts/{}/params.bin", circuit_name),
        params_bytes,
    )
    .expect("Failed to write params.bin");
    let key_bytes = marshall_pk(k, &pk).expect("Failed to marshall pk");
    std::fs::write(format!("artifacts/{}/pk.bin", circuit_name), key_bytes)
        .expect("Failed to write pk.bin");
}

/// This function is used to generate the artifacts for the DepositCircuit
fn gen_deposit(full_params: &Params) {
    gen_params_pk::<DepositCircuit<F, RANGE_PROOF_CHUNK_SIZE>>("deposit", full_params);
}

/// This function is used to generate the artifacts for the NewAccountCircuit
fn generate_new_account(full_params: &Params) {
    gen_params_pk::<NewAccountCircuit<F>>("new_account", full_params);
}

/// This function is used to generate the artifacts for the WithdrawCircuit
fn generate_withdraw(full_params: &Params) {
    gen_params_pk::<WithdrawCircuit<F, RANGE_PROOF_CHUNK_SIZE>>("withdraw", full_params);
}

/// This function is used to generate the artifacts for the MerkleCircuit
/// and the path to the root, leaf and path.
fn gen_merkle(full_params: &Params, rng: &mut impl RngCore) {
    gen_params_pk::<MerkleCircuit<NOTE_TREE_HEIGHT, F>>("merkle", full_params);

    let merkle_knowledge =
        MerkleProverKnowledge::<NOTE_TREE_HEIGHT, F>::random_correct_example(rng);
    let path_bytes = marshall_path(&merkle_knowledge.leaf, &merkle_knowledge.path);
    std::fs::write("artifacts/merkle/path.bin", path_bytes).expect("Failed to write path.bin");
}

fn main() {
    let mut rng = parameter_generation::rng();

    let full_params = read_setup_parameters(
        get_ptau_file_path(MAX_K, Format::PerpetualPowersOfTau),
        Format::PerpetualPowersOfTau,
    )
    .expect("failed to read parameters from the ptau file");

    gen_deposit(&full_params);
    generate_new_account(&full_params);
    generate_withdraw(&full_params);
    gen_merkle(&full_params, &mut rng);
    println!("cargo:rerun-if-changed=../shielder-circuits");
}