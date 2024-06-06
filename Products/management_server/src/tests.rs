#[cfg(test)]
mod tests {
    use std::iter;
    use std::ops::Deref;
    use actix_http::Request;
    use actix_web::{test, App, http, Error};
    use actix_web::dev::{Service, ServiceResponse};
    use chrono::Utc;
    use geojson::{Feature, FeatureCollection};
    use geojson::Value::Point;
    use crate::*;
    use dao::objects::Event;
    use crate::dao::objects::{NullableDateTimeRange, RiskLevel, SubEvent};

    fn get_valid_event() -> Event {
        // Costruisci il JSON dell'evento
        Event::test_event()
    }

    const VALID_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImJlZWxpdmUtbWFuYWdlIiwidHlwIjoiSldUIn0.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoiQmVlTGl2ZSIsImNyZWF0ZWRUaW1lIjoiMjAyNC0wNi0wNlQwODowMjowOFoiLCJ1cGRhdGVkVGltZSI6IiIsImRlbGV0ZWRUaW1lIjoiIiwiaWQiOiIwYjI2ZWNlNi0zM2QxLTQ1YjMtYTRmMC1iYTY5ZDIxOGE1MzEiLCJ0eXBlIjoibm9ybWFsLXVzZXIiLCJwYXNzd29yZCI6IiIsInBhc3N3b3JkU2FsdCI6IiIsInBhc3N3b3JkVHlwZSI6ImFyZ29uMmlkIiwiZGlzcGxheU5hbWUiOiIiLCJmaXJzdE5hbWUiOiIiLCJsYXN0TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vY2RuLmNhc2Jpbi5vcmcvaW1nL2Nhc2Jpbi5zdmciLCJhdmF0YXJUeXBlIjoiIiwicGVybWFuZW50QXZhdGFyIjoiIiwiZW1haWwiOiIiLCJlbWFpbFZlcmlmaWVkIjpmYWxzZSwicGhvbmUiOiIiLCJjb3VudHJ5Q29kZSI6IiIsInJlZ2lvbiI6IiIsImxvY2F0aW9uIjoiIiwiYWRkcmVzcyI6W10sImFmZmlsaWF0aW9uIjoiIiwidGl0bGUiOiIiLCJpZENhcmRUeXBlIjoiIiwiaWRDYXJkIjoiIiwiaG9tZXBhZ2UiOiIiLCJiaW8iOiIiLCJsYW5ndWFnZSI6IiIsImdlbmRlciI6IiIsImJpcnRoZGF5IjoiIiwiZWR1Y2F0aW9uIjoiIiwic2NvcmUiOjAsImthcm1hIjowLCJyYW5raW5nIjoyLCJpc0RlZmF1bHRBdmF0YXIiOmZhbHNlLCJpc09ubGluZSI6ZmFsc2UsImlzQWRtaW4iOmZhbHNlLCJpc0ZvcmJpZGRlbiI6ZmFsc2UsImlzRGVsZXRlZCI6ZmFsc2UsInNpZ251cEFwcGxpY2F0aW9uIjoiYmVlbGl2ZV9tYW5hZ2UiLCJoYXNoIjoiIiwicHJlSGFzaCI6IiIsImFjY2Vzc0tleSI6IiIsImFjY2Vzc1NlY3JldCI6IiIsImdpdGh1YiI6IiIsImdvb2dsZSI6IiIsInFxIjoiIiwid2VjaGF0IjoiIiwiZmFjZWJvb2siOiIiLCJkaW5ndGFsayI6IiIsIndlaWJvIjoiIiwiZ2l0ZWUiOiIiLCJsaW5rZWRpbiI6IiIsIndlY29tIjoiIiwibGFyayI6IiIsImdpdGxhYiI6IiIsImNyZWF0ZWRJcCI6IiIsImxhc3RTaWduaW5UaW1lIjoiIiwibGFzdFNpZ25pbklwIjoiIiwicHJlZmVycmVkTWZhVHlwZSI6IiIsInJlY292ZXJ5Q29kZXMiOm51bGwsInRvdHBTZWNyZXQiOiIiLCJtZmFQaG9uZUVuYWJsZWQiOmZhbHNlLCJtZmFFbWFpbEVuYWJsZWQiOmZhbHNlLCJsZGFwIjoiIiwicHJvcGVydGllcyI6e30sInJvbGVzIjpbXSwicGVybWlzc2lvbnMiOltdLCJncm91cHMiOltdLCJsYXN0U2lnbmluV3JvbmdUaW1lIjoiIiwic2lnbmluV3JvbmdUaW1lcyI6MCwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwibm9uY2UiOiI0M3ZtaUtxb2ZqeGUiLCJ0YWciOiIiLCJzY29wZSI6Im9wZW5pZCIsImlzcyI6Imh0dHA6Ly85My40OS45Ni4xMzo5OTg3Iiwic3ViIjoiMGIyNmVjZTYtMzNkMS00NWIzLWE0ZjAtYmE2OWQyMThhNTMxIiwiYXVkIjpbIjcxMmI4YWFmZmQ5YzRjNzFhYjdhIl0sImV4cCI6MTcxODI2ODA2OSwibmJmIjoxNzE3NjYzMjY5LCJpYXQiOjE3MTc2NjMyNjksImp0aSI6ImFkbWluLzFhZDJjYmI3LWMzZGYtNDNkZC05ZTZiLTZmNmFmOTdjYjc3YSJ9.hZrY9DPhqClDBriHXnhqkV8CghxnEFZsq9q7yA0XFBu7nxA9eub2fBycfNu-3GqDhvgm4cix2d5CfW0iYGbO4YTq3MOKuN6S1H52c5wk7tZZeRIyovdE4TVrjhTleDlemtuB2T5hW75U9EH2al_3O7KFAeqVpECu8qAYNhPtpewA0MF46ah1cP4wqq-i57ZVRylrF6oLEFif3DNUT7zc7TcgtPgJf1RJAdCbRzDuDuGQzxJAfIMBAsRp_wTiIxhyGAZ2jkC6VJj02s5i7r1RiQuUUWNDsaZn_-bU-1tGgevy26f5Ec7sKbbF7YZ0QFPcuWAh8fbwSyzKj7-yq1JViiwXM8Q9RAny0r7QLMkv9__tZKwyFpoSAXDDIPzlFxDVc4OID1wMZW2w2KDKT6-ZZsD6tam-_s9dhqDvJJnPpJ7XUpUwX5ydgxrQFhW7yDWPYfWEFBQ2rUVetR6gGpnB6S6BEZAZlhxtWVOoTAfUy8ov1OGgEVLiPaKpairkK6xv4W5DdaCD57y3lDjweDnFQQzvJTqFiLhtbHbqQdygxk-fjDj3lIve0Xwn3p6yItNtP6CdCrHfGdzN4hNH7p2_06vixRAr2wZmYJ5OJGd_KhRwoov8ONLIay7CrYaObpUBbN93YseiAR-PN7SGgiY09X6hl3Y9o13dNR6BxPy9yKg";

