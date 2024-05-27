use actix_web::{test, App, web};

#[cfg(test)]
mod tests {
    use super::*;
    
    // Import librerie get_events e get_event
    use crate::{get_events, get_event};

    #[actix_web::test]
    async fn test_get_events() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/api/v3/events").route(web::get().to(get_events)))
        )
            .await;
    
        // Tests
    }
    
    #[actix_web::test]
    async fn test_get_event() {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/api/v3/event/{id}").route(web::get().to(get_event)))
        )
            .await;
    
        // Tests
    }
}
