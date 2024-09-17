use crate::sheet::types::CellPrint;
use crate::sheet::types::Sheet;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SheetComponentProps<const WIDTH: usize, const HEIGHT: usize> {
    pub sheet: Sheet<{ WIDTH }, { HEIGHT }>
}

#[function_component]
pub fn SheetComponent<const WIDTH: usize, const HEIGHT: usize>(
    props: &SheetComponentProps::<{ WIDTH }, { HEIGHT }>,
) -> Html {
    let sheet = &props.sheet;
    html! {
        <table>
            <tbody>
            {
                sheet.cells
                    .into_iter()
                    .map(|row| {
                        return html! {
                            <tr>
                            {
                                row.into_iter().map(|cell| {
                                    match cell {
                                        Some(cell) => {
                                            let value = sheet.graph.get_node_value(cell).unwrap();
                                            html! {
                                                <td>
                                                {
                                                    value
                                                }
                                                </td>
                                            }
                                        }
                                        None => {
                                            html! {<td><p>{"None"}</p></td>}
                                        }
                                    }


                                }).collect::<Html>()
                            }
                            </tr>
                        }
                }).collect::<Html>()
            }
            </tbody>
        </table>
    }
}

// impl<const WIDTH: usize, const HEIGHT: usize> Sheet<{ WIDTH }, { HEIGHT }>  {

//     #[function_component(App)]
//     pub fn render(&self) -> Html {

//     }

// }
