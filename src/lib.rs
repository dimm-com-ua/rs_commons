pub mod domains;

#[cfg(test)]
pub mod tests {
    use crate::domains::http_entities::api_response::ApiResponse;

    #[test]
    fn response_code() {
        let resp_400 = ApiResponse::app_api_some_error("400");
        assert_eq!(resp_400.status, 400)
    }
}