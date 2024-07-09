use crate::config;
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

impl Into<String> for BetKind {
    fn into(self) -> String {
        match self {
            BetKind::Fish => "Fish".to_string(),
            BetKind::Prawn => "Prawn".to_string(),
            BetKind::Crab => "Crab".to_string(),
            BetKind::Cock => "Cock".to_string(),
            BetKind::Calabash => "Calabash".to_string(),
            BetKind::Tiger => "Tiger".to_string(),
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
pub fn Bet() -> impl IntoView {
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

    view! {
        <div>
            <p>"Bet"</p>

            <Suspense fallback=move || view! { <p>"Loading User"</p> }>
                // handles the error from the resource
                <ErrorBoundary fallback=|_| {
                    view! { <p>"Something went wrong"</p> }
                }>
                    {move || {
                        bet_infos_resource
                            .read()
                            .map(move |bet_infos| {
                                bet_infos
                                    .into_iter()
                                    .map(move |bet_info| {
                                        view! {
                                            <div>
                                                <p>{bet_info.user_public_key}</p>
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
            <button
                type="button"
                class="focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800"
            >
                "Add"
            </button>
        </div>
    }
}
