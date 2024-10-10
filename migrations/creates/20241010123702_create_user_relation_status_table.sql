-- Add migration script here
-- 创建用户关系状态表
create table user_relation_status
(
    id         serial
        constraint user_relation_status_pk
            primary key,
    status     bool                           not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time
);
comment on table user_relation_status is '用户关系状态';
comment on column user_relation_status.status is '用户关系状态名称，如：建立好友关系时对方未同意(false)等';