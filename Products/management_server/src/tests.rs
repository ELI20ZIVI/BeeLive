#[cfg(test)]
mod tests {
    use std::iter;
    use std::time::UNIX_EPOCH;
    use actix_http::Request;
    use actix_web::{test, App, http, Error};
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::test::TestRequest;
    use chrono::Utc;
    use geojson::{Feature, FeatureCollection};
    use geojson::Value::Point;
    use mongodb::bson::Document;
    use mongodb::Database;
    use crate::*;
    use dao::objects::Event;
    use crate::dao::objects::{NullableDateTimeRange, RiskLevel};

    fn get_valid_event(id: i32) -> Event {
        // Costruisci il JSON dell'evento
        let mut event = Event::test_event();
        event.id = id;
        event
    }

    fn get_a_different_valid_event(id: i32) -> Event {
        let mut event = Event::test_event();
        event.id = id;
        event.title = "A Different Test Event".to_string();
        event.description = "an amazing description, but a little different.".to_string();
        event
    }

    fn delete_event_request(id: i32) -> Request {
        TestRequest::delete()
            .uri(format!("/delete_event/{}", id).as_str())
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .to_request()
    }

    fn insert_event_request(id: i32) -> Request {
        TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .set_json(get_valid_event(id))
            .to_request()
    }

