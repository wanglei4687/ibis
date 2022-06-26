use std::env;

fn main() {
    let dsn = env::var("IBIS_DSN").is_err();
    let _guard = sentry::init((
        dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    // Sentry will capture this
    panic!("Everything is on fire!");
}
