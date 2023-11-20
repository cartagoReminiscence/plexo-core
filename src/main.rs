use std::error::Error;

use dotenv::dotenv;
use plexo_core::{
    api::graphql::schema::GraphQLSchema,
    auth::handlers::{
        email_basic_login_handler, github_callback_handler, github_sign_in_handler, logout_handler,
    },
    core::{
        app::new_core_from_env,
        config::{DOMAIN, URL},
    },
    handlers::{graphiq_handler, index_handler, ws_switch_handler},
};
use poem::{get, listener::TcpListener, middleware::Cors, post, EndpointExt, Route, Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let core = new_core_from_env().await?;

    let graphql_schema = core.graphql_api_schema();

    let app = Route::new()
        // .nest("/api", api_service)
        // .nest("/", ui)
        // .nest("/", static_page)
        // Non authenticated routes
        .at("/auth/email/login", post(email_basic_login_handler))
        // .at("/auth/email/register", post(email_basic_register_handler))
        //
        .at("/auth/github", get(github_sign_in_handler))
        .at("/auth/github/callback", get(github_callback_handler))
        //
        .at("/auth/logout", get(logout_handler))
        //
        .at("/playground", get(graphiq_handler))
        .at("/graphql", post(index_handler))
        .at("/graphql/ws", get(ws_switch_handler));

    let app = app
        .with(Cors::new().allow_credentials(true))
        .data(graphql_schema)
        .data(core.clone());

    println!("Visit GraphQL Playground at {}/playground", *DOMAIN);

    Server::new(TcpListener::bind(URL.to_owned()))
        .run(app)
        .await
        .expect("Fail to start web server");

    Ok(())
}
