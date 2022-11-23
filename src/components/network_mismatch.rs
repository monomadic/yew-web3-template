use crate::config::Config;
use yew::{platform::spawn_local, prelude::*};
use yew_ethereum_provider::{Chain, UseEthereumHandle};

#[function_component]
pub fn NetworkMismatch() -> Html {
    let ethereum = use_context::<UseEthereumHandle>()
        .expect("no UseEthereumHandle context found. you must wrap your components in an </>");

    let config: Config = Config::load();
    let chain: Chain = config.into();

    let on_switch_network = {
        let ethereum = ethereum.clone();
        let chain = chain.clone();
        Callback::from(move |_| {
            let ethereum = ethereum.clone();
            let chain = chain.clone();
            spawn_local(async move {
                ethereum.switch_chain_with_fallback(&chain).await.unwrap();
            });
        })
    };

    html! {
        <div>
            if ethereum.connected() && ethereum.chain_id().is_some() && ethereum.chain_id() != Some(8989) {
                <div class="inline-flex items-center">
                    <div class="bg-blue-50 text-blue-800 px-6 py-4 rounded-lg shadow my-5 text-center">
                        <div class="text-center"><img src="/images/icons/network.svg" height="40px" width="40px" alt="disconnected" class="inline" /></div>
                        <div class="text-xl font-bold my-2">{ "Network Mismatch" }</div>
                        <p>
                            { "Your wallet is connected to " }
                            <strong>{ &ethereum.chain_id_hex().expect("chain_id is None") }</strong>
                            { ", but this application runs on " }
                            <strong>{ &chain.chain_id }</strong>
                        </p>
                        <div class="px-0 sm:px-20">
                            <button onclick={on_switch_network} class="btn mt-3 btn-block btn-warning">
                                <img src="/images/networks/43113.svg" alt="switch chain" class="inline-flex mr-2" style="max-height: 20px;" />
                                { "Switch to " }
                                { &chain.chain_name }
                            </button>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
