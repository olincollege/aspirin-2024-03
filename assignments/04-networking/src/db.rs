use std::path::Path;
use std::str::FromStr;

use rusqlite::{Connection, Result};

use crate::food::*;

pub struct AspirinEatsDb {
    conn: Connection,
}

impl AspirinEatsDb {
    /// Create a new AspirinEatsDb instance from a given path
    /// If the database does not exist, it will be created
    pub fn from_path<P>(db_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let db = Self {
            conn: Connection::open(db_path)?,
        };
        db.create_table()?;
        Ok(db)
    }

    /// Create a new AspirinEatsDb instance in memory. Useful for testing
    pub fn in_memory() -> Result<Self> {
        let db = Self {
            conn: Connection::open_in_memory()?,
        };
        db.create_table()?;
        Ok(db)
    }

    fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS orders (
            id	        INTEGER NOT NULL,
            customer	TEXT NOT NULL,
            food        TEXT NOT NULL,
            status	    TEXT NOT NULL,
            total       REAL NOT NULL,
            PRIMARY KEY(id AUTOINCREMENT)
        )",
            [], // no params for this query
        )?;
        Ok(())
    }
}

impl AspirinEatsDb {
    /// Insert a new Order into the database
    pub fn add_order(&self, order: Order) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO orders (customer, food, status, total) VALUES (?1, ?2, ?3, ?4)",
            [
                order.customer,
                serde_json::to_string(&order.food).expect("Failed to serialize food"),
                serde_json::to_string(&order.status).expect("Failed to serialize status"),
                order.total.to_string(),
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Get an order by ID from the database
    pub fn get_order(&self, id: i64) -> Result<Option<Order>> {
        let mut stmt = self
            .conn
            .prepare("SELECT customer, food, status, total FROM orders WHERE id = ?1")?;
        let mut rows = stmt.query([&id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Order {
                id: Some(id),
                customer: row.get(0)?,
                food: {
                    let food_str: String = row.get(1)?;
                    serde_json::from_str(&food_str).expect("db should contain valid json")
                },
                status: {
                    let status: String = row.get(2)?;
                    OrderStatus::from_str(&status).expect("db should contain valid status")
                },
                total: row.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Remove an order by ID from the database
    pub fn remove_order(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM orders WHERE id = ?1", [&id])?;
        Ok(())
    }

    /// Remove all orders from the database
    pub fn reset_orders(&self) -> Result<()> {
        self.conn.execute("DELETE FROM orders", [])?;
        self.conn.execute(
            "UPDATE SQLITE_SEQUENCE SET SEQ='0' WHERE NAME='orders';",
            [],
        )?;
        Ok(())
    }

    /// Get all orders from the database
    pub fn get_all_orders(&self) -> Result<Vec<Order>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, customer, food, status, total FROM orders")?;

        let order_iter = stmt.query_map([], |row| {
            Ok(Order {
                id: row.get(0)?,
                customer: row.get(1)?,
                food: {
                    let food_str: String = row.get(2)?;
                    serde_json::from_str(&food_str).expect("db should contain valid json")
                },
                status: {
                    let status: String = row.get(3)?;
                    OrderStatus::from_str(&status).expect("db should contain valid status")
                },
                total: row.get(4)?,
            })
        })?;

        Ok(order_iter.map(Result::unwrap).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_order() -> Order {
        Order {
            id: None,
            customer: "Amit".to_string(),
            food: vec![MenuItem::Fries, MenuItem::Drink],
            status: OrderStatus::Pending,
            total: 8.0,
        }
    }

    #[test]
    fn test_add_get_order() {
        let db = AspirinEatsDb::in_memory().unwrap();
        let mut order = get_test_order();

        order.id = Some(db.add_order(order.clone()).unwrap());

        let got = db.get_order(order.id.unwrap()).unwrap().unwrap();
        assert_eq!(got, order);
    }

    #[test]
    fn test_get_all_orders() {
        let db = AspirinEatsDb::in_memory().unwrap();
        let mut order1 = get_test_order();
        let mut order2 = get_test_order();

        order1.id = Some(db.add_order(order1.clone()).unwrap());
        order2.id = Some(db.add_order(order2.clone()).unwrap());

        let got = db.get_all_orders().unwrap();
        assert_eq!(got, vec![order1, order2]);
    }

    #[test]
    fn test_remove_order() {
        let db = AspirinEatsDb::in_memory().unwrap();
        let order = get_test_order();

        let id = db.add_order(order.clone()).unwrap();

        db.remove_order(id).unwrap();
        let got = db.get_order(id).unwrap();
        assert_eq!(got, None);
    }

    #[test]
    fn test_reset_orders() {
        let db = AspirinEatsDb::in_memory().unwrap();
        for _id in 0..5 {
            db.add_order(get_test_order()).unwrap();
        }

        db.reset_orders().unwrap();
        let orders = db.get_all_orders().unwrap();
        assert_eq!(orders.len(), 0);
    }
}
