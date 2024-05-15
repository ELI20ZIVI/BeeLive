use actix_web::{test, App};
use dao::objects::Event;
use your_module::{get_event, get_events};
use mongodb::{results::InsertOneResult, Collection};

#[actix_rt::test]
async fn test_insert_new_event() {
    let mut app = test::init_service(
        App::new()
            .service(insert_new_event)
    )
    .await;

    // -----------------------------------------------------
    // Esecuzione richiesta POST per inserire un nuovo evento
    let req = test::TestRequest::post()
        .uri("/api/v3/insert_new_event")
        .set_json(&Event {
            id: 0,
            title: "test".to_string(),
            description: "an amazing description".to_string(),
            remote_document: Some("test remote document . org".to_string()),
            summary: "a long summary".to_string(),
            validity: NullableDateTimeRange::new(Some(now), Some(now)),
            visibility: NullableDateTimeRange::new(Some(now), Some(now)),
            locked_by: None,
            polygons: FeatureCollection::from_iter(iter::once(Feature::from(Point(vec![10.0, 6.0])))),
            creator_id: 0,
            subevents: vec![],
            categories: vec![],
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);
    // -------------------------------------------------------------------------------
    // Esecuzione richiesta POST per inserire un nuovo evento con parametri non validi
    let req = test::TestRequest::post()
        .uri("/api/v3/insert_new_event")
        .set_json(&Event {
            id: "0",
            title: "test",
            description: descrizione,
            remote_document: Some("test remote document . org".to_string()),
            summary: "a long summary".to_string(),
            validity: "01/01/2024".to_string(),
            visibility: NullableDateTimeRange::new(Some(now), Some(now)),
            locked_by: None,
            polygons: FeatureCollection::from_iter(iter::once(Feature::from(Point(vec![10.0, 6.0])))),
            creator_id: 0,
            subevents: vec![],
            categories: vec![],
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 400);

    // -------------------------------------------------------------------------------
    // Esecuzione richiesta POST per inserire un nuovo evento uguale ad uno già presente
    let req = test::TestRequest::post()
        .uri("/api/v3/insert_new_event")
        .set_json(&Event {
            id: 0,
            title: "test".to_string(),
            description: "an amazing description".to_string(),
            remote_document: Some("test remote document . org".to_string()),
            summary: "a long summary".to_string(),
            validity: NullableDateTimeRange::new(Some(now), Some(now)),
            visibility: NullableDateTimeRange::new(Some(now), Some(now)),
            locked_by: None,
            polygons: FeatureCollection::from_iter(iter::once(Feature::from(Point(vec![10.0, 6.0])))),
            creator_id: 0,
            subevents: vec![],
            categories: vec![],
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 406);

    // Rimozione dell'evento 0
    mongobd_collection.delete_one(doc! {"id": 0}, None).await.unwrap();
}