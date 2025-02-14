-- Add migration script here
-- 创建权限与用户资源关联表
create table permission_user_resource_relations
(
    id                 serial
        constraint permission_user_resource_relations_pk
            primary key,
    resource_id        int                            not null,
    user_permission_id int                            not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    foreign key (user_permission_id) references user_permissions (id),
    foreign key (resource_id) references user_resources (id)

);
comment on table permission_user_resource_relations is '权限与用户资源关联表(M:N)';
comment on column permission_user_resource_relations.resource_id is '资源对应的id';
comment on column permission_user_resource_relations.user_permission_id is '资源对应的权限id';
