-- Add migration script here
--创建文章分组 (每个文章必然属于一个或多个分组，一个分组必然包含多个文章)
create table article_groups
(
    id          serial
        constraint article_groups_pk
            primary key,
    name        varchar(24)                    not null,
    description varchar(256)                   not null,
    created_at  time default current_timestamp not null,
    updated_at  time,
    deleted_at  time
);
comment on table article_groups is '社区分组表';
comment on column article_groups.name is '分组名称';
comment on column article_groups.description is '分组描述';