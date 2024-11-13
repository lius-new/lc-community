#[cfg(test)]
mod test_user_services {
    use lc_services::users::reset_password;

    use crate::commons;

    #[tokio::test]
    async fn test_reset_password() {
        commons::setup().await;

        reset_password("4855d414-9376-4d7e-9dc4-0803416c23e3", "lsmima")
            .await
            .unwrap();
    }
}
