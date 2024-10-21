-- Add migration script here
-- 创建博客标签表
create table article_tags
(
    id          serial
        constraint article_tags_pk
            primary key,
    name        varchar(24)                    not null,
    description varchar(256)                   not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp 
);
comment on table article_tags is '文章标签表';
comment on column article_tags.name is '文章标签名称';
comment on column article_tags.description is '文章标签描述';
