use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use leptos_actix::*;
        use actix_files::{Files};
        use actix_web::{HttpServer, middleware::Compress, web};
        use darkmode::app::register_server_functions;

        fn app(cx: leptos::Scope) -> impl IntoView {
            use darkmode::app::*;

            view! { cx, <App /> }
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
            let addr = conf.leptos_options.site_address.clone();

            simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
            log::info!("serving at http://{addr}");

            register_server_functions();

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = generate_route_list(app);

                actix_web::App::new()
                    .route("/api/{tail:.*}", handle_server_fns())
                    .leptos_routes(leptos_options.clone(), routes, app)
                    // used by cargo-leptos. Should handle static files another way if not using cargo-leptos.
                    // fallback (for static files)
                    .service(Files::new("", format!("./{site_root}")))
                    .wrap(Compress::default())
            })
            .bind(&addr)?
            .run()
            .await
        }
    }
    else {
        pub fn main() {
            // no client-side main function
            // unless we want this to work with e.g., Trunk for pure client-side testing
            // see lib.rs for hydration function instead
        }
    }
}
