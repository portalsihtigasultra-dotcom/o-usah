use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::pages::{
    admin::AdminPage,
    curah_hujan::analisis::AnalisisPage,
    curah_hujan::detail::DetailPage,
    curah_hujan::input::InputPage,
    curah_hujan::rekap::RekapPage,
    home::HomePage,
    login::LoginPage,
    validasi::ValidasiPage,
};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div>
            <h1>"404"</h1>
            <p>"Halaman tidak ditemukan"</p>
        </div>
    }
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Routes fallback=NotFound>
            <Route path=path!("/") view=HomePage />
            <Route path=path!("/login") view=LoginPage />
            <Route path=path!("/curah-hujan") view=RekapPage />
            <Route path=path!("/curah-hujan/input") view=InputPage />
            <Route path=path!("/curah-hujan/:id") view=DetailPage />
            <Route path=path!("/curah-hujan/analisis") view=AnalisisPage />
            <Route path=path!("/debit") view=|| view! { <p>"Debit — Dalam Proses"</p> } />
            <Route path=path!("/klimatologi") view=|| view! { <p>"Klimatologi — Dalam Proses"</p> } />
            <Route path=path!("/kualitas-air") view=|| view! { <p>"Kualitas Air — Dalam Proses"</p> } />
            <Route path=path!("/validasi") view=ValidasiPage />
            <Route path=path!("/admin") view=AdminPage />
        </Routes>
    }
}