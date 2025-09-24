use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = r"C:\Users\user\AppData\Roaming\AstralRinthApp\app.db";
    
    let conn = Connection::open(db_path)?;
    
    // Get current timestamp
    let now = chrono::Utc::now().timestamp_millis();
    
    // Try to insert with just the most basic fields
    match conn.execute(
        "INSERT OR REPLACE INTO profiles (path, name, game_version, mod_loader, install_stage, created, modified, groups, override_extra_launch_args, override_custom_env_vars) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            "test-profile",           // path
            "Test Profile",           // name
            "1.21.1",                // game_version
            "neoforge",              // mod_loader
            "installed",             // install_stage
            now,                     // created
            now,                     // modified
            "[]",                    // groups
            "[]",                    // override_extra_launch_args
            "{}"                     // override_custom_env_vars
        ]
    ) {
        Ok(rows_affected) => {
            println!("✓ Successfully inserted test profile with basic fields + groups + override_extra_launch_args + override_custom_env_vars ({} rows affected)", rows_affected);
        }
        Err(e) => {
            println!("✗ Failed to insert with basic fields + groups + override_extra_launch_args + override_custom_env_vars: {}", e);
        }
    }
    
    Ok(())
}
