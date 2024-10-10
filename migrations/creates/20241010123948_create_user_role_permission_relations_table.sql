-- Add migration script here
-- 创建用户角色权限关联表
create table user_role_permission_relations
(
    id                 serial
        constraint user_role_permission_relations_pk
            primary key,
    user_role_id       int                            not null,
    user_permission_id int                            not null,
    created_at         time default current_timestamp not null,
    updated_at         time,
    deleted_at         time,
    foreign key (user_role_id) references user_roles (id),
    foreign key (user_permission_id) references user_permissions (id)
);
comment on table user_role_permission_relations is '用户角色权限关联表';
comment on column user_role_permission_relations.user_role_id is '用户角色id';
comment on column user_role_permission_relations.user_permission_id is '用户权限id';