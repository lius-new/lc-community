#[cfg(test)]
mod test_article_services {

    #[tokio::test]
    async fn test_view_by_hash() {
        let res = lc_services::articles::article_services::view_by_hash("")
            .await
            .unwrap();

        println!("{:?}", res)
    }

    //#[tokio::test]
    //async fn test_create() {
    //let mut payload = lc_dto::articles::CreateArticleRequestParams {
    //    title: String::new("test"),
    //    description: String::new("this is test article description"),
    //    content: String::new("this is test article content"),
    //    tags: Vec::new(),
    //    groups: Vec::new(),
    //};
    //
    //let res = lc_services::articles::article_services::create("", payload)
    //    .await
    //    .unwrap();
    //
    //println!("{:?}", res)
    //}
}

#[cfg(test)]
mod test_article_groups_services {
    use crate::commons;
    use lc_dto::articles::article_groups::{
        CreateArticleGroupRequestParams, ModifyArticleGroupRequestParams,
    };

    #[tokio::test]
    async fn test_create() {
        commons::setup().await;

        let payload = CreateArticleGroupRequestParams {
            name: "abcd".to_string(),
            description: "abc".to_string(),
        };
        lc_services::articles::article_groups_services::create(payload)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_modify() {
        commons::setup().await;

        let payload = ModifyArticleGroupRequestParams {
            id: 1,
            name: "ad".to_string(),
            description: "abc".to_string(),
        };

        lc_services::articles::article_groups_services::modify(payload)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod test_article_tags_services {
    use crate::commons;

    use lc_dto::articles::article_tags::{
        CreateArticleTagRequestParams, ModifyArticleTagRequestParams,
    };

    #[tokio::test]
    async fn test_create() {
        commons::setup().await;

        let payload = CreateArticleTagRequestParams {
            name: "abcd".to_string(),
            description: "abc".to_string(),
        };

        lc_services::articles::article_tags_services::create(payload)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_modify() {
        commons::setup().await;

        let payload = ModifyArticleTagRequestParams {
            id: 1,
            name: "abc".to_string(),
            description: "abc".to_string(),
        };

        lc_services::articles::article_tags_services::modify(payload)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_delete() {
        commons::setup().await;

        lc_services::articles::article_tags_services::delete(1)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_view() {
        commons::setup().await;

        let tag = lc_services::articles::article_tags_services::view(1)
            .await
            .unwrap();

        println!("{:?}", tag);
    }

    #[tokio::test]
    async fn test_view_by_page() {
        commons::setup().await;

        let tags = lc_services::articles::article_tags_services::view_by_page(10, 1)
            .await
            .unwrap();

        println!("{:?}", tags);
    }

    #[tokio::test]
    async fn test_toggle_visiable() {
        commons::setup().await;

        lc_services::articles::article_tags_services::toggle_visiable(1)
            .await
            .unwrap();
    }
}
