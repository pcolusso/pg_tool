use clap::{Arg, App};
use indicatif::{ProgressBar, ProgressStyle};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

mod settings;
mod query;
mod dump;

fn main() {
  let matches = App::new("postgres-tool")
                        .version("0.1")
                        .author("Paul Colusso <me@paulcolusso.com>")
                        .get_matches();

  let settings = settings::Settings::load_settings().unwrap();
  let size = query::get_db_size(settings.connection_string());

  let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
  let dump_thread = thread::spawn(move || {
    dump::dump(tx);
  });

  let pb = ProgressBar::new(size.unwrap() as u64);
  pb.set_style(ProgressStyle::default_bar()
      .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
      .progress_chars("#>-"));

  for x in rx {
    pb.set_position(x as u64);
  }

  dump_thread.join().expect("Dump thread panicked.");
}
