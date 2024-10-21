-- Add migration script here
-- 创建用户间沟通表(不包含沟通内容)
create table user_communicates
(
    id           serial
        constraint user_communicates_pk
            primary key,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    uuid         varchar(64)                    not null,
    current_uuid varchar(64)                    not null,
    foreign key (uuid) references users (uuid),
    foreign key (current_uuid) references users (uuid),
    constraint check_uuid_different check ( uuid != current_uuid )
);
comment on table user_communicates is '用户沟通表';
comment on column user_communicates.uuid is '用户uuid';
comment on column user_communicates.current_uuid is '对方用户的uuid';
