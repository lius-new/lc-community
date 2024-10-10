-- Add migration script here
-- 创建用户权限关联表
create table user_permissions
(
    id                   serial
        constraint user_permissions_pk
            primary key,
    name                 varchar(24)                    not null,
    description          varchar(256)                   not null,
    parent_permission_id int                            not null,
    created_at           time default current_timestamp not null,
    updated_at           time,
    deleted_at           time,
    foreign key (parent_permission_id) references user_permissions (id),
    constraint id_parent_group_id_different check ( user_permissions.id != user_permissions.parent_permission_id )
);
comment on table user_permissions is '用户权限表';
comment on column user_permissions.name is '权限名称';
comment on column user_permissions.description is '权限描述';
comment on column user_permissions.parent_permission_id is '权限父级id';