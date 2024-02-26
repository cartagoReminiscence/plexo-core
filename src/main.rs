use std::error::Error;

use dotenv::dotenv;
use plexo_core::{
    api::{graphql::schema::GraphQLSchema, openapi::api::PlexoOpenAPI},
    auth::handlers::{email_basic_login_handler, github_callback_handler, github_sign_in_handler, logout_handler},
    core::{
        app::new_core_from_env,
        config::{DOMAIN, URL, CERT_PEM_PATH, KEY_PEM_PATH},
    },
    handlers::{graphiq_handler, index_handler, ws_switch_handler},
};
use poem::{get, listener::{Listener, TcpListener}, middleware::Cors, post, EndpointExt, Route, Server};
use poem::listener::{RustlsCertificate, RustlsConfig };
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let core = new_core_from_env().await?;

    core.prelude().await?;

    let graphql_schema = core.graphql_api_schema();

    let api_prefix = "/v1/api";

    let openapi_server = format!("{}{}", *DOMAIN, api_prefix);

    let api_service = OpenApiService::new(PlexoOpenAPI::new(core.clone()), "Plexo Open API", "1.0").server(openapi_server);

    let spec_json_handler = api_service.spec_endpoint();
    let spec_yaml_handler = api_service.spec_endpoint_yaml();

    let ui = api_service.swagger_ui();

    let app = Route::new()
        .nest(api_prefix, api_service)
        .nest("/", ui)
        .at("/openapi.json", get(spec_json_handler))
        .at("/openapi.yaml", get(spec_yaml_handler))
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
    
    if CERT_PEM_PATH.to_string() != "none"
    {
        let cert_content = std::fs::read(CERT_PEM_PATH.to_string()).expect("Error al leer el certificado");
    
        let key_content = std::fs::read(KEY_PEM_PATH.to_string()).expect("Error al leer la clave privada");
    
        let rustls_cert = RustlsCertificate::new()
        .cert(cert_content)
        .key(key_content);
    
        let config =
        RustlsConfig::new().fallback(rustls_cert);
        let listener = TcpListener::bind(URL.to_owned()).rustls(config);
        Server::new(listener)
        .run(app)
        .await
        .expect("Fail to start web server");

    }
    else{
        let listener = TcpListener::bind(URL.to_owned());
        Server::new(listener)
        .run(app)
        .await
        .expect("Fail to start web server");

    }

    Ok(())
}
