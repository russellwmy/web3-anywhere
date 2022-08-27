#![allow(non_snake_case)]

use dioxus::prelude::*;
use web3_anywhere::{
    key_man::{KeyStore, Signer},
    near::{Wallet, WalletConfig},
};

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut wallet = connect_wallet();
    // call this before init connect state
    wallet.complete_sign_in_with_access_key();

    let connected = use_state(&cx, || wallet.is_signed_in());
    let username = wallet.get_account_id();

    let handle_click = move |_| {
        let mut wallet = wallet.to_owned();
        match connected.get() {
            true => {
                let connected = connected.clone();
                wallet.sign_out();
                connected.set(false);
            }
            false => {
                let connected = connected.clone();
                cx.spawn({
                    async move {
                        wallet
                            .request_sign_in(
                                // contract_id
                                Some("test.testnet".to_string()),
                                // method_names
                                Some(vec![]),
                                //success_url
                                None,
                                // failure_url
                                None,
                            )
                            .await;
                        connected.set(true);
                    }
                });
            }
        }
    };

    let button_text = match connected.get() {
        true => username.unwrap().to_string(),
        false => "connect".to_string(),
    };
    let button_icon = match connected.get() {
        true => rsx!(i {
            class: "fa-solid fa-link-slash"
        }),
        false => rsx!(i {
            class: "fa-solid fa-link"
        }),
    };

    cx.render(rsx!(
       div {
            class: "w-screen h-screen flex items-center justify-center",
           button {
                class: "btn primary normal-case",
                onclick: handle_click,
                span {
                    class: "mr-2",
                    button_icon,
                },
                "{button_text}"

           }
       }
    ))
}

pub fn connect_wallet() -> Wallet {
    let key_store = KeyStore::new_browser_local_storage_key_store();
    let signer = Signer::new_in_memory_signer(key_store.clone());
    let mut config = WalletConfig::new(key_store);

    config.network = Some("testnet".to_string());
    config.wallet_url = Some("https://wallet.testnet.near.org".to_string());
    config.helper_url = Some("https://helper.testnet.near.org".to_string());
    config.explorer_url = Some("https://explorer.testnet.near.org".to_string());
    config.signer = Some(signer);

    let wallet = Wallet::new(config);
    log::info!("connected wallet");
    wallet
}
