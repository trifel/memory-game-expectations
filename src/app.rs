use crate::pages::home::HomePage;
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="" view=move || view! { <HomePage/> }/>
            </Routes>
        </Router>
    }
}
