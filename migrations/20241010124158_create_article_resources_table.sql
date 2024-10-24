-- Add migration script here
-- 创建文章资源表
create table article_resources
(
    id          serial
        constraint article_resources_pk
            primary key,
    name        varchar(72)                    not null,
    description varchar(48)                    not null,
    resource    varchar(72)                    not null,
    method      varchar(12)                    not null,
    can_use     bool default true,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp
);
comment on table article_resources is '文章资源表';
comment on column article_resources.name is '文章资源名称';
comment on column article_resources.description is '文章资源描述';
comment on column article_resources.resource is '文章资源内容: (http example: POST+/users/login)';
comment on column article_resources.can_use is '接口是否可用(开放)';
