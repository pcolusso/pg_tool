use postgres::{Connection, TlsMode};
use std::error::Error;

pub fn get_db_size(connection_string: String) -> Result<i32, Box<Error>> {
  let conn = Connection::connect(connection_string, TlsMode::None)?;

  for row in &conn.query("SELECT pg_database_size('iptaas-development');", &[])? {
    let size_in_bytes: i64 = row.get(0);
    return Ok(size_in_bytes as i32);
  }

  return Ok(0);
}