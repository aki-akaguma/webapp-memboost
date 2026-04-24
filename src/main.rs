use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_desktop::{Config, WindowBuilder};

mod components;
use components::*;

fn main() {
    // You can set the ports and IP manually with env vars:
    //   server launch:
    //     IP="0.0.0.0" PORT=8080 ./server

    // You can supplement panic on firefox browser.
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();

    #[cfg(not(debug_assertions))]
    let level = dioxus::logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus::logger::tracing::Level::DEBUG;
    dioxus::logger::init(level).expect("failed to init logger");

    // In the case of release desktop and release mobile,
    // connect backend calls to public api
    #[cfg(not(debug_assertions))]
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        // Specify the URL that previously delpoyed the public webapp.
        // This webapp was created with `dx bundle --web`.
        let backend_url = "https://aki.omusubi.org/memboost";
        dioxus::fullstack::set_server_url(backend_url);
    }

    // In the case of only desktop
    #[cfg(feature = "desktop")]
    dioxus::LaunchBuilder::new()
        .with_cfg({
            let cnf = Config::default().with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Mem Boost"),
            );
            #[cfg(not(debug_assertions))]
            let cnf = cnf.with_menu(None);
            cnf
        })
        .launch(App);

    // In the other case, simple launch app
    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        MemBoost {}
    }
}
