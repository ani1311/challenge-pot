use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="page page--about">
            <header class="page-header">
                <h1 class="page-title">"About"</h1>
                <p class="page-subtitle">
                    "Challenge Pot tracks sugar as points earned and exercise as points paid back. The leaderboard is simply each user's running total."
                </p>
            </header>

            <section class="about-section">
                <h2>"How scoring works"</h2>
                <p>
                    "Every tracked entry becomes an activity. Activities produce points using a fixed formula. Sugar adds points. Exercise subtracts points. Lower scores mean more activity has been used to offset sugar."
                </p>

                <table class="about-table">
                    <thead>
                        <tr>
                            <th>"Activity"</th>
                            <th>"Input"</th>
                            <th>"Formula"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Sugar"</td>
                            <td>"grams"</td>
                            <td>"+1 point per gram"</td>
                        </tr>
                        <tr>
                            <td>"Walk"</td>
                            <td>"kilometers"</td>
                            <td>"-2 points per km"</td>
                        </tr>
                        <tr>
                            <td>"Hike"</td>
                            <td>"kilometers"</td>
                            <td>"-3 points per km"</td>
                        </tr>
                        <tr>
                            <td>"Run"</td>
                            <td>"kilometers"</td>
                            <td>"-4 points per km"</td>
                        </tr>
                        <tr>
                            <td>"Swimming"</td>
                            <td>"kilometers"</td>
                            <td>"-4 points per km"</td>
                        </tr>
                        <tr>
                            <td>"Bike"</td>
                            <td>"kilometers"</td>
                            <td>"-1 point per km"</td>
                        </tr>
                        <tr>
                            <td>"Mountain bike"</td>
                            <td>"kilometers"</td>
                            <td>"-5 points per km"</td>
                        </tr>
                        <tr>
                            <td>"Racquet sport"</td>
                            <td>"hours"</td>
                            <td>"-5 points per hour"</td>
                        </tr>
                    </tbody>
                </table>
            </section>

            <section class="about-section">
                <h2>"Examples"</h2>
                <div class="example-list">
                    <p>
                        <strong>"25g sugar"</strong>
                        " adds "
                        <span class="points-positive">"+25"</span>
                        " points."
                    </p>
                    <p>
                        <strong>"3 km walk"</strong>
                        " subtracts "
                        <span class="points-negative">"-6"</span>
                        " points."
                    </p>
                    <p>
                        <strong>"5 km run"</strong>
                        " subtracts "
                        <span class="points-negative">"-20"</span>
                        " points."
                    </p>
                    <p>
                        <strong>"1 hour racquet sport"</strong>
                        " subtracts "
                        <span class="points-negative">"-5"</span>
                        " points."
                    </p>
                </div>
            </section>

            <section class="about-section">
                <h2>"Leaderboard totals"</h2>
                <p>
                    "The leaderboard adds up all logged activity for each person. If someone logs 40g of sugar and then runs 5 km, their total changes by +40 and then -20, leaving a net score of +20."
                </p>
                <p>
                    "Rank is based on the resulting total. The app treats the points as a balance: sugar raises the balance, exercise lowers it."
                </p>
            </section>

            <section class="about-section">
                <h2>"Why the mapping is fixed"</h2>
                <p>
                    "The formulas live in the server's activity model, so every client submits the same activity types and the server applies one consistent scoring rule. That keeps the leaderboard comparable across users."
                </p>
            </section>
        </section>
    }
}
