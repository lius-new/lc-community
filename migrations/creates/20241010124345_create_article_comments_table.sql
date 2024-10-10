-- Add migration script here
-- 创建文章评论表
create table article_comments
(
    id                serial
        constraint article_comments_pk
            primary key,
    article_id        int                            not null,
    parent_comment_id int,
    content           varchar(256)                   not null,
    created_at        time default current_timestamp not null,
    updated_at        time,
    deleted_at        time,
    foreign key (parent_comment_id) references article_comments (id),
    foreign key (article_id) references articles (id)
);
comment on table article_comments is '文章评论表';
comment on column article_comments.article_id is '文章id';
comment on column article_comments.parent_comment_id is '文章评论id, 如果为空则表示是属于文章评论, 如果不为空则表示该评论属于某条评论';
comment on column article_comments.content is '评论内容';