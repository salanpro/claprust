use clap::Parser;
use std::{process::Command, thread, time::Duration};

#[derive(Parser, Debug)]
#[command(name = "yensona", about = "Programa para abrir programas")]
struct Args {
    /// Tiempo
    #[arg(short, long)]
    time: u64,

    /// Aplicacion
    #[arg(short, long)]
    app: String,
}

fn main() {
    let args = Args::parse();

    println!(
        "El programa {} se abrira en {} segundos",
        args.app, args.time
    );

    for i in (1..=args.time).rev() {
        println!("Abriendo en {}", i);
        thread::sleep(Duration::from_secs(1));
    }

    let com = Command::new(&args.app).spawn();

    match com {
        Ok(_) => println!("'{}' abierto con éxito.", args.app),
        Err(e) => eprintln!("Error al abrir '{}': {}", args.app, e),
    }

    println!("Abriendo")
}
