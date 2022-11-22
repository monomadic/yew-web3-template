use yew::prelude::*;

#[function_component]
pub(crate) fn Logo() -> Html {
    html! {
        <div>
            <h3 class="uppercase font-bold text-base">
                <img
                    src="/images/providers/metamask.svg"
                    alt="logo-img"
                    height="30px"
                    width="30px"
                    style="display:inline-block;"
                />
                {" Yew Web3 Example"}
            </h3>
        </div>
    }
}
