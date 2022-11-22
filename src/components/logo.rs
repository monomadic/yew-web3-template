use yew::prelude::*;

#[function_component]
pub(crate) fn Logo() -> Html {
    let svg = Html::from_html_unchecked(include_str!("../../assets/images/BTC.svg").into());

    html! {
        <div>
            <h3 class="uppercase font-bold text-base inline-flex space-x-4 items-center">
                <div class="">{svg}</div>
                <div class="">{"Yew Web3 Example"}</div>
            </h3>
        </div>
    }
}
