-- Add migration script here
-- 创建用户角色分组关联表
create table user_role_group_relations
(
    id            serial
        constraint user_role_group_relations_pk
            primary key,
    user_group_id int                            not null,
    user_role_id  int                            not null,
    created_at    time default current_timestamp not null,
    updated_at    time,
    deleted_at    time,
    foreign key (user_group_id) references user_groups (id),
    foreign key (user_role_id) references user_roles (id)
);
comment on table user_role_group_relations is '用户角色分组关联表';
comment on column user_role_group_relations.user_group_id is '用户分组id';
comment on column user_role_group_relations.user_role_id is '用户角色id';