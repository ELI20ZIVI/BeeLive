use actix_web::{test, App};
use actix_web::http::StatusCode;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;

#[actix_web::test]
async fn test_get_events() {
    let mut app = test::init_service(
        App::new()
            .service(get_events)
    )
    .await;

    // -----------------------------------------------------
    // Esecuzione richiesta GET richiedendo tutti gli eventi
    let req = test::TestRequest::with_uri("/api/v3/events").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    // -------------------------------------------------------------------
    // Esecuzione richiesta GET richiedendo tutti gli eventi con parametri

    // mode: Combo tra preferenze in locale e in remoto
    // overwrite: Sovrascrive le preferenze in locale con quelle in remoto
    let req = test::TestRequest::with_uri("/api/v3/events?mode=overwrite").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    // combine: Combina le preferenze in locale e in remoto
    let req = test::TestRequest::with_uri("/api/v3/events?mode=combine").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    // ifempty: Se le preferenze in locale sono vuote, usa quelle in remoto
    let req = test::TestRequest::with_uri("/api/v3/events?mode=ifempty").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    // mode con valore non correto
    let req = test::TestRequest::with_uri("/api/v3/events?mode=nonvalido").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 400);

    // -------------------------------------------------------------------
    // addb: Lista di categorie (ID) da aggiungere ai filtraggi degli eventi broadcast

    // Lista di categorie valide
    let req = test::TestRequest::with_uri("/api/v3/events?addb=1,2").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    // Lista di categorie non valide
    let req = test::TestRequest::with_uri("/api/v3/events?addb=1,2,999").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 422); // ? 406

    // Lista di categorie non accettabili
    let req = test::TestRequest::with_uri("/api/v3/events?addb=1,2,uno").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 400);

    // -------------------------------------------------------------------
    // subb: Lista di categorie (ID) da rimuovere dai filtraggi degli eventi broadcast

    // Lista di categorie valide
    let req = test::TestRequest::with_uri("/api/v3/events?subb=1,2").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    // Lista di categorie non corrette
    let req = test::TestRequest::with_uri("/api/v3/events?subb=1,2,999").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 422);

    // Lista di categorie non accettabili
    let req = test::TestRequest::with_uri("/api/v3/events?subb=1,2,uno").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 400);

    // -------------------------------------------------------------------
    // addi: Lista di categorie (ID) da aggiungere ai filtraggi degli eventi individuali

    // Lista di categorie valide
    let req = test::TestRequest::with_uri("/api/v3/events?addi=1,2").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    // Lista di categorie non valide
    let req = test::TestRequest::with_uri("/api/v3/events?addi=1,999").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 422);

    // Lista di categorie non corrette
    let req = test::TestRequest::with_uri("/api/v3/events?addi=uno,2").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 400);

    // -------------------------------------------------------------------
    // subi: Lista di categorie (ID) da rimuovere dai filtraggi degli eventi individuali

    // Lista di categorie valide
    let req = test::TestRequest::with_uri("/api/v3/events?subi=1,2").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    // Lista di categorie non valide
    let req = test::TestRequest::with_uri("/api/v3/events?subi=1,999").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 422);

    // Lista di categorie non corrette
    let req = test::TestRequest::with_uri("/api/v3/events?subi=uno,2").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_get_event() {
    // Crea un'applicazione Actix per testare la funzione get_events
    let mut app = test::init_service(
        App::new()
            .service(get_events)
    )
    .await;

    // ------------------------------------------------------
    // Esecuzione richiesta GET richiedendo l'evento di prova
    let req = test::TestRequest::with_uri("/api/v3/event/0").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 200);

    // ----------------------------------------------------------
    // Esecuzione richiesta GET richiedendo un evento inesistente
    let req = test::TestRequest::with_uri("/api/v3/event/999").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status().as_u16(), 404);
}