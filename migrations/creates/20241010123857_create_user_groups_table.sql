-- Add migration script here
-- 创建用户分组表
create table user_groups
(
    id              serial
        constraint user_groups_pk
            primary key,
    name            varchar(24)                    not null,
    description     varchar(256)                   not null,
    parent_group_id int                            not null,
    created_at      time default current_timestamp not null,
    updated_at      time,
    deleted_at      time,
    foreign key (parent_group_id) references user_groups (id),
    constraint id_parent_group_id_different check ( user_groups.id != user_groups.parent_group_id )
);
comment on table user_groups is '用户分组表';
comment on column user_groups.id is '用户分组id';
comment on column user_groups.name is '用户分组名称';
comment on column user_groups.description is '用户分组描述';
comment on column user_groups.parent_group_id is '用户分组的父级分组id';