    const VALID_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImJlZWxpdmUtbWFuYWdlIiwidHlwIjoiSldUIn0.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoiQmVlTGl2ZSIsImNyZWF0ZWRUaW1lIjoiMjAyNC0wNi0wNlQwODowMjowOFoiLCJ1cGRhdGVkVGltZSI6IjIwMjQtMDYtMDlUMjE6MDg6NTZaIiwiZGVsZXRlZFRpbWUiOiIiLCJpZCI6IjBiMjZlY2U2LTMzZDEtNDViMy1hNGYwLWJhNjlkMjE4YTUzMSIsInR5cGUiOiJub3JtYWwtdXNlciIsInBhc3N3b3JkIjoiIiwicGFzc3dvcmRTYWx0IjoiIiwicGFzc3dvcmRUeXBlIjoiYXJnb24yaWQiLCJkaXNwbGF5TmFtZSI6IiIsImZpcnN0TmFtZSI6IiIsImxhc3ROYW1lIjoiIiwiYXZhdGFyIjoiaHR0cHM6Ly9jZG4uY2FzYmluLm9yZy9pbWcvY2FzYmluLnN2ZyIsImF2YXRhclR5cGUiOiIiLCJwZXJtYW5lbnRBdmF0YXIiOiIiLCJlbWFpbCI6IiIsImVtYWlsVmVyaWZpZWQiOmZhbHNlLCJwaG9uZSI6IiIsImNvdW50cnlDb2RlIjoiIiwicmVnaW9uIjoiIiwibG9jYXRpb24iOiIiLCJhZGRyZXNzIjpbXSwiYWZmaWxpYXRpb24iOiIiLCJ0aXRsZSI6IiIsImlkQ2FyZFR5cGUiOiIiLCJpZENhcmQiOiIiLCJob21lcGFnZSI6IiIsImJpbyI6IiIsImxhbmd1YWdlIjoiIiwiZ2VuZGVyIjoiIiwiYmlydGhkYXkiOiIiLCJlZHVjYXRpb24iOiIiLCJzY29yZSI6MCwia2FybWEiOjAsInJhbmtpbmciOjIsImlzRGVmYXVsdEF2YXRhciI6ZmFsc2UsImlzT25saW5lIjpmYWxzZSwiaXNBZG1pbiI6ZmFsc2UsImlzRm9yYmlkZGVuIjpmYWxzZSwiaXNEZWxldGVkIjpmYWxzZSwic2lnbnVwQXBwbGljYXRpb24iOiJiZWVsaXZlX21hbmFnZSIsImhhc2giOiIiLCJwcmVIYXNoIjoiIiwiYWNjZXNzS2V5IjoiIiwiYWNjZXNzU2VjcmV0IjoiIiwiZ2l0aHViIjoiIiwiZ29vZ2xlIjoiIiwicXEiOiIiLCJ3ZWNoYXQiOiIiLCJmYWNlYm9vayI6IiIsImRpbmd0YWxrIjoiIiwid2VpYm8iOiIiLCJnaXRlZSI6IiIsImxpbmtlZGluIjoiIiwid2Vjb20iOiIiLCJsYXJrIjoiIiwiZ2l0bGFiIjoiIiwiY3JlYXRlZElwIjoiIiwibGFzdFNpZ25pblRpbWUiOiIiLCJsYXN0U2lnbmluSXAiOiIiLCJwcmVmZXJyZWRNZmFUeXBlIjoiIiwicmVjb3ZlcnlDb2RlcyI6bnVsbCwidG90cFNlY3JldCI6IiIsIm1mYVBob25lRW5hYmxlZCI6ZmFsc2UsIm1mYUVtYWlsRW5hYmxlZCI6ZmFsc2UsImxkYXAiOiIiLCJwcm9wZXJ0aWVzIjp7fSwicm9sZXMiOltdLCJwZXJtaXNzaW9ucyI6W10sImdyb3VwcyI6W10sImxhc3RTaWduaW5Xcm9uZ1RpbWUiOiIiLCJzaWduaW5Xcm9uZ1RpbWVzIjowLCJ0b2tlblR5cGUiOiJhY2Nlc3MtdG9rZW4iLCJub25jZSI6ImZFY0xlMHI0WW1ERyIsInRhZyI6IiIsInNjb3BlIjoib3BlbmlkIiwiaXNzIjoiaHR0cDovLzkzLjQ5Ljk2LjEzOjk5ODciLCJzdWIiOiIwYjI2ZWNlNi0zM2QxLTQ1YjMtYTRmMC1iYTY5ZDIxOGE1MzEiLCJhdWQiOlsiNzEyYjhhYWZmZDljNGM3MWFiN2EiXSwiZXhwIjoxNzc4NjE2MzYzLCJuYmYiOjE3MTgxMzYzNjMsImlhdCI6MTcxODEzNjM2MywianRpIjoiYWRtaW4vMjExYjQ0Y2YtNzA0Yi00NTljLTk5NGYtNTFiYTM4ZWY1ZDAyIn0.fmYPj-XKlKrCTxEdyGARPKThjSOAL-u2Yz5JTBBHUPlr4oODQofx7q8BzO9kI42JvUVHrgGyXKuwcje1u-ju1Ka8BJqNzpUmqQYmlnZ05z8YBur6gslB-BVBDlyoIGU-vS6hYnBBBLWg51YvXTw9Oh25YtsT0xTP_qXWOO8TbZ3tEebe0zwqn-S6gA6FE4DT8H_1aPAfqAgikEPVVBgl1SlzOA78UcLEfNOOCn8TUR4dbTTezppQyoS9yJDolT-zuszeRUqngQMk__S9UqrWQOpFn5zp6LCMOataXBjNZbDm75azK4nPVAwXa5yvLPQJSb6jgEWv5_tFBiIE3m0EgwUCz-TJDcoWsBtGPntNn-ycu4ZrOEVDlT7ycvnlrul5qfFiKoI2hyICRtxWmv9plZcySmiB6EjirZ1O2VbXkoxOsRfrfZDTVRwUjGQp95ICBFw6QoAPs636UX11Tzexfbu6Ce_UjDGLlb9grC61ESEn_sjNahp-kqE5UcVDv8QN6O6DBUNtBe74TrIITNSb3BBjfQfYZebW5Tvq8eY9VXnjHz0FO1yn9QLPyfE1Orw6HgB5FvoxXRBkeW1HkGRqJ9HEihjygyfRDcAWekh8pfaj16h0-ZHd9iw3QSiFunXYBXaoHlqw0rA_gOjWyRrnoyGAHFDRYgzAhvUcwXCBBvk";
    const UNAUTHORIZED_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImJlZWxpdmUtbWFuYWdlIiwidHlwIjoiSldUIn0.eyJvd25lciI6ImJlZWxpdmUiLCJuYW1lIjoidW5hdXRoel91c2VyIiwiY3JlYXRlZFRpbWUiOiIyMDI0LTA2LTA5VDE2OjIyOjA3WiIsInVwZGF0ZWRUaW1lIjoiIiwiZGVsZXRlZFRpbWUiOiIiLCJpZCI6IjA0NDhlMGFiLTU3MjctNDY5Yi1iNWY5LWVmNjMxZTE0NGIwYyIsInR5cGUiOiJub3JtYWwtdXNlciIsInBhc3N3b3JkIjoiIiwicGFzc3dvcmRTYWx0IjoiIiwicGFzc3dvcmRUeXBlIjoiYXJnb24yaWQiLCJkaXNwbGF5TmFtZSI6IiIsImZpcnN0TmFtZSI6IiIsImxhc3ROYW1lIjoiIiwiYXZhdGFyIjoiaHR0cHM6Ly9jZG4uY2FzYmluLm9yZy9pbWcvY2FzYmluLnN2ZyIsImF2YXRhclR5cGUiOiIiLCJwZXJtYW5lbnRBdmF0YXIiOiIiLCJlbWFpbCI6IiIsImVtYWlsVmVyaWZpZWQiOmZhbHNlLCJwaG9uZSI6IiIsImNvdW50cnlDb2RlIjoiIiwicmVnaW9uIjoiIiwibG9jYXRpb24iOiIiLCJhZGRyZXNzIjpbXSwiYWZmaWxpYXRpb24iOiIiLCJ0aXRsZSI6IiIsImlkQ2FyZFR5cGUiOiIiLCJpZENhcmQiOiIiLCJob21lcGFnZSI6IiIsImJpbyI6IiIsImxhbmd1YWdlIjoiIiwiZ2VuZGVyIjoiIiwiYmlydGhkYXkiOiIiLCJlZHVjYXRpb24iOiIiLCJzY29yZSI6MCwia2FybWEiOjAsInJhbmtpbmciOjMsImlzRGVmYXVsdEF2YXRhciI6ZmFsc2UsImlzT25saW5lIjpmYWxzZSwiaXNBZG1pbiI6ZmFsc2UsImlzRm9yYmlkZGVuIjpmYWxzZSwiaXNEZWxldGVkIjpmYWxzZSwic2lnbnVwQXBwbGljYXRpb24iOiJiZWVsaXZlX21hbmFnZSIsImhhc2giOiIiLCJwcmVIYXNoIjoiIiwiYWNjZXNzS2V5IjoiIiwiYWNjZXNzU2VjcmV0IjoiIiwiZ2l0aHViIjoiIiwiZ29vZ2xlIjoiIiwicXEiOiIiLCJ3ZWNoYXQiOiIiLCJmYWNlYm9vayI6IiIsImRpbmd0YWxrIjoiIiwid2VpYm8iOiIiLCJnaXRlZSI6IiIsImxpbmtlZGluIjoiIiwid2Vjb20iOiIiLCJsYXJrIjoiIiwiZ2l0bGFiIjoiIiwiY3JlYXRlZElwIjoiIiwibGFzdFNpZ25pblRpbWUiOiIiLCJsYXN0U2lnbmluSXAiOiIiLCJwcmVmZXJyZWRNZmFUeXBlIjoiIiwicmVjb3ZlcnlDb2RlcyI6bnVsbCwidG90cFNlY3JldCI6IiIsIm1mYVBob25lRW5hYmxlZCI6ZmFsc2UsIm1mYUVtYWlsRW5hYmxlZCI6ZmFsc2UsImxkYXAiOiIiLCJwcm9wZXJ0aWVzIjp7fSwicm9sZXMiOltdLCJwZXJtaXNzaW9ucyI6W10sImdyb3VwcyI6W10sImxhc3RTaWduaW5Xcm9uZ1RpbWUiOiIiLCJzaWduaW5Xcm9uZ1RpbWVzIjowLCJ0b2tlblR5cGUiOiJhY2Nlc3MtdG9rZW4iLCJ0YWciOiIiLCJzY29wZSI6InByb2ZpbGUiLCJpc3MiOiJodHRwOi8vOTMuNDkuOTYuMTM6OTk4NyIsInN1YiI6IjA0NDhlMGFiLTU3MjctNDY5Yi1iNWY5LWVmNjMxZTE0NGIwYyIsImF1ZCI6WyI3MTJiOGFhZmZkOWM0YzcxYWI3YSJdLCJleHAiOjE3MTg1NTQ5NDEsIm5iZiI6MTcxNzk1MDE0MSwiaWF0IjoxNzE3OTUwMTQxLCJqdGkiOiJhZG1pbi81NWU5MDExOC05ZDdiLTQxZGUtODIyNy1hZTlkYzM1N2Q2NzMifQ.TErdd8Qq4y6w6syWBuWlumJqddAAHwBvsayy31urC0L8s15RmfTa_PvkjZzXfLZ6l76COCIkWCVpjkyg9jRcnoyGJ2oJz0nDRwyIu-_s7Y_fSNd_ZrhbEAJ3wRnA7RCYEOuyC84zZ3ik0mJ2GSoYWze86zV5KGzAMGex2em1xUNmDBAd0_KWEsfCDp0Eue2oWBmr7Hi-7K5uygstUQrJmue3DV5ON9mR0dYmmuWmDUU_XLrkZyd2dn-joJDGj59Ht3DWuAknZ4IQ1RekVtDwOGjc5c224D1ufmR7QUQslR2EVVu_iE2IM_TBAZCwFXxYJa0HPaiFBVjdk58MEfumrs0xkFSt-XzsDdk_M_vzXS4TtYxmqVSpSz3wKRxjetDlhULRQSkKQ8DEHfmqL-OhIPlfF5NAZMFVI1H5Zi3I31zFiRbKwUPJk5CIyqtD3HsXb1SyxNLHTVWJuAF8bduaKGPs-0ai0ZO4saXYyewHqU9UGSGCYHtOpgt3-NcopjlU_DSDL2lbuRBC9YKjvEg5lEnW-bM_gyUpizp0WzuROWgqAUReJz22T_a5G1wlYdL_mpce05KYSeBxU6XheSvOvwuBiHqMu0U4uZK8hHqXRm2D_vHX8G69ZKco_HRNczvBKG0QiZHFMLNPHColS8qAdBgdIz0zSBrX3KqbZL0IJ78";

