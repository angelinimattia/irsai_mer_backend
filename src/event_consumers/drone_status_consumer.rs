use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use futures_lite::stream::StreamExt;
use std::env;
use log::{debug, warn};
use crate::models::drone_status::DroneStatus;
use crate::db::MongoDB;

pub async fn connect_and_consume() {
    let addr = env::var("RABBIT_MQ_ENDPOINT").unwrap();
    let connection = Connection::connect(&addr, ConnectionProperties::default()).await.expect("Failed to connect");
    let channel = connection.create_channel().await.expect("Failed to create channel");
    
    let queue_name = env::var("RABBIT_MQ_QUEUE_NAME_DRONE_STATUS").unwrap();
    let queue = channel
        .queue_declare(
            &queue_name,    
            QueueDeclareOptions::default(),
                FieldTable::default(),
            )
        .await
        .expect("Failed to declare queue");

    let mut consumer = channel
        .basic_consume(
            queue.name().as_str(),
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to start consumer");
    
    let db = MongoDB::new()
        .await
        .expect("Falied to connect to the Database");


    while let Some(delivery) = consumer.next().await {
        let delivery = match delivery  {
            Ok(delivery) => delivery,
            Err(error_message) => {
                warn!("Error while reading the message in the Drone Consumer:\n {}", error_message);
                continue;
            }
        };

        let message = String::from_utf8_lossy(&delivery.data).to_string();
        let drone_status = serde_json::from_str::<DroneStatus>(&message);
        let Ok(drone_status) = drone_status else {
            let _ = delivery.nack(BasicNackOptions { multiple: false, requeue: false }).await;
            debug!("drone_status_conusmer: Nack -> Could not deserialise the incoming message: \n {}", message);
            continue; 
        }; 

        let drone_statues = db.get_drone_status();
        let result = drone_statues
            .insert_one(drone_status)
            .await;
        
        let Ok(_) = result else {
            let _ = delivery.nack(BasicNackOptions { multiple: false, requeue: true });
            debug!("drone_status_consmer: Nack -> Issues in writing in the DB (requeue)");
            continue;
        };
        
        // send succesfull ack!
        let _ = delivery.ack(BasicAckOptions::default()).await;
    }
}
