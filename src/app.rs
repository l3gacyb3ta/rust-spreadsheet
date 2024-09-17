use crate::{
    graph::types::ValueNode,
    sheet::types::{Coord, Sheet},
    sheetRender::SheetComponent,
};
use yew::prelude::*;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[function_component(App)]
pub fn app() -> Html {
    let mut sheet = Sheet::<WIDTH, HEIGHT>::new();
    let a = sheet.add_node(Coord::<usize> { x: 0, y: 0 }, Box::new(ValueNode::new(1.0)));
    let b = sheet.add_node(Coord::<usize> { x: 1, y: 0 }, Box::new(ValueNode::new(2.0)));

    let ab = sheet
        .set_cell_parse("Sum(A1, B1)".to_owned(), Coord::<usize> { x: 0, y: 1 })
        .unwrap();

    // let sheetState = use_state(|| );

    let counter = use_state(|| 0.);

    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1.;
            // sheet.graph.nodes.get_mut(&a).unwrap().set_value(value);
            counter.set(value);
        }
    };

    // assert_eq!(sheet.graph.get_node_value(ab), Some(3.0))

    html! {
        <main>
            // <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Sheet!" }</h1>
            // <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <p> { *counter } </p>
            <button onclick={onclick}> {"Heyyy"} </button>
            <SheetComponent::<WIDTH, HEIGHT> sheet={sheet}/>

        </main>
    }
}
