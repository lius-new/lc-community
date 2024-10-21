-- Add migration script here
-- 创建用户登陆信息表
create table user_login_infos
(
    id                     serial
        constraint user_login_infos_pk
            primary key,

    login_created_time timestamp with time zone default now(),
    login_created_address  point,
    logout_created_time    timestamp with time zone,
    logout_created_address point,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    uuid                   varchar(64) unique             not null,
    foreign key (uuid) references users (uuid)
);
comment on table user_login_infos is '用户登陆信息表';
comment on column user_login_infos.login_created_time is '用户登陆时间';
comment on column user_login_infos.login_created_address is '用户登陆时地址';
comment on column user_login_infos.logout_created_time is '用户退出时间';
comment on column user_login_infos.logout_created_address is '用户退出地址';
comment on column user_login_infos.uuid is '用户uuid';
