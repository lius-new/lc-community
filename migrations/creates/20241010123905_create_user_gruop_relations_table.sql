-- Add migration script here
-- 创建用户与用户分组关联表
create table user_group_relations
(
    id            serial
        constraint user_group_relations_pk
            primary key,
    uuid          varchar(64)                    not null,
    user_group_id int                            not null,
    created_at    time default current_timestamp not null,
    updated_at    time,
    deleted_at    time,
    foreign key (uuid) references users (uuid),
    foreign key (user_group_id) references user_groups (id)
);
comment on table user_group_relations is '用户与用户分组关联表';
comment on column user_group_relations.uuid is '用户uuid';
comment on column user_group_relations.user_group_id is '用户分组id';