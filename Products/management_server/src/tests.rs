#[cfg(test)]
mod tests {
    use actix_http::Request;
    use actix_web::{test, App, http, Error};
    use actix_web::dev::{Service, ServiceResponse};
    use crate::*;
    use dao::objects::Event;

    async fn create_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Connessione al DB e ottenimento collezione degli eventi
        println!("Connecting to MongoDB...");
        let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@beelive.mongo:27017").await.unwrap();
        println!("Connected to MongoDB!");
        let mongodb_events_collection = client.database("events").collection::<Event>("events");
        let mongodb_users_collection = client.database("users").collection::<User>("users");

        let data = data {
            counter: Cell::new(None),
            mongodb_events_collection,
            mongodb_users_collection,
        };

        // Inizializza l'applicazione per il test
        let app = test::init_service(
            App::new()
                .app_data(Data::new(data.clone()))
                .service(insert_event)
                .service(list_events_by_id)
                .service(modify_event)
                .service(delete_event)
        ).await;

        app
    }

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
            .insert_header(("Authorization", "Bearer valid_token"))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 200 OK
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    // 401 - Test modify_event senza token di autenticazione
    #[test]
    async fn insert_event_201() {

        // Ottenimento applicazione
        let app = create_app().await;

        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer valid_token"))
            .set_json("{\"id\":0,\"title\":\"Incontro G7\",\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"2024-03-15T06:00:00Z\",\"end\":\"2024-03-15T11:00:00Z\"}}]}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 201 Created
        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }

    // 400 - Test insert_event con body non valido
    #[test]
    async fn insert_event_400() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer valid_token"))
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
            .insert_header(("Authorization", "Bearer invalid_token"))
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
            .insert_header(("Authorization", "Bearer valid_token"))
            .set_json("{\"id\":0,\"title\":12345,\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"2024-03-15T06:00:00Z\",\"end\":\"2024-03-15T11:00:00Z\"}}]}")
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    }
}