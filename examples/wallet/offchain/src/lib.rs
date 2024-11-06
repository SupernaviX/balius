use balius_sdk::{Ack, WorkerResult};
use balius_sdk::{Config, FnHandler, Params, Utxo, Worker};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct WalletConfig {
    address: String,
}

struct BalanceRequest {}

#[derive(Serialize, Deserialize, Clone)]
struct Datum {}

fn handle_utxo(config: Config<WalletConfig>, utxo: Utxo<Datum>) -> WorkerResult<Ack> {
    Ok(Ack)
}

#[balius_sdk::main]
fn main() -> Worker {
    Worker::new().with_utxo_handler(
        balius_sdk::wit::balius::app::driver::UtxoPattern {
            address: None,
            token: None,
        },
        FnHandler::from(handle_utxo),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use balius_sdk::txbuilder::{primitives, Address, Hash, UtxoSet};

    use std::{collections::HashMap, str::FromStr as _};

    #[test]
    fn test_happy_path() {
        let output = primitives::MintedTransactionOutput::PostAlonzo(primitives::MintedPostAlonzoTransactionOutput {
            address: Address::from_bech32("addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x").unwrap().to_vec().into(),
            value: primitives::Value::Coin(5_000_000),
            datum_option: None,
            script_ref: None,
        });

        let cbor = pallas_codec::minicbor::to_vec(&output).unwrap();

        let test_utxos: HashMap<_, _> = vec![(
            "f7d3837715680f3a170e99cd202b726842d97f82c05af8fcd18053c64e33ec4f#0"
                .parse()
                .unwrap(),
            cbor,
        )]
        .into_iter()
        .collect();

        let config = WalletConfig {
            address: "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x".into(),
        };

        handle_utxo(config, utxo).unwrap();
    }
}
