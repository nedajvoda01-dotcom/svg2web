pub fn init_logger() {
	let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
		.try_init();
}
