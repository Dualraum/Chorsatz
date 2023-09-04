#![allow(dead_code)]
mod generator;
mod notes;
mod scale;

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .expect("Could not initialize logger.");

    let mut line = String::new();
    let mut accords = Vec::new();

    log::info!("Enter a valid accord key or nothing to finish the input loop:");

    while std::io::stdin().read_line(&mut line).is_ok() && {
        line = line.replace(['\n', '\r'], "");
        !line.is_empty()
    } {
        match notes::create_triad(&line) {
            Ok(accord) => {
                accords.push(accord);
                log::info!("Added accord {line}. Enter the next accord key:");
            }
            Err(e) => log::error!("{}", e),
        }
        line.clear();
    }

    log::info!(
        "Calculation solution for the following accord list: {:?}",
        accords
    );

    let solutions = generator::generate_satb(&accords);

    if solutions.is_empty() {
        log::warn!("No solutions found!");
    } else {
        log::info!("Found {} valid solutions.", solutions.len());
    }

    for (index, (solution, score)) in solutions.iter().take(10).enumerate() {
        log::info!("Solution {} with score {:.2}:", index + 1, score);
        for block in solution {
            print!("{} -> ", block);
        }
        println!()
    }

    log::info!("Press Enter to exit the program.");
    if std::io::stdin().read_line(&mut line).is_err() {
        log::error!("That was not a nice input.");
    };
}