    fn nice_response_error(response: ServiceResponse) -> String {
        format!("{:?}\n{:?}", response.response(), response.response().body())
    }

    fn get_invalid_event(id: i32) -> Event {

        unimplemented!();
        
        let now = Utc::now();
        let start_of_time = chrono::DateTime::<Utc>::from(UNIX_EPOCH);

        Event {
            risk_level: RiskLevel::Alert,
            id: id,
            title: "test".to_string(),
            description: "an amazing description".to_string(),
            remote_document: Some("test remote document . org".to_string()),
            summary: "a long summary".to_string(),
            validity: NullableDateTimeRange::new(Some(now), Some(start_of_time)),
            visibility: NullableDateTimeRange::new(Some(now), Some(now)),
            locked_by: None,
            polygons: FeatureCollection::from_iter(iter::once(Feature::from(Point(vec![10.0, 6.0])))),
            creator_id: 0,
            subevents: vec![],
            categories: vec![],
        }
    }

    async fn create_db() -> Database {
        let client = Client::with_uri_str("mongodb://BeeLive:BeeLive@localhost:27017").await.unwrap();
        client.database("beelive_test")
    }

    async fn create_dao() -> Dao {
        create_db().await.into()
    }

    async fn create_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {

        // Connessione al DB e ottenimento collezione degli eventi
        let database = create_db().await;

        let database : Dao = database.into();
        database.users.drop(None).await.unwrap();

        database.authorized_users.drop(None).await.unwrap();
        database.authorized_users.insert_one(User{
            id: "0b26ece6-33d1-45b3-a4f0-ba69d218a531".to_owned(),
            username: "BeeLive".to_owned(),
            email: "example@example.com".to_owned(),
            categories: vec![],
        }, None).await.unwrap();

        assert_ne!(
            database.authorized_users.count_documents(None, None).await.unwrap(),
            0, 
            "No authorized user found in the 'beelive_test/authorized_users' collection, which will obviously make all the tests fail.\
            did you run the 'management_server/scripts/mongobd_create_test_authorized_user.py' script?"
        );

        database.events.drop(None).await.unwrap();
        assert_eq!(
            database.events.count_documents(None, None).await.unwrap(),
            0,
            "The events collection should have been dropped"
        );

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
            mongodb: database,
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

// --- Test list_events ---


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
            .insert_header(("Authorization", "Bearer ".to_string()+UNAUTHORIZED_TOKEN))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 403 Forbidden
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN, "{:?}", nice_response_error(resp));
    }

    // 422 - Test list_events con pagina invalida
    #[test]
    async fn list_events_422() {

        // Ottenimento applicazione
        let app = create_app().await;

        // Crea una richiesta di test per la route "/list_events"
        let req = test::TestRequest::get()
            .uri("/list_events?page=0")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY, "{:?}", nice_response_error(resp));
    }

