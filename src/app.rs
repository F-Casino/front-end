use crate::phantom::PhantomWallet;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let user = create_resource(|| (), |_| async { PhantomWallet::connect().await });

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <div class="flex">
            <img class="w-10/12" src="public/asset/baucua.jpg"/>
            <div>
                <p>"Bet"</p>
                <button
                    type="button"
                    class="focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800"
                >"Add"</button>
            </div>
        </div>
        <div>
            <p>"User info"</p>
            <Suspense fallback=move || view! { <p>"Loading User"</p> }>
                // handles the error from the resource
                <ErrorBoundary fallback=|_| {
                    view! { <p>"Something went wrong"</p> }
                }>
                    {move || {
                        user.read()
                            .map(move |x| {
                                x.map(move |y| {
                                    view! {
                                        // successful call from the server fn
                                        <p>"Public key: " {y.public_key.to_string()}</p>
                                    }
                                })
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
