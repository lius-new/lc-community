-- Add migration script here
-- 创建用户表
create table users
(
    id         serial
        constraint users_pk
            primary key,
    uuid       varchar(64) unique             not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp 
);
comment on table users is '用户表';
comment on column users.id is '用户id';
comment on column users.uuid is '用户uuid';
