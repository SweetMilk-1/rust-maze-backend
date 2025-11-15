#[cfg(test)]
mod handler_tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::{json, Value};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tower::util::ServiceExt; // for `oneshot` - исправлен импорт
    use uuid::Uuid;

    use crate::api::{create_api_router, MapStore};

    #[tokio::test]
    async fn test_create_map_success() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store);

        let request_body = json!({ "map_string": "# #\n # " });

        let request = Request::builder()
            .method("POST")
            .uri("/v1/maps")
            .header("content-type", "application/json")
            .body(Body::from(request_body.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"]["id"].is_string());
        assert_eq!(body["data"]["map_string"], "# #\n # ");
    }

    #[tokio::test]
    async fn test_create_map_invalid_format() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store);

        let request_body = json!({ "map_string": "invalid map" });

        let request = Request::builder()
            .method("POST")
            .uri("/v1/maps")
            .header("content-type", "application/json")
            .body(Body::from(request_body.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], "Invalid map format");
    }

    #[tokio::test]
    async fn test_list_maps_empty() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store);

        let request = Request::builder()
            .method("GET")
            .uri("/v1/maps")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_array());
        assert_eq!(body["data"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_list_maps_with_data() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store.clone());

        // First create a map
        let create_request_body = json!({ "map_string": "# #\n # " });
        let create_request = Request::builder()
            .method("POST")
            .uri("/v1/maps")
            .header("content-type", "application/json")
            .body(Body::from(create_request_body.to_string()))
            .unwrap();

        let create_response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(create_response.status(), StatusCode::OK);

        // Then list maps
        let list_request = Request::builder()
            .method("GET")
            .uri("/v1/maps")
            .body(Body::empty())
            .unwrap();

        let list_response = app.oneshot(list_request).await.unwrap();

        assert_eq!(list_response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(list_response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_array());
        let maps = body["data"].as_array().unwrap();
        assert_eq!(maps.len(), 1);
        assert_eq!(maps[0]["map_string"], "# #\n # ");
    }

    #[tokio::test]
    async fn test_get_map_success() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store.clone());

        // First create a map
        let create_request_body = json!({ "map_string": "# #\n # " });
        let create_request = Request::builder()
            .method("POST")
            .uri("/v1/maps")
            .header("content-type", "application/json")
            .body(Body::from(create_request_body.to_string()))
            .unwrap();

        let create_response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(create_response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();
        let map_id = body["data"]["id"].as_str().unwrap();

        // Then get the map by ID
        let get_request = Request::builder()
            .method("GET")
            .uri(format!("/v1/maps/{}", map_id))
            .body(Body::empty())
            .unwrap();

        let get_response = app.oneshot(get_request).await.unwrap();

        assert_eq!(get_response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(get_response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["data"]["id"], map_id);
        assert_eq!(body["data"]["map_string"], "# #\n # ");
    }

    #[tokio::test]
    async fn test_get_map_not_found() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store);

        let non_existent_id = Uuid::new_v4();

        let request = Request::builder()
            .method("GET")
            .uri(format!("/v1/maps/{}", non_existent_id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], "Map not found");
    }

    #[tokio::test]
    async fn test_delete_map_success() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store.clone());

        // First create a map
        let create_request_body = json!({ "map_string": "# #\n # " });
        let create_request = Request::builder()
            .method("POST")
            .uri("/v1/maps")
            .header("content-type", "application/json")
            .body(Body::from(create_request_body.to_string()))
            .unwrap();

        let create_response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(create_response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();
        let map_id = body["data"]["id"].as_str().unwrap();

        // Then delete the map
        let delete_request = Request::builder()
            .method("DELETE")
            .uri(format!("/v1/maps/{}", map_id))
            .body(Body::empty())
            .unwrap();

        let delete_response = app.clone().oneshot(delete_request).await.unwrap();

        assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

        // Verify the map is gone by trying to get it
        let get_request = Request::builder()
            .method("GET")
            .uri(format!("/v1/maps/{}", map_id))
            .body(Body::empty())
            .unwrap();

        let get_response = app.oneshot(get_request).await.unwrap();

        assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_map_not_found() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store);

        let non_existent_id = Uuid::new_v4();

        let request = Request::builder()
            .method("DELETE")
            .uri(format!("/v1/maps/{}", non_existent_id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], "Map not found");
    }

    #[tokio::test]
    async fn test_solve_map_success() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store.clone());

        // First create a map
        let create_request_body = json!({ "map_string": "     \n     \n     " }); // 5x3 empty map
        let create_request = Request::builder()
            .method("POST")
            .uri("/v1/maps")
            .header("content-type", "application/json")
            .body(Body::from(create_request_body.to_string()))
            .unwrap();

        let create_response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(create_response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();
        let map_id = body["data"]["id"].as_str().unwrap();

        // Then solve the map
        let solve_request = Request::builder()
            .method("GET")
            .uri(format!("/v1/maps/{}/solve?start_x=0&start_y=0&finish_x=2&finish_y=2", map_id))
            .body(Body::empty())
            .unwrap();

        let solve_response = app.oneshot(solve_request).await.unwrap();

        assert_eq!(solve_response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(solve_response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"]["path_found"].as_bool().unwrap());
        assert!(body["data"]["solution_map"].is_string());
    }
    
    #[tokio::test]
    async fn test_solve_map_not_found() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store);

        let non_existent_id = Uuid::new_v4();

        let request = Request::builder()
            .method("GET")
            .uri(format!("/v1/maps/{}/solve?start_x=0&start_y=0&finish_x=1&finish_y=1", non_existent_id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], "Map not found");
    }

    #[tokio::test]
    async fn test_solve_map_invalid_start_position() {
        let map_store: MapStore = Arc::new(RwLock::new(HashMap::new()));
        let app = create_api_router(map_store.clone());

        // First create a map with walls
        let create_request_body = json!({ "map_string": "# # #\n# # #\n# # #" });
        let create_request = Request::builder()
            .method("POST")
            .uri("/v1/maps")
            .header("content-type", "application/json")
            .body(Body::from(create_request_body.to_string()))
            .unwrap();

        let create_response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(create_response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(create_response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();
        let map_id = body["data"]["id"].as_str().unwrap();

        // Try to solve with invalid start position (on wall)
        let solve_request = Request::builder()
            .method("GET")
            .uri(format!("/v1/maps/{}/solve?start_x=0&start_y=0&finish_x=2&finish_y=2", map_id))
            .body(Body::empty())
            .unwrap();

        let solve_response = app.oneshot(solve_request).await.unwrap();

        assert_eq!(solve_response.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(solve_response.into_body(), usize::MAX).await.unwrap(); // исправлен импорт
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], "Start position  is invalid");
    }
}