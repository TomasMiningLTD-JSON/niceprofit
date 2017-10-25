#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate config;

mod runner;
mod parser;
mod performance_calculator;
mod nicehash_api_raw;
mod nicehash_api;
mod benchmark;
mod nicehash_cpuminer_mapper;
mod algorithm_picker;

const USER_WALLET_ARG: &str = "wallet";
const DEV_WALLET: &str = "393EZrk5mwZ6gdVYmX5nguesVVJwxD9X2U";
const CPUMINER_MULTI_PATH: &str = "cpuminer";
const BENCHMARK_TIME_MS: u64 = 10000;
const LOCATION: &str = "eu";

fn main() {
    let clap_yaml = load_yaml!("clap.yml");
    let matches = App::from_yaml(clap_yaml).get_matches();

    let user_wallet = matches.value_of(&USER_WALLET_ARG).unwrap_or(&DEV_WALLET);

    let nicehash_response = nicehash_api::get_profitability().unwrap();
    let algorithms= nicehash_response.result.simplemultialgo;

    let benchmark = benchmark::benchmark(algorithms.clone(), LOCATION, BENCHMARK_TIME_MS, CPUMINER_MULTI_PATH, DEV_WALLET);

    println!("{:#?}", benchmark);

    let best_algorithm = algorithm_picker::pick_cpuminer_algorithm(algorithms, benchmark.clone());

    println!("{:#?}", best_algorithm);
}
