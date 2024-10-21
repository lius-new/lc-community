-- Add migration script here
-- 创建用户资源表(保存用户相关接口资源)
create table user_resources
(
    id          serial
        constraint user_resources_pk
            primary key,
    name        varchar(36)                    not null,
    description varchar(48)                    not null,
    resource    varchar(72)                    not null,
    can_use     bool default true,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp
);
comment on table user_resources is '用户资源表';
comment on column user_resources.name is '用户资源名称';
comment on column user_resources.description is '用户资源描述';
comment on column user_resources.resource is '用户资源内容: (http example: POST+/users/login)';
comment on column user_resources.can_use is '接口是否可用(开放)';
