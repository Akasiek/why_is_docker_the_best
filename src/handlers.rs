use axum::response::Html;
use crate::render_template;
use crate::templates::{IndexTemplate, ServerErrorTemplate};
use askama::Template;

pub async fn index() -> Html<String> {
    let template = IndexTemplate {};
    render_template!(template)
}

pub async fn server_error() -> Html<String> {
    let template = ServerErrorTemplate {};

    match template.render() {
        Ok(html) => Html(html),
        Err(_) => {
            Html("<h1>Internal Server Error</h1><p>Unable to render error page.</p>".to_string())
        }
    }
}
