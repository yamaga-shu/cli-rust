mod commands;

use commands::create_app;

fn main() {
    let app = create_app();
    let matches = app.get_matches();

    let handlers = commands::get_commands();
    let handlers_map: std::collections::HashMap<&str, Box<dyn commands::CommandHandler>> =
        handlers.into_iter().collect();

    match matches.subcommand() {
        Some((cmd_name, sub_matches)) => {
            if let Some(handler) = handlers_map.get(cmd_name) {
                handler.run(sub_matches);
            } else {
                eprintln!("Unknown command: {}", cmd_name);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("No subcommand provided");
            std::process::exit(1);
        }
    }
}
