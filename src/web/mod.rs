use std::sync::Arc;

use actix_web::{http, middleware, server, App};
use failure::Error as FailureError;

mod android_adapter;
mod app;
mod auth;
mod state;

use self::app::App as WebApp;
use error::AppError;
use registry::Registry;

use self::state::AppState as InnerAppState;

pub type AppState = Arc<InnerAppState>;

pub fn start() {
    let app = match WebApp::from_args() {
        Ok(app) => app,
        Err(err) => {
            error!("{}", err);
            println!("{}", err);
            ::std::process::exit(1);
        }
    };
    info!("{:?}", &app);

    match start_web(app) {
        Err(err) => {
            error!("{}", err);
            println!("{}", err);
            ::std::process::exit(1);
        }
        Ok(_) => {}
    };
}

fn start_web(app: WebApp) -> Result<(), FailureError> {
    let config = crate::config::config(&app.config_path)?;
    let config_without_passwords = crate::config::Config {
        email_smtp_credential_password: None,
        ..config.clone()
    };
    info!("config: {:?}", &config_without_passwords);
    let registry = Registry::new(config.data_path.clone().into())?;
    info!("registry created");
    let sys = actix::System::new("accounting-web");
    let state = Arc::new(InnerAppState::new(registry));
    server::new(move || {
        App::<AppState>::with_state(state.clone())
            .middleware(middleware::Logger::default())
            .resource("/android/v1/sms/latest", |r| {
                r.method(http::Method::GET)
                    .with(android_adapter::get_sms_latest)
            }).resource("/android/v1/sms", |r| {
                r.method(http::Method::POST).with_config(android_adapter::post_sms, |cfg| { cfg.1.limit(2 * 1024 * 1024); })
            })
    }).bind("0.0.0.0:8080")
    .unwrap()
    .shutdown_timeout(1)
    .workers(1)
    .start();

    info!("Started http server: 0.0.0.0:8080");
    match sys.run() {
        0 => Ok(()),
        code @ _ => Err(AppError::ActixError { code }.into()),
    }
}
