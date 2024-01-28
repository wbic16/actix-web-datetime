// frontend/src/main.rs

// dependencies
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
      <header>
        <h1>"HTMX: An Illustration"</h1>
      </header>
      <main>
        <h2>"Today's Date:"</h2>
        <p hx-get="http://localhost:8000/">""</p>
      </main>
      <footer>
        <p>"Copyright Jeffery D. Mitchell"</p>
      </footer>
    }
}

// main function
fn main() {
    mount_to_body(|| view! { <App /> })
}
