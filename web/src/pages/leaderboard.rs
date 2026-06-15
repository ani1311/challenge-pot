use leptos::prelude::*;
use common::{LeaderboardEntry,LeaderboardUser};

mod table;

#[component]
pub fn Leaderboard() -> impl IntoView {

    let entries =vec![
        LeaderboardEntry {
            user: LeaderboardUser{
                id: "user_1".to_string(),
                username: "Ani".to_string(),
            },
            points: 42,
        },
    ]; 

    view! {
        <table>
            <thead> 
                <tr>
                    <th> "Username" </th>
                    <th> "Points" </th>
                </tr>
            </thead>
            <tbody>
                {
                    entries.into_iter().map(|e|{
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
    }
}
