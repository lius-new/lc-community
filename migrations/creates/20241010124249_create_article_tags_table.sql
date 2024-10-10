-- Add migration script here
-- 创建博客标签表
create table article_tags
(
    id          serial
        constraint article_tags_pk
            primary key,
    name        varchar(24)                    not null,
    description varchar(256)                   not null,
    created_at  time default current_timestamp not null,
    updated_at  time,
    deleted_at  time
);
comment on table article_tags is '文章标签表';
comment on column article_tags.name is '文章标签名称';
comment on column article_tags.description is '文章标签描述';