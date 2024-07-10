use std::collections::HashMap;

use crate::{config, phantom::PhantomWallet};
use leptos::*;
use leptos_dom::helpers::location;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum BetKind {
    Fish,
    Prawn,
    Crab,
    Cock,
    Calabash,
    Tiger,
}

impl From<BetKind> for String {
    fn from(value: BetKind) -> String {
        match value {
            BetKind::Fish => "Fish".to_string(),
            BetKind::Prawn => "Prawn".to_string(),
            BetKind::Crab => "Crab".to_string(),
            BetKind::Cock => "Cock".to_string(),
            BetKind::Calabash => "Calabash".to_string(),
            BetKind::Tiger => "Tiger".to_string(),
        }
    }
}

impl From<String> for BetKind {
    fn from(value: String) -> BetKind {
        match value.as_ref() {
            "Fish" => BetKind::Fish,
            "Prawn" => BetKind::Prawn,
            "Crab" => BetKind::Crab,
            "Cock" => BetKind::Cock,
            "Calabash" => BetKind::Calabash,
            "Tiger" => BetKind::Tiger,
            _ => unreachable!()
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BetInfo {
    pub user_public_key: String,
    pub amount: u64,
    pub kind: BetKind,
}

#[component]
pub fn Bet(user_public_key: String) -> impl IntoView {
    let bet_infos_resource = create_resource(
        || (),
        |_| async {
            let bet_infos: Vec<BetInfo> =
                reqwest::get(format!("{}{}", config::ENDPOINT, "/room/get-bets"))
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            bet_infos
        },
    );
    let (kind, set_kind) = create_signal("".to_string());
    let (amount, set_amount) = create_signal(0);

    view! {
        <div>
            <p>"Bet"</p>

            <Suspense fallback=move || view! { <p>"Loading bet"</p> }>
                // handles the error from the resource
                <ErrorBoundary fallback=|_| {
                    view! { <div></div> }
                }>
                    {move || {
                        bet_infos_resource
                            .get()
                            .map(move |bet_infos| {
                                bet_infos
                                    .into_iter()
                                    .map(move |bet_info| {
                                        view! {
                                            <div class="my-10">
                                                <p>{bet_info.user_public_key}</p>
                                                <p>Kind: {
                                                    let temp: String = bet_info.kind.into();
                                                    temp
                                                }</p>
                                                <p>Amount: {bet_info.amount}</p>
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
            <div>

                <form on:submit=move |e| {
                   e.prevent_default();
                let user_public_key = user_public_key.clone();
                spawn_local(async move {
                        let bet_info = BetInfo {
        user_public_key,
        kind: kind.get().into(),
        amount: amount.get(),
    };

                            reqwest::Client::new()
                                .post(format!("{}{}", config::ENDPOINT, "/user/bet"))
                                .json(
                                    &bet_info
                                ).send()
                                    .await
                                .unwrap();
                            location().reload().unwrap();
                });
                }>
                    <label for="kind">Kind: </label>
                    <input
                        type="text"
                        id="kind"
                        name="kind"
                        on:input=move |ev| {
                            // event_target_value is a Leptos helper function
                            // it functions the same way as event.target.value
                            // in JavaScript, but smooths out some of the typecasting
                            // necessary to make this work in Rust
                            set_kind(event_target_value(&ev));
                        }

                        // the `prop:` syntax lets you update a DOM property,
                        // rather than an attribute.
                        prop:value=kind
                    /> <br/>
                    <label for="amount">Amount: </label>
                    <input
                        type="text"
                        id="amount"
                        name="amount"
                        on:input=move |ev| {
                            // event_target_value is a Leptos helper function
                            // it functions the same way as event.target.value
                            // in JavaScript, but smooths out some of the typecasting
                            // necessary to make this work in Rust
                            set_amount(event_target_value(&ev).parse().unwrap());
                        }

                        // the `prop:` syntax lets you update a DOM property,
                        // rather than an attribute.
                        prop:value=amount
                    /> <br/>
                    <button
                        class="focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800"
                    >
                        "Add"
                    </button>
                </form>
            </div>
        </div>
    }
}
