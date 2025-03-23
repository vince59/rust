use axum::extract::State;
use axum::http::StatusCode;
use axum::{Router, extract::Form, response::Html, routing::get};
use chrono::{Days, Months, prelude::*};
use minijinja::{Environment, context};
use serde::Deserialize;
use std::sync::Arc;
use std::time::SystemTime;

struct AppState {
    env: Environment<'static>,
}

#[tokio::main]
async fn main() {
    // Ajout des vues
    let mut env = Environment::new();
    env.add_template("main", include_str!("../views/layout/main.jinja"))
        .unwrap();
    env.add_template("menu", include_str!("../views/layout/menu.jinja"))
        .unwrap();
    env.add_template("home", include_str!("../views/home.jinja"))
        .unwrap();
    env.add_template(
        "planning.create",
        include_str!("../views/planning/create.jinja"),
    )
    .unwrap();
    env.add_template(
        "planning.result",
        include_str!("../views/planning/result.jinja"),
    )
    .unwrap();

    // pass env to handlers via state
    let app_state = Arc::new(AppState { env });

    // Ajout des routes
    let app = Router::new()
        .route("/", get(controller_home))
        .route(
            "/planning",
            get(controller_planning_create).post(controller_planning_generate),
        )
        .with_state(app_state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn controller_home(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("home").unwrap();

    let rendered = template
        .render(context! {
            title => "Vélo",
        })
        .unwrap();

    Ok(Html(rendered))
}

async fn controller_planning_create(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("planning.create").unwrap();

    let rendered = template
        .render(context! {
            title => "Nouveau planning",
        })
        .unwrap();

    Ok(Html(rendered))
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    nbdays: i32,
    strdate: String,
    enddate: String,
}

async fn controller_planning_generate(
    State(state): State<Arc<AppState>>,
    Form(mut input): Form<Input>,
) -> Result<Html<String>, StatusCode> {
    input.nbdays += 7;

    dbg!(&input);
    let template = state.env.get_template("planning.result").unwrap();
    let days = vec!["Data 1", "Data 2", "Data 3"];

    let curr_date = NaiveDate::parse_from_str(&input.strdate, "%Y-%m-%d").unwrap();
    println!("Parse Date from String: {curr_date:?}");
    let day_number = curr_date.weekday().number_from_monday();
    println!("jour: {}", day_number);
    let new_date = curr_date.checked_add_days(Days::new(10));
    println!("Add days and months to a Date/Time: {new_date:?}");

    let rendered = template
        .render(context! {
            title => "Content",
            nbdays => input.nbdays,
            days => days,
        })
        .unwrap();

    Ok(Html(rendered))
}
