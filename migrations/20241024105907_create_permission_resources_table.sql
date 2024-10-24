-- Add migration script here
-- 创建权限资源表
create table permission_resources
(
    id          serial
        constraint permission_resources_pk
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
comment on table article_resources is '权限资源表';
comment on column article_resources.name is '权限资源名称';
comment on column article_resources.description is '权限资源描述';
comment on column article_resources.resource is '权限资源内容: (http example: POST+/users/login)';
comment on column article_resources.can_use is '接口是否可用(开放)';

-- 创建权限与权限资源关联表
create table permission_permission_resource_relations
(
    id                 serial
        constraint permission_permission_resource_relations_pk
            primary key,
    resource_id        int                            not null,
    user_permission_id int                            not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    foreign key (user_permission_id) references user_permissions (id),
    foreign key (resource_id) references permission_resources (id)

);
comment on table permission_user_resource_relations is '权限与权限资源关联表(M:N)';
comment on column permission_user_resource_relations.resource_id is '资源对应的id';
comment on column permission_user_resource_relations.user_permission_id is '资源对应的权限id';
