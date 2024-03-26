use yew::prelude::*;
use yew_router::prelude::*;

mod form;
mod products;
mod router;

use router::{switch, Route};

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="px-4 py-5 my-5 text-center">
                <h1 class="display-5 fw-bold text-primary">{"YewAxum ProductsApp"} </h1>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
