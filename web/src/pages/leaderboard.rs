use leptos::prelude::*;

use crate::api;

mod table;

#[component]
pub fn Leaderboard() -> impl IntoView {

    let leaderboard = LocalResource::new(|| async {
        api::leaderboard::fetch_leaderboard().await
    });

    view! {
        {move || match leaderboard.get(){
            Some(Ok(resp)) => view! {
                    <table>
                        <thead> 
                            <tr>
                                <th> "Username" </th>
                                <th> "Points" </th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                resp.entries.into_iter().map(|e|{
                                    view! {
                                        <tr>
                                            <td>{e.user.username}</td>
                                            <td>{e.points}</td>
                                        </tr>
                                    }
                                }).collect_view()
                            }
                        </tbody>
                    </table>
            }.into_any(),
            Some(Err(error)) => view! {
                <p>"Failed to load leaderboard: " {error.to_string()}</p>
            }.into_any(),
            None => view! { <p>"Loading leaderboard..."</p> }.into_any(),
        }}

    }
}
