use rocket::{get, post};
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};
use serde_derive::{Deserialize, Serialize};
use sqlx::{Sqlite};

#[derive(Database)]
#[database("day13")]
pub struct D13DB(sqlx::SqlitePool);

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Order<'a> {
    id: u32,
    region_id: u32,
    gift_name: &'a str,
    quantity: u32
}

#[get("/sql")]
pub async fn sql(mut db: Connection<D13DB>) -> String {
    let (id, ): (i32, ) = sqlx::query_as("SELECT 20231213;").fetch_one(&mut **db).await.unwrap();
    id.to_string()
}

#[post("/reset")]
pub async fn reset(mut db: Connection<D13DB>) {
    let _ = sqlx::query("DROP TABLE IF EXISTS orders;
        CREATE TABLE orders (
          id INT PRIMARY KEY,
          region_id INT,
          gift_name VARCHAR(50),
          quantity INT
        );").execute(&mut **db).await;
}

#[post("/orders", data = "<data>")]
pub async fn order_post(mut db: Connection<D13DB>, data: Json<Vec<Order<'_>>>) {
    for order in data.iter() {
        match sqlx::query(
            "INSERT INTO orders (id, region_id, gift_name, quantity)\
            VALUES ($1, $2, $3, $4);"
        )
            .bind(order.id)
            .bind(order.region_id)
            .bind(order.gift_name)
            .bind(order.quantity)
            .execute(&mut **db)
            .await {
            Ok(_) => {}
            Err(_) => {panic!("Cannot post {:?}", order)}
        }
    }
}


#[derive(Serialize)]
pub struct TotalOrders {
    total: i64
}

#[get("/orders/total")]
pub async fn order_total(mut db: Connection<D13DB>) -> Json<TotalOrders> {
    let (total, ) : (i64,) = sqlx::query_as("SELECT SUM(quantity) FROM orders").fetch_one(&mut **db).await.unwrap();
    Json::from(TotalOrders{
        total
    })
}


#[derive(Serialize)]
pub struct PopularOrder {
    popular: Option<String>
}

#[get("/orders/popular")]
pub async fn order_popular(mut db: Connection<D13DB>) -> Json<PopularOrder> {
    match sqlx::query_as::<Sqlite, (String, i64)>("SELECT gift_name, max(quantity) from (
                                         SELECT SUM(quantity) as quantity,
                                                gift_name
                                         FROM orders group by gift_name
                                     )").fetch_one(&mut **db).await {
        Ok((name, quantity)) if quantity > 0 => {
            Json::from(PopularOrder {
                popular: Some(name)
            })
        }
        _ => Json::from(PopularOrder {
            popular: None
        })
    }

}