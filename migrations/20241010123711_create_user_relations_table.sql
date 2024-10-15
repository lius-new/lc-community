-- Add migration script here
-- 创建用户关系表
create table user_relations
(
    id                      serial
        constraint user_relations_pk
            primary key,
    created_at              time default current_timestamp not null,
    updated_at              time,
    deleted_at              time,

    uuid                    varchar(64)                    not null,
    current_uuid            varchar(64)                    not null,
    user_relation_type_id   int                            not null,
    user_relation_status_id int                            not null,
    foreign key (uuid) references users (uuid),
    foreign key (current_uuid) references users (uuid),
    foreign key (user_relation_type_id) references user_relation_types (id),
    foreign key (user_relation_status_id) references user_relation_status (id),
    constraint check_uuid_different check ( uuid != current_uuid )
);
comment on table user_relations is '用户关系描述表(包含对用户关系的描述)';
comment on column user_relations.uuid is '用户uuid';
comment on column user_relations.current_uuid is '对方用户的uuid, 比如好友就是好友的uuid, 比如关注就是关注用户的uuid';
comment on column user_relations.user_relation_type_id is '用户关系类型的id';
comment on column user_relations.user_relation_status_id is '用户关系状态的id';