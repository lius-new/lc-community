-- Add migration script here
-- 创建用户间沟通记录表(仅沟通内容)
create table user_communicate_contents
(
    id                          serial
        constraint user_communicate_contents_pk
            primary key,
    created_at                  time default current_timestamp not null,
    updated_at                  time,
    deleted_at                  time,

    u1_send_content             varchar(256)                   not null,
    u2_send_content             varchar(256)                   not null,
    user_communicate_id         int                            not null,
    u1_send_content_read_status boolean                        not null,
    u2_send_content_read_status boolean                        not null,
    foreign key (user_communicate_id) references user_communicates (id)
);
comment on table user_communicate_contents is '用户沟通记录表';
comment on column user_communicate_contents.u1_send_content is '沟通发起者的沟通过程中的信息';
comment on column user_communicate_contents.u2_send_content is '沟通接收者的沟通过程中的信息';
comment on column user_communicate_contents.u1_send_content_read_status is '沟通发起者发送的消息是否被已读';
comment on column user_communicate_contents.u2_send_content_read_status is '沟通接收者发送的消息是否被已读';