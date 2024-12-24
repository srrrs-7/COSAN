
async fn get_request_body(req: Request<Body>) -> Result<String, StatusCode> {
    let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();
    Ok(body)
}

async fn get_request_params(req: Request<Body>) -> Result<HashMap<String, String>, StatusCode> {
    let params = req.uri().query().unwrap();
    let params = form_urlencoded::parse(params.as_bytes()).into_owned().collect();
    Ok(params)
}