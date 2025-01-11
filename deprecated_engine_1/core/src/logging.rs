/*
    let colors = fern::colors::ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .info(fern::colors::Color::Green);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] ({}) {} {}",
                record.target().magenta().bold(),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    error!("This is an error with value: {}", 42);
    error!("Vector out of bounds.\n          {}| {}\n          {}| {}", "msg".magenta(), "in function renderStatic()", "err".magenta(), "indexing out of bounds");
    info!("This is an info message.");

    */

    fn this() {
        
    }