// --- Test insert_event ---

    // 201 - Test insert_event con dati validi
    #[test]
    async fn insert_event_201() {

        // Ottenimento applicazione
        let app = create_app().await;

        let id = 0;

        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .set_json(get_valid_event(id))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 201 Created
        assert_eq!(resp.status(), http::StatusCode::CREATED, "{:?}", nice_response_error(resp));
    }

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
        let id = 0;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .set_json(get_valid_event(id))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 401 Unauthorized
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED, "{:?}", nice_response_error(resp));
    }

    // 403 - Test insert_event con un token valido che però è per un utente non autorizzato
    #[test]
    async fn insert_event_403() {

        // Ottenimento applicazione
        let app = create_app().await;

        let id= 0;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer ".to_string()+UNAUTHORIZED_TOKEN))
            .set_json(get_valid_event(id))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 403 Forbidden
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN, "{:?}", nice_response_error(resp));
    }

    // 422 - Test insert_event con un evento non valido
    #[test]
    async fn insert_event_422() {
        
        // Ottenimento applicazione
        let app = create_app().await;

        let id = 0;

        // Crea una richiesta di test per la route "/insert_new_event"
        let req = test::TestRequest::post()
            .uri("/insert_new_event")
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .set_json(get_invalid_event(id))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY, "{:?}", nice_response_error(resp));
    }

