-- Add migration script here
-- 创建用户信息表
create table user_infos
(
    id         serial
        constraint user_infos_pk
            primary key,
    nickname   varchar(24)                    not null,
    password   varchar(32)                    not null,
    email      varchar(32),
    phone      varchar(18),
    gender     boolean,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    uuid       varchar(64) unique             not null,
    constraint uuid_foreign foreign key (uuid) references users (uuid)
);
comment on table user_infos is '用户信息表';
comment on column user_infos.nickname is '用户昵称';
comment on column user_infos.password is '用户密码';
comment on column user_infos.email is '用户邮箱';
comment on column user_infos.phone is '用户电话';
comment on column user_infos.gender is '用户性别';
comment on column user_infos.uuid is '用户uuid';
-- comment on constraint uuid_foreign on user_infos is 'uuid是外键';
