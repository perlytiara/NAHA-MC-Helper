use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = r"C:\Users\user\AppData\Roaming\AstralRinthApp\app.db";

    let conn = Connection::open(db_path)?;

    // First, let's see what tables exist
    println!("=== TABLES ===");
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table';")?;
    let table_iter = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(0)?)
    })?;

    for table in table_iter {
        println!("Table: {}", table?);
    }

    // Check if profiles table exists and its structure
    println!("\n=== PROFILES TABLE STRUCTURE ===");
    let mut stmt = conn.prepare("PRAGMA table_info(profiles);")?;
    let column_iter = stmt.query_map([], |row| {
        Ok(format!("{}: {}", row.get::<_, String>(1)?, row.get::<_, String>(2)?))
    })?;

    for column in column_iter {
        println!("Column: {}", column?);
    }

    // Check all profiles
    println!("\n=== ALL PROFILES ===");
    let mut stmt = conn.prepare("SELECT path, name, game_version, mod_loader, mod_loader_version, install_stage FROM profiles;")?;
    let profile_iter = stmt.query_map([], |row| {
        let path: String = row.get(0)?;
        let name: String = row.get(1)?;
        let game_version: String = row.get(2)?;
        let mod_loader: String = row.get(3)?;
        let mod_loader_version: Option<String> = row.get(4)?;
        let install_stage: String = row.get(5)?;
        Ok(format!("Path: {}, Name: {}, Game: {}, Loader: {}, LoaderVer: {:?}, Stage: {}",
                   path, name, game_version, mod_loader, mod_loader_version, install_stage))
    })?;

    for profile in profile_iter {
        println!("Profile: {}", profile?);
    }

    Ok(())
}