    fn nice_response_error(response: ServiceResponse) -> String {
        format!("{:?}\n{:?}", response.response(), response.response().body())
    }

    fn get_invalid_event() -> Event {

        let now = Utc::now();

        Event {
            risk_level: RiskLevel::Alert,
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
        }
    }

    async fn create_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {

        // Connessione al DB e ottenimento collezione degli eventi
        println!("Connecting to MongoDB...");
        let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@localhost:27017").await.unwrap();
        println!("Connected to MongoDB!");
        let database = client.database("beelive_test");

        #[cfg(debug_assertions)]
            let base_cert_path = "assets";
        #[cfg(not(debug_assertions))]
            let base_cert_path = "/usr/share/beelive_management";

        let cert_path = format!("{}/CA/casdoor.pem", base_cert_path);
        let mut cert = File::open(cert_path).expect("Cannot open the CA pem file");

        let casdoor_client_id = "712b8aaffd9c4c71ab7a".to_string();

        let authenticator = Authenticator::from_file(Some(Algorithm::RS256), &mut cert, &casdoor_client_id)
            .expect("Cannot create the authenticator");


        let data = AppData {
            counter: Mutex::new(None),
            mongodb: database.into(),
            authenticator,
        };
        let data = Data::new(data);

        // Inizializza l'applicazione per il test
        let app = test::init_service(
            App::new()
                .app_data(data.clone()  )
                .service(insert_event)
                .service(list_events_by_id)
                .service(modify_event)
                .service(delete_event) // Correggere sta cosa
        ).await;

        app
    }

