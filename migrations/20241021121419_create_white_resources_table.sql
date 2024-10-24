-- Add migration script here

-- 资源白名单
create table white_resources
(
    id          serial
        constraint white_resources_pk
            primary key,
    name        varchar(72)                    not null,
    description varchar(48)                    not null,
    resource    varchar(72)                    not null,
    method    varchar(12)                    not null,
    can_use     bool default true,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp
);
comment on table white_resources is '资源白名单表';
comment on column white_resources.name is '资源名称';
comment on column white_resources.description is '资源描述';
comment on column white_resources.resource is '资源内容: (http example: POST+/users/login)';
comment on column white_resources.can_use is '资源是否可用(开放)';
COMMENT ON COLUMN white_resources.method IS '资源的请求方法';
