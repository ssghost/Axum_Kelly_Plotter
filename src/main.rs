use yew::prelude::*;
use yew_router::prelude::*;
mod fetch;
mod plot;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/fetch")]
    Fetch,
    #[at("/kplot")]
    Kplot
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "This is a kelly criterion plotter for stock symbols." }</h1> },
        Route::Fetch => html! { <fetch::Fetch /> },
        Route::Kplot => html! { <plot::Kplot />}
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}