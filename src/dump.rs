use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::sync::mpsc::{Sender};
use std::fs::File;
use std::error::Error;

pub fn dump(transmitter: Sender<usize>) -> Result<(), Box<Error>> {
  let mut cmd = Command::new("pg_dumpall")
    .arg("-h").arg("localhost")
    .arg("-U").arg("postgres")
    //use pgpass; it's recommended for libpq and unless we can pass the vars securely, it's probs the best way.
    .stdout(Stdio::piped())
    .spawn()?;

  let mut size_accrued: usize = 0;
  let mut line_number: i32 = 0;

  let stdout = cmd.stdout.as_mut().unwrap();
  let file = File::create("out.sql")?;
  let map = File::create("out.sql.map")?;
  let reader = BufReader::new(stdout);
  let mut writer = BufWriter::new(file);
  let mut map_writer = BufWriter::new(map);


  for line in reader.lines() {
    let l = line?;

    size_accrued += l.len();
    line_number += 1;

    if l.contains("CREATE DATABASE") {
      map_writer.write(format!("cd: {:?}\n", line_number).as_bytes());
    }

    transmitter.send(size_accrued);

    writer.write(l.as_bytes())?;
    writer.write("\n".as_bytes())?;
  }

  drop(transmitter);

  Ok(())
}