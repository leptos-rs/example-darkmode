use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use actix_files::{Files, NamedFile};
        use actix_web::{HttpServer, middleware::Compress, web};
        use leptos_start::app::*;

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
            let addr = conf.leptos_options.site_address.clone();

            simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
            log::info!("serving at http://{addr}");

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let pkg_dir = &leptos_options.site_pkg_dir;
                let bundle_path = format!("/{site_root}/{pkg_dir}");
                let output_name = leptos_options.output_name.clone();

                actix_web::App::new()
                    // used by cargo-leptos. Can be removed if using wasm-pack and cargo run.
                    .service(Files::new(&bundle_path, format!("./{bundle_path}")))
                    .route("/style.css", web::get().to(move || {
                        let bundle_path = bundle_path.clone();
                        let output_name = output_name.clone();
                        async move {
                            NamedFile::open_async(format!("./{}/{}.css", bundle_path, output_name)).await.expect("could not open CSS file")
                        }
                    }))
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .route("/{tail:.*}", leptos_actix::render_app_to_stream(leptos_options.to_owned(), |cx| view! { cx, <App/> }))
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
