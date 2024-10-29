-- Add migration script here
--创建文章分组 (每个文章必然属于一个或多个分组，一个分组必然包含多个文章)
create table article_groups
(
    id          serial
        constraint article_groups_pk
            primary key,
    name        varchar(24) unique             not null,
    description varchar(256)                   not null,
    visiable    boolean  default true          not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp
);
comment on table article_groups is '社区分组表';
comment on column article_groups.name is '分组名称';
comment on column article_groups.description is '分组描述';
