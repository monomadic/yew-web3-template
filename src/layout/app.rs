use yew::prelude::*;
use yew_ethereum_provider::EthereumContextProvider;

use super::header::Header;
use crate::components::NetworkMismatch;

#[function_component]
pub(crate) fn App() -> Html {
    html! {
        <EthereumContextProvider>
            <Header />
            <main class="max-w-7xl mx-auto px-2 sm:px-6 lg:px-8 text-center">
                <NetworkMismatch />
            </main>
        </EthereumContextProvider>
    }
}
