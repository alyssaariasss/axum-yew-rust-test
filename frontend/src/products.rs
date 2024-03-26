use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: i32,
    name: String,
    price: i32,
}

#[function_component]
pub fn Products() -> Html {
    let data: UseStateHandle<Vec<Product>> = use_state(std::vec::Vec::new);
    {
        let data_clone = data.clone();
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data = reqwest::get("http://localhost:3000/api/products")
                    .await
                    .expect("Cannot get data from URL")
                    .json::<Vec<Product>>()
                    .await
                    .expect("Cannot convert to JSON");
                data_clone.set(fetched_data)
            });
            || ()
        });
    }

    let products = data
        .iter()
        .map(|product| {
            html! {
                <ul class="list-group my-2">
                    <li class="list-group-item p-3 text-primary-emphasis bg-light-subtle border border-light-subtle rounded-3" key={product.id}>
                        <p class="my-0">{format!("Name: {}", product.name)}</p>
                        <p class="my-0">{format!("Price: {}", product.price)}</p>
                    </li>
                </ul>
            }
        })
        .collect::<Html>();

    html! {
        <div class="container my-4">
            <button class="btn btn-outline-primary">
                <Link<Route> to={Route::AddProduct} classes="link-offset-2 link-underline link-underline-opacity-0">{"Add New Product"}</Link<Route>>
            </button>
            <div class="my-4">
                <h2>{"List of Products: "} {data.len()}</h2>
                <p class="mt-4">{products}</p>
            </div>
        </div>
    }
}
