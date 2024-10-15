-- Add migration script here
-- 创建博客文章表
create table articles
(
    id          serial
        constraint articles_pk
            primary key,
    title       varchar(64)                    not null,
    description varchar(256)                   not null,
    content     varchar                        not null,
    hash        varchar(128)                   not null,
    created_at  time default current_timestamp not null,
    updated_at  time,
    deleted_at  time
);
comment on table articles is '文章表';
comment on column articles.title is '文章标题';
comment on column articles.description is '文章描述';
comment on column articles.content is '文章内容';
comment on column articles.hash is '文章hash';