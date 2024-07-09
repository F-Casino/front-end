use leptos::*;

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
                            .read()
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
