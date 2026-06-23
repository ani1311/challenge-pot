use leptos::prelude::*;

use crate::{api, auth::AuthState};

mod table;

#[component]
pub fn Leaderboard() -> impl IntoView {
    let auth = expect_context::<AuthState>();

    let leaderboard = LocalResource::new(move || async move {
        let token = auth
            .token()
            .ok_or_else(|| "Your session has expired. Log in again.".to_owned())?;

        api::leaderboard::fetch_leaderboard(token).await
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
