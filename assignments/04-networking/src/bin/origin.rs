use aspirin_eats::db::AspirinEatsDb;

/// Change this path to match where you want to store the database file
const DB_PATH: &str =
    "/home/amit/Documents/code/aspirin/dev-aspirin/assignments/05-networking/aspirin_eats.db";

fn main() {
    let db = AspirinEatsDb::from_path(DB_PATH).expect("Failed to open database");
}
