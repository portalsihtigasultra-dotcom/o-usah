mod api;
mod app;
mod components;
mod pages;
mod routes;

#[cfg(feature = "ssr")]
fn shell(_opts: leptos::config::LeptosOptions) -> impl leptos::prelude::IntoView {
    app::App()
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let mut leptos_options = conf.leptos_options;
    leptos_options.site_addr = "127.0.0.1:3001".parse().unwrap();

    let addr = leptos_options.site_addr;
    let routes = generate_route_list(app::App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, app::App)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    println!("O'USAH Web starting on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}