use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::{Client, Collection, options::FindOneAndUpdateOptions, options::ReturnDocument};
use crate::model::Soldier;
use futures_util::stream::StreamExt;
use mongodb::bson::{doc, Document};

async fn get_next_sequence(client: &Client, db_name: &str, sequence_name: &str) -> Result<i64, mongodb::error::Error> {
    let counters: Collection<Document> = client.database(db_name).collection("counters");
    let filter = doc! { "_id": sequence_name };
    let update = doc! { "$inc": { "seq": 1 } };
    let options = FindOneAndUpdateOptions::builder()
        .upsert(true)
        .return_document(ReturnDocument::After)
        .build();

    let result = counters.find_one_and_update(filter, update).with_options(options).await?;
    
    match result {
        Some(doc) => {
            if let Ok(seq) = doc.get_i64("seq") {
                Ok(seq)
            } else if let Ok(seq) = doc.get_i32("seq") {
                Ok(seq as i64)
            } else {
                Err(mongodb::error::Error::custom("Failed to get sequence number from document"))
            }
        },
        None => Err(mongodb::error::Error::custom("Failed to find or create sequence document")),
    }
}

#[utoipa::path(
    get,
    path = "/soldiers",
    responses(
        (status = 200, description = "List all soldiers", body = [Soldier]),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/soldiers")]
pub async fn get_soldiers(client: web::Data<Client>, db_name: web::Data<String>) -> impl Responder {
    let collection: Collection<Soldier> = client.database(db_name.get_ref()).collection("soldiers");
    let mut cursor = match collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(err) => {
            eprintln!("Error finding soldiers: {:?}", err);
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    };

    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(soldier) => results.push(soldier),
            Err(err) => {
                eprintln!("Error iterating cursor: {:?}", err);
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        }
    }

    HttpResponse::Ok().json(results)
}

#[utoipa::path(
    post,
    path = "/soldiers",
    request_body = Soldier,
    responses(
        (status = 201, description = "Soldier created successfully"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/soldiers")]
pub async fn add_soldier(client: web::Data<Client>, db_name: web::Data<String>, soldier: web::Json<Soldier>) -> impl Responder {
    let mut new_soldier = soldier.into_inner();
    
    // Auto-increment logic
    match get_next_sequence(client.get_ref(), db_name.get_ref(), "soldier_id").await {
        Ok(next_id) => new_soldier.id = Some(next_id),
        Err(err) => {
            eprintln!("Error getting next sequence: {:?}", err);
            return HttpResponse::InternalServerError().body(format!("Failed to generate ID: {}", err));
        }
    }

    let collection: Collection<Soldier> = client.database(db_name.get_ref()).collection("soldiers");
    let result = collection.insert_one(new_soldier).await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => {
            eprintln!("Error inserting soldier: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
