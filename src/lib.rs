//TODO include needed modules

pub async fn server(routes) {
    // TODO connect to db 
    
    // TODO get the ip from ENV 
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
