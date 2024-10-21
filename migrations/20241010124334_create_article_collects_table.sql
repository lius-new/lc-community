-- Add migration script here
-- 创建文章收藏表
create table article_collects
(
    id         serial
        constraint article_collects_pk
            primary key,
    uuid       varchar(64)                    not null,
    article_id int                            not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    foreign key (uuid) references users (uuid),
    foreign key (article_id) references articles (id),
    -- 表示uuid和article_id只能同时出现一次.(多次点赞)
    constraint unique_user_article_collect unique (uuid, article_id)
);

comment on table article_collects is '文章收藏表';
comment on column article_collects.uuid is '用户uuid';
comment on column article_collects.article_id is '文章id';
