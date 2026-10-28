use axum::extract::State;

use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use crud_rust_usuarios::db::Database;
use crud_rust_usuarios::usuario::{CriarUsuario, Usuario};
use tokio::net::TcpListener;

async fn criar_usuario(
    State(db): State<Database>,
    Json(payload): Json<CriarUsuario>,
) -> Result<(StatusCode, Json<Usuario>), (StatusCode, String)> {
    let usuario = Usuario::new(payload.nome, payload.email)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e))?;

    db.inserir(usuario.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((StatusCode::CREATED, Json(usuario)))
}

#[axum::debug_handler]
async fn listar_usuarios(
    State(db): State<Database>,
) -> Result<(StatusCode, Json<Vec<Usuario>>), (StatusCode, String)> {
    let usuarios = db
        .listar()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok((StatusCode::OK, Json(usuarios)))
}

#[tokio::main]
async fn main() {
    let db = Database::new();

    let app = Router::new()
        .route("/usuario", post(criar_usuario))
        .route("/usuario", get(listar_usuarios))
        .with_state(db);

    let listener = match TcpListener::bind("127.0.0.1:3000").await {
        Ok(l) => {
            println!("Servidor no ar, maluco!");
            l
        }
        Err(e) => {
            println!("Falha ao subir servidor {}", e);
            return;
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
