#![no_std]

mod extensions;
mod test;
mod types;

use extensions::{env_extensions::EnvExtensions, u64_extensions::U64Extensions};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, BytesN, Env, Vec};
use types::asset::Asset;
use types::error::Error;
use types::{config_data::ConfigData, price_data::PriceData};

#[contract]
pub struct PriceOracleContract;

#[contractimpl]
impl PriceOracleContract {
    pub fn base(e: Env) -> Asset {
        e.get_base_asset()
    }

    pub fn decimals(e: Env) -> u32 {
        e.get_decimals()
    }

    pub fn resolution(e: Env) -> u32 {
        e.get_resolution() / 1000
    }

    pub fn period(e: Env) -> Option<u64> {
        e.get_retention_period()
    }

    pub fn assets(e: Env) -> Vec<Asset> {
        e.get_assets()
    }

    pub fn last_timestamp(e: Env) -> u64 {
        e.get_last_timestamp()
    }

    pub fn price(e: Env, asset: Asset, timestamp: u64) -> Option<PriceData> {
        let resolution = e.get_resolution();
        let normalized_timestamp = timestamp.get_normalized_timestamp(resolution.into());
        //get the price
        get_price_data(&e, asset, normalized_timestamp)
    }

    pub fn lastprice(e: Env, asset: Asset) -> Option<PriceData> {
        //get the last timestamp
        let timestamp = e.get_last_timestamp();
        //get the price
        get_price_data(&e, asset, timestamp)
    }

    pub fn version(_e: Env) -> u32 {
        env!("CARGO_PKG_VERSION")
            .split(".")
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap()
    }

    //Admin section

    pub fn admin(e: Env) -> Option<Address> {
        e.get_admin()
    }

    pub fn config(e: Env, admin: Address, config: ConfigData) {
        admin.require_auth();
        if e.is_initialized() {
            e.panic_with_error(Error::AlreadyInitialized);
        }
        e.set_admin(&config.admin);
        e.set_base_asset(&config.base_asset);
        e.set_decimals(config.decimals);
        e.set_resolution(config.resolution);
        e.set_retention_period(config.period);

        Self::__add_assets(&e, config.assets);
    }

    pub fn bump(e: Env, ledgers_to_live: u32) {
        e.bump(ledgers_to_live);
    }

    pub fn add_assets(e: Env, admin: Address, assets: Vec<Asset>) {
        e.panic_if_not_admin(&admin);
        Self::__add_assets(&e, assets);
    }

    pub fn set_period(e: Env, admin: Address, period: u64) {
        e.panic_if_not_admin(&admin);
        e.set_retention_period(period);
    }

    pub fn set_price(e: Env, admin: Address, updates: Vec<i128>, timestamp: u64) {
        e.panic_if_not_admin(&admin);

        let retention_period = e.get_retention_period().unwrap();

        let ledgers_to_live: u32 = ((retention_period / 1000 / 5) + 1) as u32;

        //get the last timestamp
        let last_timestamp = e.get_last_timestamp();

        //iterate over the updates
        for (i, price) in updates.iter().enumerate() {
            let asset = i as u8;
            //store the new price
            e.set_price(asset, price, timestamp, ledgers_to_live);
        }
        if timestamp > last_timestamp {
            e.set_last_timestamp(timestamp);
        }
    }

    pub fn update_contract(env: Env, admin: Address, wasm_hash: BytesN<32>) {
        env.panic_if_not_admin(&admin);
        env.deployer().update_current_contract_wasm(wasm_hash)
    }

    fn __add_assets(e: &Env, assets: Vec<Asset>) {
        let mut presented_assets = e.get_assets();

        let mut assets_indexes: Vec<(Asset, u32)> = Vec::new(&e);
        for asset in assets.iter() {
            //check if the asset has been already added
            if has_asset(&presented_assets, &asset) {
                panic_with_error!(&e, Error::AssetAlreadyPresented);
            }
            presented_assets.push_back(asset.clone());
            assets_indexes.push_back((asset, presented_assets.len() as u32 - 1));
        }

        e.set_assets(presented_assets);
        for (asset, index) in assets_indexes.iter() {
            e.set_asset_index(asset, index);
        }
    }
}

fn has_asset(assets: &Vec<Asset>, asset: &Asset) -> bool {
    for current_asset in assets.iter() {
        if &current_asset == asset {
            return true;
        }
    }
    false
}
fn get_price_data(e: &Env, asset: Asset, timestamp: u64) -> Option<PriceData> {
    let asset: Option<u8> = e.get_asset_index(asset);
    if asset.is_none() {
        return None;
    }
    get_price_data_by_index(e, asset.unwrap(), timestamp)
}

fn get_price_data_by_index(e: &Env, asset: u8, timestamp: u64) -> Option<PriceData> {
    let price = e.get_price(asset, timestamp);
    if price.is_none() {
        return None;
    }
    Some(PriceData {
        price: price.unwrap(),
        timestamp,
    })
}
