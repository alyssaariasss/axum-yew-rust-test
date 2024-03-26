use serde_json::json;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::router::Route;

#[function_component]
pub fn Form() -> Html {
    let navigator = use_navigator().unwrap();

    let name_ref = NodeRef::default();
    let name_ref_outer = name_ref.clone();

    let price_ref = NodeRef::default();
    let price_ref_outer = price_ref.clone();

    // Submit button event
    let onclick = Callback::from(move |_e: MouseEvent| {
        let price = price_ref.cast::<HtmlInputElement>().unwrap();
        let name = name_ref.cast::<HtmlInputElement>().unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            let product = json!({
                "name": name.value(),
                "price": price.value().parse::<i32>().unwrap()
            });

            let client = reqwest::Client::new();
            let _res = client
                .post("http://localhost:3000/api/products")
                .json(&product)
                .send()
                .await;
        });

        navigator.push(&Route::Home);
    });

    html! {
        <div class="card text-center mx-5 bg-light-subtle">
            <div class="card-body">
            <h2 class="card-title my-3">{"Add a Product"}</h2>
            <form class="px-5">
                <div class="mb-3 row">
                    <label for="name" class="col-sm-2 col-form-label">
                    {"Name"}
                    </label>
                    <div class="col-sm-10">
                        <input ref={name_ref_outer.clone()}
                            id="name"
                            class="form-control"
                            type="text"
                        />
                    </div>
                </div>
                <div class="mb-3 row">
                    <label for="price" class="col-sm-2 col-form-label">
                    {"Price"}
                    </label>
                    <div class="col-sm-10">
                        <input ref={price_ref_outer.clone()}
                            id="price"
                            class="form-control"
                            type="number"
                        />
                    </div>
                </div>
                <button {onclick} class="btn btn-primary">{"Add Product"}</button>
            </form>
            </div>
        </div>
    }
}
