pub mod user {
    pub mod auth {
        use axum::routing::{get, MethodRouter};

        pub fn route() -> MethodRouter {
            get(handle_get()).post(handle_post())
        }

        fn handle_get() -> String {
            "GET".to_string()
        }

        fn handle_post() -> String {
            "POST".to_string()
        }
    }
}
