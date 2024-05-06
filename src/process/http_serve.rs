use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use crate::HttpServeOpts;
use anyhow::Result;
use axum::{extract::{Path, State}, http::StatusCode, routing::get, Router};
use tower_http::{services::ServeDir, validate_request::ValidateRequestHeaderLayer};
use tracing::{error, info};

#[derive(Debug, Clone)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http(opts: HttpServeOpts) -> Result<()> {
    let addr = SocketAddr::from(([0,0,0,0], opts.port));
    info!("Serve directory: {} on port: {}", opts.dir.display(), addr);
    let path = opts.dir.clone();
    //create a request state
    let state = HttpServeState {
        path
    };
    //create a dir_server
    let dir_server = ServeDir::new(&state.path)
    .append_index_html_on_directories(true)
    .precompressed_gzip()
    .precompressed_br()
    .precompressed_zstd();
    //create a axum router
    let app_router:Router = Router::new()
    .nest_service("/tower", dir_server)
        .route("/*path", get(file_handle))
        .with_state(Arc::new(state));
    // Add authentication middleware to the router
    let app_router = app_router.layer(ValidateRequestHeaderLayer::bearer("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    //start the server with the router and listener
    axum::serve(listener, app_router).await?;
    Ok(())
}

async fn file_handle(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>
) -> (StatusCode,String

) {
    let path = state.path.join(path);
    info!("enter handle with reading:{}",path.display());
    if !path.is_file() {
        //if path is directory, create a index.html file and return it
        if path.is_dir() {
            //这里获取path对应的directory下的所有文件，然后生成完整的html展示<li>文件名</li>的列表
            let mut html = String::new();
            html.push_str("<html><body><ul>");
            match tokio::fs::read_dir(&path).await {
                Ok(mut entries) => {
                    let mut html = String::new(); // 初始化html字符串
                    while let Some(entry) = entries.next_entry().await.expect("Failed to read next entry") {
                        let file_name = entry.file_name();
                        let file_name = file_name.to_string_lossy();
                        html.push_str(&format!("<li><a href=\"{}\">{}</a></li>", file_name, file_name));
                    }
                    (StatusCode::OK, html)
                }
                Err(e) => {
                    error!("read dir error:{}",e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "read dir error".to_string())
                }
            }
        } else {
            return (StatusCode::NOT_FOUND, "file not found".to_string());
        }
    } else {
        match tokio::fs::read(path).await {
            Ok(content) => {
                //try to convert the content to string，if failed, return the content as bytes
                match String::from_utf8(content.clone()) {
                    Ok(content) => (StatusCode::OK, content),
                    Err(_) => {
                        (StatusCode::OK, content.iter().map(|&c| c as char).collect::<String>())
                    }
                }
            }
            Err(e) => {
                error!("read file error:{}",e);
                (StatusCode::INTERNAL_SERVER_ERROR, "read file error".to_string())
            }
        }
    }
}
