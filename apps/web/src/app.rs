use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;

use crate::routes::AppRoutes;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="id">
            <head>
                <Meta charset="UTF-8" />
                <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <Title text="O'USAH — Operasional Unit Sistem Analisis Hidrologi" />
            </head>
            <body>
                <Router>
                    <AppRoutes />
                </Router>
            </body>
        </html>
    }
}