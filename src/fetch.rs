use std::env;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use yahoo_finance::{history, Interval, Timestamped, Result};

#[tokio::main]
#[function_component]
pub async fn Fetch() -> Html{
    let input_node_ref = use_node_ref();
    let symbol = use_state<String>(|| "AAPL");
    let data = use_state<Result>();

    let onchange = {
        let input_node_ref = input_node_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                symbol.set(input.value());
            } 
        })      
    };
    let onclick = {
        let data = data.clone();
        Callback::from(move |_| {
            data.set(history::retrieve_interval(symbol, Interval::_6mo).await.unwrap())
        })
    };

    html! {
        <>
            <label for="input_symbol">
                { "Input the symbol to be plotted:" }
                <input ref={input_node_ref}
                    {onchange}
                    id="input_symbol"
                    type="text"
                    value={symbol}
                />
            </label>
            <div>
            <button {onclick}>{ "Fetch data" }</button>
            </div>
        </>
    }
}