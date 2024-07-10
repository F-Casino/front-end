use std::str::FromStr;

use leptos::{leptos_dom::logging::console_log, *};
use solana_client_wasm::{
    solana_sdk::{pubkey::Pubkey, system_instruction, transaction::Transaction},
    WasmClient,
};

use crate::phantom::PhantomWallet;

#[component]
pub fn UserInfo() -> impl IntoView {
    let wallet_resource = create_resource(|| (), |_| async { PhantomWallet::connect().await });

    view! {
        <div>
            <p>"User info"</p>
            <Suspense fallback=move || view! { <p>"Loading User"</p> }>
                // handles the error from the resource
                <ErrorBoundary fallback=|_| {
                    view! { <p>"Something went wrong"</p> }
                }>
                    {move || {
                        wallet_resource
                            .get()
                            .map(move |wallet| {
                                wallet
                                    .map(move |wallet| {
                                        view! {
                                            // successful call from the server fn
                                            <p>"Public key: " {wallet.public_key.to_string()}</p>
                                        }
                                    })
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
