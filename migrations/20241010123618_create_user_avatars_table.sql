-- Add migration script here
-- 创建用户头像表
create table user_avatars
(
    id         serial
        constraint user_avatars_pk
            primary key,
    hash       varchar(128)                   not null,
    path       varchar(256)                   not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
    uuid       varchar(64) unique             not null,
    foreign key (uuid) references users (uuid)
);
comment on table user_avatars is '用户头像表';
comment on column user_avatars.hash is '头像hash(唯一)';
comment on column user_avatars.path is '头像位置,可以是cdn或者某个资源位置';