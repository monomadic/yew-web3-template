use yew::prelude::*;
use yew_ethereum_provider::{ConnectButton, UseEthereumHandle};

use crate::components::{ThemeSwitch, Logo};

#[function_component]
pub fn Header() -> Html {
    let ethereum = use_context::<UseEthereumHandle>().expect(
        "no ethereum provider found. you must wrap your components in an <EthereumProvider/>",
    );
    let address = ethereum
        .address()
        .map(|addr| format!("{:?}", addr).to_lowercase())
        .unwrap_or_default();

    log::info!("{:?}", address);

    html! {
        <nav class="bg-black bg-opacity-10 shadow-sm">
            <div class="max-w-7xl mx-auto px-2 sm:px-6 lg:px-12">
                <div class="relative flex items-center justify-between h-16">
                    <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">

                        <button
                            type="button"
                            class="inline-flex items-center justify-center p-2 rounded-md text-white hover:opacity-50 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
                            aria-controls="mobile-menu"
                            aria-expanded="false"
                        >
                            <svg
                                class="block h-6 w-6"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                            <svg
                                class="hidden h-6 w-6"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12"
                                />
                            </svg>
                        </button>
                    </div>
                    <div class="flex-1 flex sm:ml-0 ml-10 sm:items-stretch justify-center sm:justify-start">
                        <div class="flex-shrink-0 flex items-center justify-center -ml-4 sm:m-0">
                            <Logo />
                        </div>
                    </div>
                    <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
                        <div class="hidden sm:flex inline-flex space-x-4 items-center">
                            <ThemeSwitch />
                            <ConnectButton />

                            if ethereum.connected() {
                                <div class={classes!("w-10", "h-10", "p-0.5", "mask", "mask-squircle", "bg-white")}>
                                    <yew_ethereum_blockies::Blockie {address} />
                                </div>
                            }
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
