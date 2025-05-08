use crate::routes::prelude::*;

pub async fn handler(State(_state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    println!("GET /");
    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct Tmpl {}

    let template = Tmpl {};
    Ok(Html(template.render()?))
}
