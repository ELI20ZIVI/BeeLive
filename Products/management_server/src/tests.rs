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
    use crate::dao::objects::{NullableDateTimeRange, SubEvent};

    fn get_valid_event() -> Event {
        // Costruisci il JSON dell'evento
        Event::test_event()
    }

    fn get_invalid_event() -> Event {

        let now = Utc::now();

        Event {
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

        let data = AppData {
            counter: Cell::new(None),
            mongodb: database,
            authenticator,
        };

        // Inizializza l'applicazione per il test
        let app = test::init_service(
            App::new()
                .app_data(Data::new(data.clone()))
                .service(insert_event)
                .service(list_events_by_id)
                .service(modify_event)
                .service(crate::delete_event) // Correggere sta cosa
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
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .set_json(get_valid_event())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 201 Created
        assert_eq!(resp.status(), http::StatusCode::CREATED);
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
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
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
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN);
    }

    // 422 - Test list_events con pagina negativa
    #[test]
    async fn list_events_422() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/list_events"
        let req = test::TestRequest::get()
            .uri("/list_events?page=-2")
            .insert_header(("Authorization", "Bearer validToken"))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    }

    // 200 - Test list_events con pagina valida
    #[test]
    async fn list_events_200() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/list_events"
        let req = test::TestRequest::get()
            .uri("/list_events")
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 200 OK
        assert_eq!(resp.status(), http::StatusCode::OK);
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
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .set_json("{evento sbagliato}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 400 Bad Request
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    // 401 - Test insert_event senza token di autenticazione
    #[test]
    async fn insert_event_401() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .set_json("{evento}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 401 Unauthorized
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
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
            .set_json("{evento}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 403 Forbidden
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN);
    }

    // 422 - Test insert_event con body non valido
    #[test]
    async fn insert_event_422() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .set_json(get_invalid_event())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
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
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
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
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN);
    }

    // 404 - Evento non esistente
    #[test]
    async fn delete_event_404() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri("/delete_event/1")
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 404 Not Found
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    // 418 - Evento non bloccato dall'utente che vuole fare modifiche
    #[test]
    async fn delete_event_418() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri("/delete_event/0")
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 418 I'm a teapot
        assert_eq!(resp.status(), http::StatusCode::IM_A_TEAPOT);
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
            .insert_header((http::header::AUTHORIZATION, "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
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
                    "start": "2024-03-14T22:00:00Z",
                    "end": "2024-03-15T19:00:00Z"
                },
                "visibility": {
                    "start": null,
                    "end": "2024-03-15T19:00:00Z"
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
                            "start": "2024-03-14T22:00:00Z",
                            "end": "2024-03-15T19:00:00Z"
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
        assert_eq!(resp.status(), http::StatusCode::OK);
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
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }

    // 403 - Utente non autorizzato o token non fornito
    #[test]
    async fn modify_event_403() {
        // Ottenimento applicazione
        let app = create_app().await;



        // Crea una richiesta di test per la route "/modify_event"
        let mut req = test::TestRequest::put()
            .uri("/modify_event/0")
            .insert_header((http::header::AUTHORIZATION, "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
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
            .insert_header((http::header::AUTHORIZATION, "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .set_json("{\"id\":1,\"title\":\"Incontro G7\",\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"2024-03-15T06:00:00Z\",\"end\":\"2024-03-15T11:00:00Z\"}}]}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 404 Not Found
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    // 418 - Evento non bloccato dall'utente che vuole fare modifiche
    #[test]
    async fn modify_event_418() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/0")
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .set_json("{\"id\":0,\"title\":\"Incontro G7\",\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"2024-03-15T06:00:00Z\",\"end\":\"2024-03-15T11:00:00Z\"}}]}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 418 I'm a teapot
        assert_eq!(resp.status(), http::StatusCode::IM_A_TEAPOT);
    }

    // 422 - Evento non valido
    #[test]
    async fn modify_event_422() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/0")
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .set_json("{\"id\":0,\"title\":12345,\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"2024-03-15T06:00:00Z\",\"end\":\"2024-03-15T11:00:00Z\"}}]}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    }

    // 423 - Evento bloccato da un altro utente
    #[test]
    async fn modify_event_423() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/0")
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .set_json(get_valid_event())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 423 Locked
        assert_eq!(resp.status(), http::StatusCode::LOCKED);
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
            .insert_header(("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6ImNlcnQtYnVpbHQtaW4iLCJ0eXAiOiJKV1QifQ.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoicGlldHJvIiwiaWQiOiIwNDFkZWI2My01NTM0LTQxZjQtYWZmZi01Yzc4ZWUxYzhmNzQiLCJkaXNwbGF5TmFtZSI6IiIsImF2YXRhciI6Imh0dHBzOi8vYXMyLmZ0Y2RuLm5ldC92Mi9qcGcvMDQvMTAvNDMvNzcvMTAwMF9GXzQxMDQzNzczM19oZHE0UTNRT0g5dXdoMG1jcUFoUkZ6T0tmckNSMjRUYS5qcGciLCJlbWFpbCI6IiIsInBob25lIjoiIiwidG9rZW5UeXBlIjoiYWNjZXNzLXRva2VuIiwic2NvcGUiOiJvcGVuaWQiLCJpc3MiOiJodHRwOi8vMTkyLjE2OC4xNDQuNDM6ODAwMCIsInN1YiI6IjA0MWRlYjYzLTU1MzQtNDFmNC1hZmZmLTVjNzhlZTFjOGY3NCIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTYzMzQzNjcsIm5iZiI6MTcxNTcyOTU2NywiaWF0IjoxNzE1NzI5NTY3LCJqdGkiOiJhZG1pbi85NGYzMTY2OS01NzQ3LTQzOTYtOTdlNi01NzkwMmQ5ZDE3MTcifQ.ugWwX3IXeIQfj-dYJpj-FVMcDaMlAgPjMopeZTOWNKtoSViH95A0WTx8tEHeR6bqQCNwM0Qs1IMAdnkIddS9raO9vUrxOt-DNqU9FYUTzqdcFxQNN_PGH-cyhbGftgI-tqeP5iZN_Gg4pgpJUMWfepI5sP02IViJd4YilER1SWXoyogzU4gsNmE6X-NJAFYoiR58wTz6emc6neHZSR4RrBsjMkWiIEMebkiOw4nbI28yrT01HbVfhsOOOx9cs3POYwYJIUCwXxNNe2Xf7pPE-A1cuHfO_QcbnYrz1lfNA_8804xc2sGWBi8AlDRjFcTjXPD8fNI9MwxLmJ2BpSm3AglNpQj2vxlNl3QajPxCxG8DiVNr5jSdOTGa5lcpePqJ-BJtVr1k5ijfxQvuxCeaUDRyMqiprvG6IBwP-LB7VKwh3fQTqWAbauf7uxVOUQ3D0is8-lYebYNuy57_9YuC_8Ler3lFELnqfONyPXsWw8rCE6Pczk3NuC3WLytxGSjNDDCM9PN2roNAYixHRDTxGIq8nx1C7rWo58kPXKRFd0y2RtPJC3hnRcbavqvkEdkoIGv8u5IWJlL7-luqGmeiJ4pt3KWUCGvQODiEXz0h9PTNptnDYpIGGCxKBKTX0UVs0EGY3mWYAzDWG63U_SZcG8harEhhK8CHLRCNFCscyy8"))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 200 OK
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
