-- Add migration script here
-- 创建用户关系类型表(好友关系，关注关系等。。。)
create table user_relation_types
(
    id          serial
        constraint user_relation_types_pk
            primary key,
    name        varchar(64)                    not null,
    description varchar(128)                   not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp
);
comment on table user_relation_types is '用户关系表';
comment on column user_relation_types.name is '用户关系名称, 如："好友，关注，拉黑等"';
comment on column user_relation_types.description is '用户关系描述"';
