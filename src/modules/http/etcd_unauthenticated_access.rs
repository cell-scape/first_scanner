#[async_trait]
impl HttpModule for EtcdUnauthenticatedAccess {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/version", &endpoint);
        let res = http_client.get(&url).send().await?;
        if !res.status().is_success() {
            return Ok(None);
        }
        let body = res.text().await?;
        if body.contains(r#""etcdserver""#) &&
        body.contains(r#""etcdcluster""#) &&
        body.chars().count() < 200 {
            return Ok(Some(HttpFinding::EtcdUnauthenticatedAccess(url)));
        }
        Ok(None)
    }
}