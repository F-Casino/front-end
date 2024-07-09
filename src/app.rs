use crate::phantom::PhantomWallet;
use leptos::*;
use leptos_meta::*;
use crate::component::user_info::UserInfo;
use crate::component::bet::Bet;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let user = create_resource(|| (), |_| async { PhantomWallet::connect().await });

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <div class="flex">
            <img class="w-10/12" src="public/asset/baucua.jpg"/>
            <Bet/>
        </div>
        <UserInfo/>
    }
}