    // Inserzione nuovo evento prima di fare i test

    // 201 - Test insert_event con body valido
    #[test]
    async fn insert_event_201() {

        // Ottenimento applicazione
        let app = create_app().await;

        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .set_json(get_valid_event())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 201 Created
        assert_eq!(resp.status(), http::StatusCode::CREATED, "{:?}", nice_response_error(resp));
    }

// --- Test list_events ---
    // 401 - Test list_events senza token di autenticazione
    #[test]
    async fn list_events_401() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/list_events"
        let req = test::TestRequest::get()
            .uri("/list_events")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 403 Forbidden
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED, "{:?}", nice_response_error(resp));
    }

    // 403 - Test list_events con utente non autorizzato
    #[test]
    async fn list_events_403() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/list_events"
        let req = test::TestRequest::get()
            .uri("/list_events")
            .insert_header(("Authorization", "Bearer unauthorized_token"))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 403 Forbidden
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN, "{:?}", nice_response_error(resp));
    }

    // 422 - Test list_events con pagina negativa
    #[test]
    async fn list_events_422() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/list_events"
        let req = test::TestRequest::get()
            .uri("/list_events?page=-2")
            .insert_header(("Authorization", "Bearer " ))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY, "{:?}", nice_response_error(resp));
    }

    // 200 - Test list_events con pagina valida
    #[test]
    async fn list_events_200() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/list_events"
        let req = test::TestRequest::get()
            .uri("/list_events")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 200 OK
        assert_eq!(resp.status(), http::StatusCode::OK, "{:?}", nice_response_error(resp));
    }

// --- Test insert_event ---

    // 400 - Test insert_event con body non valido
    #[test]
    async fn insert_event_400() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .set_json("{evento sbagliato}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 400 Bad Request
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST, "{:?}", nice_response_error(resp));
    }

    // 401 - Test insert_event senza token di autenticazione
    #[test]
    async fn insert_event_401() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .set_json(get_valid_event())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 401 Unauthorized
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED, "{:?}", nice_response_error(resp));
    }

    // 403 - Test insert_event con utente non autorizzato
    #[test]
    async fn insert_event_403() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer unauthorized_token"))
            .set_json(get_valid_event())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 403 Forbidden
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN, "{:?}", nice_response_error(resp));
    }

    // 422 - Test insert_event con body non valido
    #[test]
    async fn insert_event_422() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .set_json(get_invalid_event())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY, "{:?}", nice_response_error(resp));
    }

// --- Test delete_event ---
    // 401 - Token di autenticazione non fornito
    #[test]
    async fn delete_event_401() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri("/delete_event/0")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 401 Unauthorized
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED, "{:?}", nice_response_error(resp));
    }

    // 403 - Utente non autorizzato o token non fornito
    #[test]
    async fn delete_event_403() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri("/delete_event/0")
            .insert_header(("Authorization", "Bearer unauthorized_token"))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 403 Forbidden
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN, "{:?}", nice_response_error(resp));
    }

    // 404 - Evento non esistente
    #[test]
    async fn delete_event_404() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri("/delete_event/1").insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 404 Not Found
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND, "{:?}", nice_response_error(resp));
    }

    // 418 - Evento non bloccato dall'utente che vuole fare modifiche
    #[test]
    async fn delete_event_418() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri("/delete_event/0")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 418 I'm a teapot
        assert_eq!(resp.status(), http::StatusCode::IM_A_TEAPOT, "{:?}", nice_response_error(resp));
    }

    /* Non implementato
    // 423 - Evento bloccato da un altro utente
    #[test]
    async fn delete_event_423() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri("/delete_event/0")
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 423 Locked
        assert_eq!(resp.status(), http::StatusCode::LOCKED);
    }
    */

