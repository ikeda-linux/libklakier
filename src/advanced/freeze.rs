use crate::database::initialise::DATABASE_PATH;
use rusqlite::Connection;

pub fn freeze(oper: bool) {
    // sets FREEZE in table SETTINGS to true or false (based on the value of oper)
    // this will freeze package updates, installs and uninstalls until they are unfrozen manually
    let conn = Connection::open(DATABASE_PATH).unwrap();
    conn.execute(
        "UPDATE SETTINGS SET FREEZE = ? WHERE ID = 1",
        &[&oper],
    )
}