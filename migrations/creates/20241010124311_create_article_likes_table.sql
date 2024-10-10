-- Add migration script here
-- 创建文章点赞表
create table article_likes
(
    id         serial
        constraint article_likes_pk
            primary key,
    uuid       varchar(64)                    not null,
    article_id int                            not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
    foreign key (uuid) references users (uuid),
    foreign key (article_id) references articles (id),
    -- 表示uuid和article_id只能同时出现一次.(多次点赞)
    constraint unique_user_article_like unique (uuid, article_id)
);

comment on table article_likes is '文章点赞表';
comment on column article_likes.uuid is '用户uuid';
comment on column article_likes.article_id is '文章id';