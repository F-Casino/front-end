use std::str::FromStr;

use leptos::*;
use serde::{Deserialize, Serialize};
use solana_client_wasm::solana_sdk::{pubkey::Pubkey, transaction::Transaction};
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhantomResult {
    #[serde(rename_all = "camelCase")]
    Connect {
        public_key: String,
    },
    Disconnect,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhantomResponse {
    id: u32,
    jsonrpc: String,
    result: PhantomResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method")]
pub enum PhantomRequest {
    Connect,
    SignTransaction { params: PhantomMethodParams },
    SignAndSendTransaction { params: PhantomMethodParams },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhantomMethodParams {
    message: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PhantomParams {
//     method: PhantomMethod,
//     params: Option<PhantomMethodParams>,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhantomStatus {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhantomWallet {
    pub status: PhantomStatus,
    pub public_key: Pubkey,
}

impl Default for PhantomWallet {
    fn default() -> PhantomWallet {
        PhantomWallet {
            status: PhantomStatus::Disconnected,
            public_key: Pubkey::default(),
        }
    }
}

impl PhantomWallet {
    // async fn request(request: PhantomRequest) -> anyhow::Result<JsValue> {
    //     let window = web_sys::window().unwrap();
    //     if let Some(solana) = window.get("solana") {
    //         let handle_message_str = wasm_bindgen::JsValue::from_str("_handleMessage");
    //         let handle_message_method: js_sys::Function =
    //             js_sys::Reflect::get(&*solana, &handle_message_str)
    //                 .unwrap()
    //                 .into();
    //
    //         window
    //             .remove_event_listener_with_callback("message", &handle_message_method)
    //             .unwrap();
    //         window
    //             .add_event_listener_with_callback("message", &handle_message_method)
    //             .unwrap();
    //         let is_phantom =
    //             js_sys::Reflect::get(&*solana, &wasm_bindgen::JsValue::from_str("isPhantom"))
    //                 .unwrap();
    //         if is_phantom == JsValue::from(true) {
    //             let request_str = wasm_bindgen::JsValue::from_str("request");
    //             let request_method: js_sys::Function =
    //                 js_sys::Reflect::get(&*solana, &request_str).unwrap().into();
    //
    //             let value = serde_wasm_bindgen::to_value(&request).unwrap();
    //
    //             let resp = request_method.call1(&solana, &value).unwrap();
    //             let promise = js_sys::Promise::resolve(&resp);
    //             let result = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    //
    //             Ok(result)
    //         } else {
    //             anyhow::bail!("Wallet not found");
    //         }
    //     } else {
    //         anyhow::bail!("Wallet not found");
    //     }
    // }
    //
    pub async fn connect() -> Result<PhantomWallet, ServerFnError> {
        let window = web_sys::window().unwrap();
        if let Some(solana) = window.get("solana") {
            let is_phantom =
                js_sys::Reflect::get(&*solana, &wasm_bindgen::JsValue::from_str("isPhantom"))
                    .unwrap();
            if is_phantom == JsValue::from(true) {
                let connect_str = wasm_bindgen::JsValue::from_str("connect");
                let connect: js_sys::Function =
                    js_sys::Reflect::get(&*solana, &connect_str).unwrap().into();
                let resp = connect.call0(&solana).unwrap();

                let promise = js_sys::Promise::resolve(&resp);
                let result = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                let pubkey_str = wasm_bindgen::JsValue::from_str("publicKey");
                let pubkey_obj: js_sys::Object =
                    js_sys::Reflect::get(&result, &pubkey_str).unwrap().into();

                let bn_str = wasm_bindgen::JsValue::from_str("toString");
                let to_string_fn: js_sys::Function =
                    js_sys::Reflect::get(&pubkey_obj, &bn_str).unwrap().into();

                let pubkey = to_string_fn.call0(&pubkey_obj).unwrap();
                let public_key = Pubkey::from_str(&pubkey.as_string().unwrap()).unwrap();
                PhantomWallet::is_connected();
                logging::log!("{}", public_key.to_string());
                Ok(PhantomWallet {
                    status: PhantomStatus::Connected,
                    public_key,
                })
            } else {
                return Err(ServerFnError::ServerError("wallet not found".to_string()));
            }
        } else {
            return Err(ServerFnError::ServerError("wallet not found".to_string()));
        }
    }
    // fn disconnect(ctx: ScopeRef<'_>) -> Result<(), Error> {
    //     let window = web_sys::window().unwrap();
    //     if let Some(solana) = window.get("solana") {
    //         let this = JsValue::null();
    //         let disconnect_str = wasm_bindgen::JsValue::from_str("disconnect");
    //         let disconnect: js_sys::Function = js_sys::Reflect::get(&*solana, &disconnect_str)
    //             .unwrap()
    //             .into();
    //         let resp = disconnect.call0(&this).unwrap();
    //         ctx.spawn_local(async move {
    //             let promise = js_sys::Promise::resolve(&resp);
    //             wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    //             PhantomWallet::is_connected();
    //             reducer(ctx, Action::WalletSet(PhantomWallet::default()))
    //         });
    //         Ok(())
    //     } else {
    //         Err(Error::PhantomWalletNotFound)
    //     }
    // }
    pub fn is_connected() -> bool {
        let window = web_sys::window().unwrap();
        if let Some(solana) = window.get("solana") {
            let is_connected_str = wasm_bindgen::JsValue::from_str("isConnected");
            let is_connected = js_sys::Reflect::get(&solana, &is_connected_str)
                .unwrap()
                .as_bool()
                .unwrap();
            is_connected
        } else {
            false
        }
    }

    fn pubkey(&self) -> Result<PhantomWallet, ServerFnError> {
        #[allow(unused_assignments)]
        let mut is_connected = false;
        let mut public_key = Pubkey::default();
        let window = web_sys::window().unwrap();
        if let Some(solana) = window.get("solana") {
            let is_connected_str = wasm_bindgen::JsValue::from_str("isConnected");
            is_connected = js_sys::Reflect::get(&solana, &is_connected_str)
                .unwrap()
                .as_bool()
                .unwrap();
            // log::debug!("is_connected: {:?}", is_connected);
            if is_connected {
                let pubkey_str = wasm_bindgen::JsValue::from_str("publicKey");
                let pubkey_obj: js_sys::Object =
                    js_sys::Reflect::get(&solana, &pubkey_str).unwrap().into();

                let bn_str = wasm_bindgen::JsValue::from_str("toString");
                let to_string_fn: js_sys::Function =
                    js_sys::Reflect::get(&pubkey_obj, &bn_str).unwrap().into();

                // log::debug!("pubkey_obj: {:?}", to_string_fn.call0(&pubkey_obj));
                if let Ok(pubkey) = to_string_fn.call0(&pubkey_obj) {
                    public_key = Pubkey::from_str(&pubkey.as_string().unwrap()).unwrap();
                    // log::debug!("pubkey: {:?}", public_key);
                };
            }
        } else {
                return Err(ServerFnError::ServerError("wallet not found".to_string()));
        }

        Ok(PhantomWallet {
            status: PhantomStatus::Connected,
            public_key,
        })
    }
    //
    // pub fn sign_transaction_method() -> Result<(), Error> {
    //     let window = web_sys::window().unwrap();
    //     if let Some(solana) = window.get("solana") {
    //         let this = JsValue::null();
    //         let sign_trans_str = wasm_bindgen::JsValue::from_str("signTransaction");
    //         let sign_trans_method: js_sys::Function =
    //             js_sys::Reflect::get(&*solana, &sign_trans_str)
    //                 .unwrap()
    //                 .into();
    //         // let resp = sign_trans.call0(&this).unwrap();
    //         log::debug!("sign_trans_method{:?}", sign_trans_method.to_string());
    //         log::debug!(
    //             "sign_transaction: {:?}",
    //             js_sys::Object::get_own_property_names(&solana)
    //         );
    //         Ok(())
    //     } else {
    //         Err(Error::PhantomWalletNotFound)
    //     }
    // }
    // pub fn sign_transaction(ctx: ScopeRef<'_>, transaction: Transaction) -> Result<(), Error> {
    //     let wallet_signal = ctx.use_context::<Signal<PhantomWallet>>();
    //     let wallet = wallet_signal.get();
    //     if wallet.status == PhantomStatus::Disconnected {
    //         let params = PhantomRequest::SignTransaction {
    //             params: PhantomMethodParams {
    //                 message: "dingus".to_string(),
    //             },
    //         };
    //     } else {
    //     }
    //     Ok(())
    // }
    //
    // pub fn create_transfer_transaction(
    //     ctx: ScopeRef<'_>,
    //     to: &Pubkey,
    //     lamports: u64,
    // ) -> Result<(), Error> {
    //     let wallet_signal = ctx.use_context::<Signal<PhantomWallet>>();
    //     let wallet = wallet_signal.get();
    //     if wallet.status == PhantomStatus::Disconnected {
    //         let params = PhantomRequest::SignTransaction {
    //             params: PhantomMethodParams {
    //                 message: "dingus".to_string(),
    //             },
    //         };
    //     } else {
    //     }
    //     Ok(())
    // }
}
