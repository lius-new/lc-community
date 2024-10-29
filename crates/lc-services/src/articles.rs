use anyhow::Result;
use axum::extract::Multipart;
use lc_utils::database;

pub mod article_services {
    use std::path::Path;

    use anyhow::anyhow;
    use lc_utils::config::AppCon;
    use tokio::{fs::File, io::AsyncWriteExt};

    use super::*;

    pub async fn view_by_hash(hash: &str) -> Result<lc_models::articles::ArticleDetail> {
        let pool = database::get_connection().await?;

        let mut article: lc_models::articles::ArticleDetail = sqlx::query_as(
            "select title, description, content,created_at from articles where hash = $1",
        )
        .bind(hash)
        .fetch_one(pool)
        .await?;

        let covers: (Option<Vec<String>>,) = sqlx::query_as(
            "select path from article_covers left join articles a on a.id = article_covers.article_id where a.hash = $1;",
        )
        .bind(hash)
        .fetch_one(pool)
        .await?;

        article.covers = covers.0.map_or(Some(vec![]), |v| Some(v));

        Ok(article)
    }

    pub async fn create(
        mut multipart: Multipart,
        payload: lc_dto::articles::CreateArticleRequestParams,
    ) -> Result<()> {
        let pool = database::get_connection().await?;
        let mut tx = pool.begin().await?;

        let article_hash = lc_utils::sha256_digest(payload.content.as_bytes())?;
        let (article_id, ): (i32,) = sqlx::query_as("insert into articles(title, description, content, hash, created_at, updated_at, deleted_at) values ($1, $2, $3, $4) returning id;")
            .bind(payload.title)
            .bind(payload.description)
            .bind(payload.content)
            .bind(article_hash)
            .fetch_one(&mut *tx).await?;

        // 如果cover的根路径(保存cover的文件夹)不存在,就error
        let root_path = AppCon.upload.article_covers.as_str();
        if Path::new(root_path).exists() {
            return Err(anyhow!("abc"));
        }

        // 获取文件, 并将文件保存起来.
        if let Some(field) = multipart.next_field().await? {
            if let Some("file") = field.name() {
                let file_name = field
                    .file_name()
                    .ok_or(anyhow!("file name not provider"))?
                    .to_string();

                let cover_path = Path::new(root_path).join(&file_name);
                let cover_data = field.bytes().await?;
                let cover_hash = lc_utils::sha256_digest(&cover_data)?;

                // 判断文件是否存在, 即查询数据库判断是否有对应的hash文件和指定位置是否存在该文件。
                // 查询该文件是否保存在数据库
                let cover_exist_with_db =
                    sqlx::query("select id from article_covers where hash = $1;")
                        .bind(&cover_hash)
                        .execute(&mut *tx)
                        .await?
                        .rows_affected()
                        > 0;
                // 查询该文件是否保存在服务器
                let cover_exist_with_server = cover_path.exists();

                // 保存文件到服务器的函数，方便下面的多次调用功能
                let save = || async {
                    let mut file = File::create(&cover_path).await?;
                    file.write_all(&cover_data).await?;
                    Ok::<(), anyhow::Error>(())
                };

                // 数据库和服务器都没有那么完整操作： 插入数据库和保存到服务器
                if !cover_exist_with_db && !cover_exist_with_server {
                    sqlx::query(
                        "insert into article_covers(hash, path, article_id) values ($1, $2, $3);",
                    )
                    .bind(cover_path.to_str())
                    .bind(&cover_hash)
                    .bind(article_id)
                    .execute(&mut *tx)
                    .await?;
                    save().await?;
                }
                // 数据库有而服务器没有：保存到服务器
                if cover_exist_with_db && !cover_exist_with_server {
                    save().await?;
                }
                // 其他情况不考虑
            }
        }

        // tag 与 article关联表。
        for tag_id in payload.tags {
            sqlx::query("insert into article_tag_relations (article_id, tag_id) values ($1, $2);")
                .bind(article_id)
                .bind(tag_id)
                .execute(&mut *tx)
                .await?;
        }

        // group 与 article关联表。
        for group_id in payload.groups {
            sqlx::query(
                "insert into article_groups_relations (article_group_id, article_id) values ($1, $2);",
            )
            .bind(article_id)
            .bind(group_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn modify(payload: lc_dto::articles::ModifyArticleRequestParams) -> Result<()> {
        let pool = database::get_connection().await?;
        let mut tx = pool.begin().await?;

        let mut set_sql = Vec::new();
        if !payload.title.is_empty() {
            set_sql.push(format!("tilte = {}", payload.title));
        }
        if !payload.description.is_empty() {
            set_sql.push(format!("description = {}", payload.description));
        }
        if !payload.content.is_empty() {
            set_sql.push(format!("content = {}", payload.content));
            set_sql.push(lc_utils::sha256_digest(payload.content.as_bytes())?);
        }

        // 更新title, description, content, hash数据。
        let (article_id,): (i32,) =
            sqlx::query_as("update articles set $1 where hash = $2 returning id;")
                .bind(set_sql.join(","))
                .bind(payload.hash)
                .fetch_one(&mut *tx)
                .await?;

        // tag 与 article关联表。
        for tag_id in payload.tags {
            sqlx::query("insert into article_tag_relations (article_id, tag_id) values ($1, $2);")
                .bind(article_id)
                .bind(tag_id)
                .execute(&mut *tx)
                .await?;
        }

        // group 与 article关联表。
        for group_id in payload.groups {
            sqlx::query(
                "insert into article_groups_relations (article_group_id, article_id) values ($1, $2);",
            )
            .bind(article_id)
            .bind(group_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_by_hash(hash: &str) -> Result<()> {
        let pool = database::get_connection().await?;
        let mut tx = pool.begin().await?;

        let (article_id,): (i32,) = sqlx::query_as("select id from articles where hash = $1")
            .bind(hash)
            .fetch_one(&mut *tx)
            .await?;

        sqlx::query("delete from articles where id = $1;")
            .bind(article_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("delete from article_covers where article_id = $1;")
            .bind(article_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn toggle_visiable(hash: &str) -> Result<()> {
        let pool = database::get_connection().await?;
        let mut tx = pool.begin().await?;

        let (visiable,): (bool,) = sqlx::query_as("select visiable from articles where hash = '';")
            .bind(hash)
            .fetch_one(&mut *tx)
            .await?;

        sqlx::query("update articles set visiable = $1 where hash = $2;")
            .bind(!visiable)
            .bind(hash)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn view_by_page(
        page_size: i32,
        page_num: i32,
    ) -> Result<lc_models::articles::ArticleByPage> {
        let pool = database::get_connection().await?;

        let offset = page_num * page_size;
        let articles: Vec<lc_models::articles::Article> = sqlx::query_as(
            "select title, description from articles order by created_at,updated_at limit $1 offset $2;",
        )
            .bind(page_size)
            .bind(offset)
        .fetch_all(pool)
        .await?;

        let (total,): (i32,) = sqlx::query_as("select count(id) from articles;")
            .fetch_one(pool)
            .await?;

        Ok(lc_models::articles::ArticleByPage { articles, total })
    }

    pub async fn toplist(
        page_size: i32,
        page_num: i32,
    ) -> Result<lc_models::articles::ArticleByPage> {
        let pool = database::get_connection().await?;

        let offset = page_num * page_size;
        let articles: Vec<lc_models::articles::Article> = sqlx::query_as(
            "select title, description from articles order by created_at,updated_at limit $1 offset $2;",
        )
            .bind(page_size)
            .bind(offset)
        .fetch_all(pool)
        .await?;

        let (total,): (i32,) = sqlx::query_as("select count(id) from articles;")
            .fetch_one(pool)
            .await?;

        Ok(lc_models::articles::ArticleByPage { articles, total })
    }

    pub async fn random(
        page_size: i32,
        page_num: i32,
    ) -> Result<lc_models::articles::ArticleByPage> {
        let pool = database::get_connection().await?;

        let offset = page_num * page_size;
        let articles: Vec<lc_models::articles::Article> = sqlx::query_as(
            "select title, description from articles order by created_at,updated_at limit $1 offset $2;",
        )
            .bind(page_size)
            .bind(offset)
        .fetch_all(pool)
        .await?;

        let (total,): (i32,) = sqlx::query_as("select count(id) from articles;")
            .fetch_one(pool)
            .await?;

        Ok(lc_models::articles::ArticleByPage { articles, total })
    }
}

pub mod article_groups_services {
    use super::*;
    use lc_dto::articles::article_groups::CreateArticleGroupRequestParams;
    use lc_models::articles::article_groups::{ArticleGroup, ArticleGroupByPage};

    pub async fn create(payload: CreateArticleGroupRequestParams) -> Result<()> {
        let pool = database::get_connection().await?;

        sqlx::query("insert into article_groups (name, description) values ($1, $2);")
            .bind(payload.name)
            .bind(payload.description)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn modify(payload: CreateArticleGroupRequestParams) -> Result<()> {
        let pool = database::get_connection().await?;

        sqlx::query("update article_groups set name = $1, description = $2 where id = $3;")
            .bind(payload.name)
            .bind(payload.description)
            .bind(payload.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// 删除文章组, 假删除.
    pub async fn delete(id: i32) -> Result<()> {
        let pool = database::get_connection().await?;

        sqlx::query("update article_groups set deleted_at = now() where id = $1;")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// 根据文章组名称来查询该文章组的信息.
    pub async fn view(id: i32) -> Result<ArticleGroup> {
        let pool = database::get_connection().await?;

        let article_group: ArticleGroup =
            sqlx::query_as("select name, description, visiable from article_groups where id = $1;")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(article_group)
    }

    /// 查询文章组分页。
    pub async fn view_by_page(page_size: i32, page_num: i32) -> Result<ArticleGroupByPage> {
        let pool = database::get_connection().await?;

        let offset = page_num * page_size;
        let article_groups: Vec<ArticleGroup> = sqlx::query_as(
            "select name,description,visiable from article_groups limit $1 offset $2;",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        let (total,): (i32,) = sqlx::query_as("select count(id) from article_groups;")
            .fetch_one(pool)
            .await?;

        Ok(ArticleGroupByPage {
            article_groups,
            total,
        })
    }

    pub async fn toggle_visiable(id: i32) -> Result<()> {
        let pool = database::get_connection().await?;

        let mut tx = pool.begin().await?;

        let (visiable,): (bool,) =
            sqlx::query_as("select visiable from article_groups where id = $1;")
                .bind(id)
                .fetch_one(&mut *tx)
                .await?;

        sqlx::query("update article_groups set visiable = $1 wher id = $2;")
            .bind(visiable)
            .bind(id)
            .execute(pool)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}

pub mod article_tags_services {
    use super::*;
    use lc_dto::articles::article_tags::CreateArticleTagRequestParams;
    use lc_models::articles::article_tags::{ArticleTag, ArticleTagByPage};

    pub async fn create(payload: CreateArticleTagRequestParams) -> Result<()> {
        let pool = database::get_connection().await?;

        sqlx::query("insert into article_groups (name, description) values ($1, $2);")
            .bind(payload.name)
            .bind(payload.description)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn modify(payload: CreateArticleTagRequestParams) -> Result<()> {
        let pool = database::get_connection().await?;

        sqlx::query("update article_tags set name = $1, description = $2 where id = $3;")
            .bind(payload.name)
            .bind(payload.description)
            .bind(payload.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(id: i32) -> Result<()> {
        let pool = database::get_connection().await?;

        sqlx::query("update article_tags set deleted_at = now() where id = $1;")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn view(id: i32) -> Result<ArticleTag> {
        let pool = database::get_connection().await?;

        let article_tag: ArticleTag =
            sqlx::query_as("select name, description, visiable from article_tags where id = $1;")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(article_tag)
    }

    pub async fn view_by_page(page_size: i32, page_num: i32) -> Result<ArticleTagByPage> {
        let pool = database::get_connection().await?;

        let offset = page_num * page_size;
        let article_tags: Vec<ArticleTag> = sqlx::query_as(
            "select name,description,visiable from article_tags limit $1 offset $2;",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        let (total,): (i32,) = sqlx::query_as("select count(id) from article_tags;")
            .fetch_one(pool)
            .await?;

        Ok(ArticleTagByPage {
            article_tags,
            total,
        })
    }

    pub async fn toggle_visiable(id: i32) -> Result<()> {
        let pool = database::get_connection().await?;

        let mut tx = pool.begin().await?;

        let (visiable,): (bool,) =
            sqlx::query_as("select visiable from article_groups where id = $1;")
                .bind(id)
                .fetch_one(&mut *tx)
                .await?;

        sqlx::query("update article_tags set visiable = $1 where id = $1;")
            .bind(visiable)
            .bind(id)
            .execute(pool)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}
