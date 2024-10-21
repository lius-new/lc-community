-- Add migration script here
-- 创建用户权限关联表
create table user_permissions
(
    id                   serial
        constraint user_permissions_pk
            primary key,
    name                 varchar(52)                    not null,
    description          varchar(256)                   not null,
    parent_permission_id int                            ,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    foreign key (parent_permission_id) references user_permissions (id),
    constraint id_parent_group_id_different check ( user_permissions.id != user_permissions.parent_permission_id )
);
comment on table user_permissions is '用户权限表';
comment on column user_permissions.name is '权限名称';
comment on column user_permissions.description is '权限描述';
comment on column user_permissions.parent_permission_id is '权限父级id';
