use rocket::{
    fairing::{Fairing, Info, Kind},
    fs::FileServer,
    http::Header,
    launch, routes,
    serde::json::{json, Json, Value},
};
use vps_back::ApiResponse;

/// # CORS Configuration
/// Implements CORS (Cross-Origin Resource Sharing) headers for the application.
/// Allows requests from localhost:5173 during development.
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    /// # `on_response`
    /// Sets CORS headers for the response.
    ///
    /// Sets the following headers:
    /// - Access-Control-Allow-Origin: http://localhost:3000 # port of the SvelteKit app
    /// - Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
    /// - Access-Control-Allow-Headers: Content-Type
    /// - Access-Control-Allow-Credentials: true
    async fn on_response<'r>(
        &self,
        _request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:3000",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// # `root`
/// Handles GET requests to the root path ("/").
/// Serves as a simple health check endpoint.
///
/// ## Returns
/// A static string greeting message
#[rocket::get("/")]
fn root() -> Json<Value> {
    ApiResponse::success(json!({
        "message": "Hello, Rocket!"
    }))
}

/// # `rocket`
/// Configures and launches the Rocket application.
/// Sets up database connection, runs migrations, configures CORS, and mounts routes.
///
/// ## Returns
/// The configured Rocket instance
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![root])
        .mount("/static", FileServer::from("static"))
}