// --- Test delete_event ---

    // 200 - Test delete_event con evento esistente
    #[test]
    async fn delete_event_200() {

        // Ottenimento applicazione
        let app = create_app().await;

        // no need to test this, if it fails then the ::delete request fails too
        let id = 0;
        test::call_service(&app, insert_event_request(id)).await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri(format!("/delete_event/{}", id).as_str())
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 200 OK
        assert_eq!(resp.status(), http::StatusCode::OK, "{}", nice_response_error(resp));

        let db = create_dao().await;
        assert_eq!(db.events.count_documents(None, None).await.unwrap(), 0, "events exist after deletion");
    }

    // 401 - Token di autenticazione non fornito
    #[test]
    async fn delete_event_401() {

        // Ottenimento applicazione
        let app = create_app().await;

        let id = 0;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri(format!("/delete_event/{}", id).as_str())
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 401 Unauthorized
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED, "{:?}", nice_response_error(resp));
    }

    // 403 - Token valido ma utente non autorizzato
    #[test]
    async fn delete_event_403() {

        // Ottenimento applicazione
        let app = create_app().await;

        let id = 0;
        test::call_service(&app, insert_event_request(id)).await;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri(format!("/delete_event/{}", id).as_str())
            .insert_header(("Authorization", format!("Bearer {}", UNAUTHORIZED_TOKEN)))
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

        let id = 0;

        // Crea una richiesta di test per la route "/delete_event"
        let req = test::TestRequest::delete()
            .uri(format!("/delete_event/{}", id).as_str()).insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 404 Not Found
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND, "{:?}", nice_response_error(resp));
    }

    // 418 - Evento non bloccato dall'utente che vuole fare modifiche
    #[test]
    async fn delete_event_418() {
        unimplemented!()
    }

    // 423 - Evento bloccato da un altro utente
    #[test]
    async fn delete_event_423() {
        unimplemented!()
    }


