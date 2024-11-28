use wgpu_tests::run;

fn main() {
    let env_filter = tracing_subscriber::filter::EnvFilter::from_default_env();
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_file(true)
        .with_line_number(true)
        .init();
    tracing::info!("Starting app");
    pollster::block_on(run())
}
