use gloo_utils::document_element;
use yew::prelude::*;

pub struct Theme {
    name: String,
    col1: String,
    col2: String,
    text: String,
}

#[function_component]
pub(crate) fn ThemeSwitch() -> Html {
    let themes = [
        Theme {
            name: "dark".to_string(),
            col1: "#3E4450".to_string(),
            col2: "#7040EB".to_string(),
            text: "Dark".to_string(),
        },
        Theme {
            name: "synthwave".to_string(),
            col1: "#2A1C65".to_string(),
            col2: "#C875AE".to_string(),
            text: "Purple".to_string(),
        },
        Theme {
            name: "aqua".to_string(),
            col1: "#1224B0".to_string(),
            col2: "#74DDF7".to_string(),
            text: "C64".to_string(),
        },
        Theme {
            name: "halloween".to_string(),
            col1: "#212121".to_string(),
            col2: "#DA8A37".to_string(),
            text: "Goth Kid".to_string(),
        },
        Theme {
            name: "forest".to_string(),
            col1: "#161212".to_string(),
            col2: "#58B560".to_string(),
            text: "Slimer".to_string(),
        },
        Theme {
            name: "valentine".to_string(),
            col1: "#EA3C8B".to_string(),
            col2: "#ECD7E7".to_string(),
            text: "BabyGirl".to_string(),
        },
    ];

    fn change_theme(ev: MouseEvent, theme_name: &str) {
        ev.prevent_default();

        document_element()
            .set_attribute("data-theme", theme_name)
            .expect("cannot set attribute data-theme");
    }

    html! {
        <div class="dropdown dropdown-end">
            <div tabindex="0" class="btn btn-circle">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    class="inline-block w-6 h-6 stroke-current"
                    ><path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"
                    /></svg
                >
            </div>
            <ul tabindex=0 class="shadow menu dropdown-content bg-base-200 rounded-box w-52">
                {
                    themes.into_iter().map(|theme| {
                        let theme_name: String = theme.name.clone();
                        html!{
                                <li>
                                    <a href="#theme" data-color={theme.name.clone()} onclick={Callback::from(move |ev| change_theme(ev, &theme_name))}>
                                        <span
                                            data-color={theme.name.clone()}
                                            class="border border-opacity-10 rounded mr-3 h-6 w-6"
                                            style={"background: linear-gradient(90deg, ".to_string() + &theme.col1 + ", " + &theme.col1 + ", 50%, " + &theme.col2 + " 50%, " + &theme.col2 + ")"}

                                        />
                                        {theme.text}
                                    </a>
                                </li>
                            }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}