// --- Test modify_event ---
    // 200 - Test modify_event con evento esistente
    #[test]
    async fn modify_event_200() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/0")
            .insert_header((http::header::AUTHORIZATION, "Bearer ".to_string()+VALID_TOKEN))
                .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .set_json(&serde_json::json!({
                "id": 0,
                "title": "Incontro G7",
                "summary": "Divieti di fermata e transito.",
                "description": "",
                "creator_id": 0,
                "polygons": {
                    "type": "FeatureCollection",
                    "features": []
                },
                "validity": {
                    "start": "1717670391",
                    "end": "1717670391"
                },
                "visibility": {
                    "start": null,
                    "end": "1717670391"
                },
                "categories": [],
                "subevents": [
                    {
                        "title": "Fase preparatoria",
                        "description": "",
                        "polygons": {
                            "type": "FeatureCollection",
                            "features": []
                        },
                        "validity": {
                            "start": "1717670391",
                            "end": "1717670391"
                        }
                    },
                    {
                        "title": "Arrivo delegazioni",
                        "description": "Descrizione",
                        "polygons": {
                            "type": "FeatureCollection",
                            "features": [
                                {
                                    "type": "Feature",
                                    "geometry": {
                                        "type": "Polygon",
                                        "coordinates": [[[11.12094551324845, 46.06685718388318]]]
                                    }
                                },
                                {
                                    "type": "Feature",
                                    "geometry": {
                                        "type": "Polygon",
                                        "coordinates": [[[11.12094551324845, 46.06685718388318]]]
                                    }
                                }
                            ]
                        },
                        "validity": {
                            "start": "2024-03-15T06:00:00Z",
                            "end": "2024-03-15T11:00:00Z"
                        }
                    }
                ]
            }))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 200 OK
        assert_eq!(resp.status(), http::StatusCode::OK, "{}", nice_response_error(resp));
    }

    // 401 - Token di autenticazione non fornito
    #[test]
    async fn modify_event_401() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/0")
            .set_json("{\"id\":0,\"title\":12345,\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"2024-03-15T06:00:00Z\",\"end\":\"2024-03-15T11:00:00Z\"}}]}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 401 Unauthorized
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED, "{:?}", nice_response_error(resp));
    }

    // 403 - Utente non autorizzato o token non fornito
    #[test]
    async fn modify_event_403() {
        // Ottenimento applicazione
        let app = create_app().await;



        // Crea una richiesta di test per la route "/modify_event"
        let mut req = test::TestRequest::put()
            .uri("/modify_event/0")
            .insert_header((http::header::AUTHORIZATION, "Bearer ".to_string()+VALID_TOKEN))
            .set_json(get_valid_event())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 200 OK
        assert_eq!(resp.status(), http::StatusCode::OK);
    }


    // 404 - Evento non esistente
    #[test]
    async fn modify_event_404() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/1")
            .insert_header((http::header::AUTHORIZATION, "Bearer ".to_string()+VALID_TOKEN))
            .set_json("{\"id\":1,\"title\":\"Incontro G7\",\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"1717670391\",\"end\":\"1717670391\"}}]}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 404 Not Found
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND, "{}", nice_response_error(resp));
    }

    // 418 - Evento non bloccato dall'utente che vuole fare modifiche
    #[test]
    async fn modify_event_418() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/0")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .set_json("{\"id\":0,\"title\":\"Incontro G7\",\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"1717670391\",\"end\":\"1717670391\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"2024-03-15T06:00:00Z\",\"end\":\"2024-03-15T11:00:00Z\"}}]}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 418 I'm a teapot
        assert_eq!(resp.status(), http::StatusCode::IM_A_TEAPOT, "{}", nice_response_error(resp));
    }

    // 422 - Evento non valido
    #[test]
    async fn modify_event_422() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/0")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .set_json("{\"id\":0,\"title\":12345,\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"2024-03-15T06:00:00Z\",\"end\":\"2024-03-15T11:00:00Z\"}}]}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY, "{}", nice_response_error(resp));
    }

    // 423 - Evento bloccato da un altro utente
    #[test]
    async fn modify_event_423() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/0")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .set_json(get_valid_event())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 423 Locked
        assert_eq!(resp.status(), http::StatusCode::LOCKED, "{}", nice_response_error(resp));
    }

    // Alla fine per l'eliminazione dell'evento di test
    // 200 - Test delete_event con evento esistente
    #[test]
    async fn delete_event_200() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri("/delete_event/0")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 200 OK
        assert_eq!(resp.status(), http::StatusCode::OK, "{}", nice_response_error(resp));
    }
}
