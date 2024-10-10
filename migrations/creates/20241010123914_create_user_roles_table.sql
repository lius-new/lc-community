-- Add migration script here
-- 创建角色表
create table user_roles
(
    id             serial
        constraint user_roles_pk
            primary key,
    name           varchar(24)                    not null,
    description    varchar(256)                   not null,
    parent_role_id int                            not null,
    created_at     time default current_timestamp not null,
    updated_at     time,
    deleted_at     time,
    foreign key (parent_role_id) references user_roles (id),
    constraint id_parent_group_id_different check ( user_roles.id != user_roles.parent_role_id )
);
comment on table user_roles is '用户角色表';
comment on column user_roles.name is '角色名称';
comment on column user_roles.description is '角色描述';
comment on column user_roles.parent_role_id is '角色父级id'