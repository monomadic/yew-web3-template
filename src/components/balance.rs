use yew::{platform::spawn_local, prelude::*};
use yew_ethereum_provider::{Chain, UseEthereumHandle};

#[function_component]
pub fn NetworkMismatch() -> Html {
    let ethereum = use_context::<UseEthereumHandle>()
        .expect("component should be wrapped in a <UseEthereumHandle>");

    let chain: Chain = crate::config::Config::load().into();

    html! {
        <div>
            if ethereum.connected() && ethereum.chain_id().is_some() && ethereum.chain_id() != Some(8989) {
                <div class="inline-flex items-center">
                </div>
            } else {
            }
        </div>
    }
}
