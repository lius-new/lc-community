-- Add migration script here
-- 创建用户关系状态表
create table user_relation_status
(
    id         serial
        constraint user_relation_status_pk
            primary key,
    status     bool                           not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp
);
comment on table user_relation_status is '用户关系状态';
comment on column user_relation_status.status is '用户关系状态名称，如：建立好友关系时对方未同意(false)等';
