use axum::{
    Router,
    extract::State,
    response::Html,
    routing::{get, post},
};

use crate::{
    core::{AppResult, AppState},
    counter::{increment_counter_value, read_counter},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/counter", get(counter_fragment))
        .route("/counter/increment", post(increment_counter_fragment))
}

async fn index(State(app_state): State<AppState>) -> AppResult<Html<String>> {
    let counter = read_counter(&app_state).await?;

    Ok(Html(format!(
        r##"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Service Status Counter</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;700&display=swap" rel="stylesheet">
    <script src="https://unpkg.com/htmx.org@1.9.12"></script>
    <style>
      :root {{
        --base: #24273a;
        --mantle: #1e2030;
        --surface0: #363a4f;
        --surface1: #494d64;
        --text: #cad3f5;
        --subtext1: #b8c0e0;
        --green: #a6da95;
        --green-press: #8ccf75;
      }}

      * {{
        box-sizing: border-box;
      }}

      body {{
        margin: 0;
        min-height: 100vh;
        display: grid;
        place-items: center;
        font-family: "JetBrains Mono", monospace;
        color: var(--text);
        background:
          radial-gradient(circle at 15% 15%, #363a4f 0%, transparent 40%),
          radial-gradient(circle at 85% 85%, #494d64 0%, transparent 45%),
          linear-gradient(160deg, var(--mantle), var(--base));
      }}

      main {{
        width: min(420px, 92vw);
        padding: 2rem 1.5rem;
        border: 1px solid var(--surface1);
        border-radius: 16px;
        background: color-mix(in srgb, var(--surface0) 82%, black 18%);
        text-align: center;
        box-shadow: 0 20px 50px rgba(0, 0, 0, 0.35);
      }}

      h1 {{
        margin: 0 0 1rem;
        font-size: 0.9rem;
        font-weight: 400;
        color: var(--subtext1);
        text-transform: uppercase;
        letter-spacing: 0.08em;
      }}

      .counter {{
        margin: 0;
        font-size: clamp(3rem, 18vw, 5rem);
        font-weight: 700;
        line-height: 1;
      }}

      .controls {{
        margin-top: 1.4rem;
        display: flex;
        gap: 0.75rem;
      }}

      button {{
        border: 0;
        border-radius: 10px;
        padding: 0.85rem 1rem;
        font: inherit;
        font-size: 0.95rem;
        color: #1e2030;
        background: var(--green);
        cursor: pointer;
      }}

      .increment-button {{
        flex: 1;
      }}

      .refresh-button {{
        width: 3rem;
        display: inline-flex;
        align-items: center;
        justify-content: center;
      }}

      .refresh-button svg {{
        width: 100%;
        height: 1.05rem;
      }}

      button:hover {{
        filter: brightness(1.05);
      }}

      button:active {{
        background: var(--green-press);
      }}
    </style>
  </head>
  <body>
    <main>
      <h1>Service Status Counter</h1>
      <div id="counter">{}</div>
      <div class="controls">
        <button class="increment-button" hx-post="/counter/increment" hx-target="#counter" hx-swap="innerHTML">Increment</button>
        <button class="refresh-button" hx-get="/counter" hx-target="#counter" hx-swap="innerHTML" aria-label="Refresh counter" title="Refresh counter">
          <svg viewBox="0 0 24 24" role="img" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12a9 9 0 1 1-2.64-6.36"></path>
            <polyline points="21 3 21 9 15 9"></polyline>
          </svg>
        </button>
      </div>
    </main>
  </body>
</html>"##,
        render_counter(counter)
    )))
}

async fn counter_fragment(State(app_state): State<AppState>) -> AppResult<Html<String>> {
    let counter = read_counter(&app_state).await?;
    Ok(Html(render_counter(counter)))
}

async fn increment_counter_fragment(State(app_state): State<AppState>) -> AppResult<Html<String>> {
    let counter = increment_counter_value(&app_state).await?;
    Ok(Html(render_counter(counter)))
}

fn render_counter(counter: i64) -> String {
    format!("<p class=\"counter\">{counter}</p>")
}