// --- Test modify_event ---
    // 200 - Test modify_event con evento esistente - modifica avvenuta con successo
    #[test]
    async fn modify_event_200() {

        // Ottenimento applicazione
        let app = create_app().await;

        let id = 0;
        test::call_service(&app, insert_event_request(id)).await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri(format!("/modify_event/{}", id).as_str())
            .insert_header((http::header::AUTHORIZATION, "Bearer ".to_string()+VALID_TOKEN))
                .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .set_json(&serde_json::json!({
                "id": id,
                "title": "Incontro G7",
                "summary": "Divieti di fermata e transito.",
                "description": "",
                "creator_id": 0,
                "risk_level": 50,
                "polygons": {
                    "type": "FeatureCollection",
                    "features": []
                },
                "validity": {
                    "begin": 1717670391,
                    "end": 1717670391
                },
                "visibility": {
                    "begin": null,
                    "end": 1717670391
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
                            "begin": 1717670391,
                            "end": 1717670391
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
                            "begin": 1717670391,
                            "end": 1717670391
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

        let id = 0;
        test::call_service(&app, insert_event_request(id)).await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri(format!("/modify_event/{}", id).as_str())
            .set_json(get_a_different_valid_event(id))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 401 Unauthorized
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED, "{:?}", nice_response_error(resp));
    }

    // 403 - Token valido ma utente non autorizzato
    #[test]
    async fn modify_event_403() {
        // Ottenimento applicazione
        let app = create_app().await;

        let id = 0;
        test::call_service(&app, insert_event_request(id)).await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri(format!("/modify_event/{}", id).as_str())
            .insert_header((http::header::AUTHORIZATION, "Bearer ".to_string()+UNAUTHORIZED_TOKEN))
            .set_json(get_valid_event(0))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 403
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN);
    }


    // 404 - Evento non esistente - nessun evento con ID uguale a quello che si vuole modificare
    #[test]
    async fn modify_event_404() {

        // Ottenimento applicazione
        let app = create_app().await;

        let id = 0;
        test::call_service(&app, insert_event_request(id)).await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri("/modify_event/9999")
            .insert_header((http::header::AUTHORIZATION, "Bearer ".to_string()+VALID_TOKEN))
            //.set_json("{\"id\":1,\"title\":\"Incontro G7\",\"summary\":\"Divieti di fermata e transito.\",\"description\":\"\",\"creator_id\":0,\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"},\"visibility\":{\"start\":null,\"end\":\"2024-03-15T19:00:00Z\"},\"categories\":[],\"subevents\":[{\"title\":\"Fase preparatoria\",\"description\":\"\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[]},\"validity\":{\"start\":\"2024-03-14T22:00:00Z\",\"end\":\"2024-03-15T19:00:00Z\"}},{\"title\":\"Arrivo delegazioni\",\"description\":\"Descrizione\",\"polygons\":{\"type\":\"FeatureCollection\",\"features\":[{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}},{\"type\":\"Feature\",\"geometry\":{\"type\":\"Polygon\",\"coordinates\":[[[11.12094551324845,46.06685718388318]]]}]}],\"validity\":{\"start\":\"1717670391\",\"end\":\"1717670391\"}}]}")
            .set_json(get_valid_event(0))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 404 Not Found
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND, "{}", nice_response_error(resp));
    }

    // 418 - Evento non bloccato dall'utente che vuole fare modifiche
    #[test]
    async fn modify_event_418() {
        unimplemented!()
    }

    // 422 - Modifiche non valide - le modifiche porterebbero ad avere un evento invalido nel sistema
    #[test]
    async fn modify_event_422() {

        // Ottenimento applicazione
        let app = create_app().await;
        
        let id = 0;
        test::call_service(&app, insert_event_request(id)).await;

        // Crea una richiesta di test per la route "/modify_event"
        let req = test::TestRequest::put()
            .uri(format!("/modify_event/{}", id).as_str())
            .insert_header(("Authorization", "Bearer ".to_string()+VALID_TOKEN))
            .set_json(get_a_different_valid_event(id+1))
            .to_request();

        // Esegui la richiesta
        let resp = test::call_service(&app, req).await;

        // Verifica che lo stato della risposta sia 422 Unprocessable Entity
        assert_eq!(resp.status(), http::StatusCode::UNPROCESSABLE_ENTITY, "{}", nice_response_error(resp));
    }

    // 423 - Evento bloccato da un altro utente
    #[test]
    async fn modify_event_423() {
        unimplemented!()
    }

}
