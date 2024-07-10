use crate::component::bet::Bet;
use crate::component::user_info::UserInfo;
use crate::phantom::PhantomWallet;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <div class="flex">
            <img class="w-10/12" src="public/asset/baucua.jpg"/>
            <Bet user_public_key={"7fV9DdpCDgJ3a4RpG1nYKK7gRgHRH19RQy1vNmPp76CT".to_string()}/>
        </div>
        <UserInfo/>
    }
}
