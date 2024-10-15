-- 创建用户表
create table users
(
    id         serial
        constraint users_pk
            primary key,
    uuid       varchar(64) unique             not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time
);
comment on table users is '用户表';
comment on column users.id is '用户id';
comment on column users.uuid is '用户uuid';

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
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
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

-- 创建用户登陆信息表
create table user_login_infos
(
    id                     serial
        constraint user_login_infos_pk
            primary key,

    login_created_time     time,
    login_created_address  point,
    logout_created_time    time,
    logout_created_address point,
    created_at             time default current_timestamp not null,
    updated_at             time,
    deleted_at             time,
    uuid                   varchar(64) unique             not null,
    foreign key (uuid) references users (uuid)
);
comment on table user_login_infos is '用户登陆信息表';
comment on column user_login_infos.login_created_time is '用户登陆时间';
comment on column user_login_infos.login_created_address is '用户登陆时地址';
comment on column user_login_infos.logout_created_time is '用户退出时间';
comment on column user_login_infos.logout_created_address is '用户退出地址';
comment on column user_login_infos.uuid is '用户uuid';

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

-- 创建用户关系类型表(好友关系，关注关系等。。。)
create table user_relation_types
(
    id          serial
        constraint user_relation_types_pk
            primary key,
    name        varchar(64)                    not null,
    description varchar(128)                   not null,
    created_at  time default current_timestamp not null,
    updated_at  time,
    deleted_at  time
);
comment on table user_relation_types is '用户关系表';
comment on column user_relation_types.name is '用户关系名称, 如："好友，关注，拉黑等"';
comment on column user_relation_types.description is '用户关系描述"';

-- 创建用户关系状态表
create table user_relation_status
(
    id         serial
        constraint user_relation_status_pk
            primary key,
    status     bool                           not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time
);
comment on table user_relation_status is '用户关系状态';
comment on column user_relation_status.status is '用户关系状态名称，如：建立好友关系时对方未同意(false)等';

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

-- 创建用户间沟通表(不包含沟通内容)
create table user_communicates
(
    id           serial
        constraint user_communicates_pk
            primary key,
    created_at   time default current_timestamp not null,
    updated_at   time,
    deleted_at   time,
    uuid         varchar(64)                    not null,
    current_uuid varchar(64)                    not null,
    foreign key (uuid) references users (uuid),
    foreign key (current_uuid) references users (uuid),
    constraint check_uuid_different check ( uuid != current_uuid )
);
comment on table user_communicates is '用户沟通表';
comment on column user_communicates.uuid is '用户uuid';
comment on column user_communicates.current_uuid is '对方用户的uuid';

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

-- 创建用户分组表
create table user_groups
(
    id              serial
        constraint user_groups_pk
            primary key,
    name            varchar(52)                    not null,
    description     varchar(256)                   not null,
    parent_group_id int                            ,
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

-- 创建角色表
create table user_roles
(
    id             serial
        constraint user_roles_pk
            primary key,
    name           varchar(52)                    not null,
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
comment on column user_roles.parent_role_id is '角色父级id';

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

-- 创建用户权限关联表
create table user_permissions
(
    id                   serial
        constraint user_permissions_pk
            primary key,
    name                 varchar(52)                    not null,
    description          varchar(256)                   not null,
    parent_permission_id int                            not null,
    created_at           time default current_timestamp not null,
    updated_at           time,
    deleted_at           time,
    foreign key (parent_permission_id) references user_permissions (id),
    constraint id_parent_group_id_different check ( user_permissions.id != user_permissions.parent_permission_id )
);
comment on table user_permissions is '用户权限表';
comment on column user_permissions.name is '权限名称';
comment on column user_permissions.description is '权限描述';
comment on column user_permissions.parent_permission_id is '权限父级id';

-- 创建用户角色权限关联表
create table user_role_permission_relations
(
    id                 serial
        constraint user_role_permission_relations_pk
            primary key,
    user_role_id       int                            not null,
    user_permission_id int                            not null,
    created_at         time default current_timestamp not null,
    updated_at         time,
    deleted_at         time,
    foreign key (user_role_id) references user_roles (id),
    foreign key (user_permission_id) references user_permissions (id)
);
comment on table user_role_permission_relations is '用户角色权限关联表';
comment on column user_role_permission_relations.user_role_id is '用户角色id';
comment on column user_role_permission_relations.user_permission_id is '用户权限id';

-- 创建用户资源表(保存用户相关接口资源)
create table user_resources
(
    id          serial
        constraint user_resources_pk
            primary key,
    name        varchar(24)                    not null,
    description varchar(48)                    not null,
    resource    varchar(72)                    not null,
    can_use     bool default true,
    created_at  time default current_timestamp not null,
    updated_at  time,
    deleted_at  time
);
comment on table user_resources is '用户资源表';
comment on column user_resources.name is '用户资源名称';
comment on column user_resources.description is '用户资源描述';
comment on column user_resources.resource is '用户资源内容: (http example: POST+/users/login)';
comment on column user_resources.can_use is '接口是否可用(开放)';


--创建文章分组 (每个文章必然属于一个或多个分组，一个分组必然包含多个文章)
create table article_groups
(
    id          serial
        constraint article_groups_pk
            primary key,
    name        varchar(24)                    not null,
    description varchar(256)                   not null,
    created_at  time default current_timestamp not null,
    updated_at  time,
    deleted_at  time
);
comment on table article_groups is '社区分组表';
comment on column article_groups.name is '分组名称';
comment on column article_groups.description is '分组描述';

-- 创建博客文章表
create table articles
(
    id          serial
        constraint articles_pk
            primary key,
    title       varchar(64)                    not null,
    description varchar(256)                   not null,
    content     varchar                        not null,
    hash        varchar(128)                   not null,
    created_at  time default current_timestamp not null,
    updated_at  time,
    deleted_at  time
);
comment on table articles is '文章表';
comment on column articles.title is '文章标题';
comment on column articles.description is '文章描述';
comment on column articles.content is '文章内容';
comment on column articles.hash is '文章hash';

-- 创建文章图片(文章在列表时显示的图片)
create table article_covers
(
    id         serial
        constraint article_covers_pk
            primary key,
    hash       varchar(128)                   not null,
    path       varchar(256)                   not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
    article_id int                            not null,
    foreign key (article_id) references articles (id)
);
comment on table article_covers is '文章在列表时显示的图片';
comment on column article_covers.hash is '图片对应的hash';
comment on column article_covers.path is '图片对应标签';
comment on column article_covers.article_id is '图片对应文章';

-- 创建文章图片(文章中对应的图片)
create table article_images
(
    id         serial
        constraint article_images_pk
            primary key,
    hash       varchar(128)                   not null,
    path       varchar(256)                   not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
    article_id int                            not null,
    foreign key (article_id) references articles (id)
);
comment on table article_images is '文章对应的图片';
comment on column article_images.hash is '图片对应的hash';
comment on column article_images.path is '图片对应标签';
comment on column article_images.article_id is '图片对应文章';

-- 创建分组文章关联表
create table article_groups_relations
(
    id               serial
        constraint article_groups_relations_pk
            primary key,
    article_group_id int                            not null,
    article_id       int                            not null,
    created_at       time default current_timestamp not null,
    updated_at       time,
    deleted_at       time,
    foreign key (article_group_id) references article_groups (id),
    foreign key (article_id) references articles (id)
);
comment on table article_groups_relations is '分组文章关联表(M:N)';
comment on column article_groups_relations.article_group_id is '分组id';
comment on column article_groups_relations.article_id is '文章id';


-- 创建用户文章关联表
create table user_article_relations
(
    id         serial
        constraint user_article_relations_pk
            primary key,
    uuid       varchar(64)                    not null,
    article_id int                            not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
    foreign key (uuid) references users (uuid),
    foreign key (article_id) references articles (id)
);
comment on table user_article_relations is '用户文章关联表(M:N)';
comment on column user_article_relations.uuid is '用户uuid';
comment on column  user_article_relations.article_id is '文章id';

-- 创建文章资源表
create table article_resources
(
    id          serial
        constraint article_resources_pk
            primary key,
    name        varchar(24)                    not null,
    description varchar(48)                    not null,
    resource    varchar(72)                    not null,
    can_use     bool default true,
    created_at  time default current_timestamp not null,
    updated_at  time,
    deleted_at  time
);
comment on table article_resources is '文章资源表';
comment on column article_resources.name is '文章资源名称';
comment on column article_resources.description is '文章资源描述';
comment on column article_resources.resource is '文章资源内容: (http example: POST+/users/login)';
comment on column article_resources.can_use is '接口是否可用(开放)';

-- 创建博客标签表
create table article_tags
(
    id          serial
        constraint article_tags_pk
            primary key,
    name        varchar(24)                    not null,
    description varchar(256)                   not null,
    created_at  time default current_timestamp not null,
    updated_at  time,
    deleted_at  time
);
comment on table article_tags is '文章标签表';
comment on column article_tags.name is '文章标签名称';
comment on column article_tags.description is '文章标签描述';

-- 创建博客文章标签关联表
create table article_tag_relations
(
    id         serial
        constraint article_tag_relations_pk
            primary key,
    article_id int                            not null,
    tag_id     int                            not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
    foreign key (article_id) references articles (id),
    foreign key (tag_id) references article_tags (id)
);
comment on table article_tag_relations is '文章标签关联表(M:N)';
comment on column article_tag_relations.article_id is '文章id';
comment on column article_tag_relations.tag_id is '标签id';

-- 创建文章点赞表
create table article_likes
(
    id         serial
        constraint article_likes_pk
            primary key,
    uuid       varchar(64)                    not null,
    article_id int                            not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
    foreign key (uuid) references users (uuid),
    foreign key (article_id) references articles (id),
    -- 表示uuid和article_id只能同时出现一次.(多次点赞)
    constraint unique_user_article_like unique (uuid, article_id)
);

comment on table article_likes is '文章点赞表';
comment on column article_likes.uuid is '用户uuid';
comment on column article_likes.article_id is '文章id';

-- 创建文章收藏表
create table article_collects
(
    id         serial
        constraint article_collects_pk
            primary key,
    uuid       varchar(64)                    not null,
    article_id int                            not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
    foreign key (uuid) references users (uuid),
    foreign key (article_id) references articles (id),
    -- 表示uuid和article_id只能同时出现一次.(多次点赞)
    constraint unique_user_article_collect unique (uuid, article_id)
);

comment on table article_collects is '文章收藏表';
comment on column article_collects.uuid is '用户uuid';
comment on column article_collects.article_id is '文章id';

-- 创建文章评论表
create table article_comments
(
    id                serial
        constraint article_comments_pk
            primary key,
    article_id        int                            not null,
    parent_comment_id int,
    content           varchar(256)                   not null,
    created_at        time default current_timestamp not null,
    updated_at        time,
    deleted_at        time,
    foreign key (parent_comment_id) references article_comments (id),
    foreign key (article_id) references articles (id)
);
comment on table article_comments is '文章评论表';
comment on column article_comments.article_id is '文章id';
comment on column article_comments.parent_comment_id is '文章评论id, 如果为空则表示是属于文章评论, 如果不为空则表示该评论属于某条评论';
comment on column article_comments.content is '评论内容';

-- 创建文章浏览记录表(该表记录用户访问文章的记录)
create table article_view_records
(
    id         serial
        constraint article_view_records_pk
            primary key,
    uuid       varchar(64)                    not null,
    article_id int                            not null,
    created_at time default current_timestamp not null,
    updated_at time,
    deleted_at time,
    foreign key (uuid) references users (uuid),
    foreign key (article_id) references articles (id)
);

comment on table article_view_records is '文章浏览记录表';
comment on column article_view_records.uuid is '用户uuid';
comment on column article_view_records.article_id is '文章id';


-- 创建权限与用户资源关联表
create table permission_user_resource_relations
(
    id                 serial
        constraint permission_user_resource_relations_pk
            primary key,
    resource_id        int                            not null,
    user_permission_id int                            not null,
    created_at         time default current_timestamp not null,
    updated_at         time,
    deleted_at         time,
    foreign key (user_permission_id) references user_permissions (id),
    foreign key (resource_id) references user_resources (id)

);
comment on table permission_user_resource_relations is '权限与用户资源关联表(M:N)';
comment on column permission_user_resource_relations.resource_id is '资源对应的id';
comment on column permission_user_resource_relations.user_permission_id is '资源对应的权限id';


-- 创建权限与文章资源关联表
create table permission_article_resource_relations
(
    id                 serial
        constraint permission_article_resource_relations_pk
            primary key,
    resource_id        int                            not null,
    user_permission_id int                            not null,
    created_at         time default current_timestamp not null,
    updated_at         time,
    deleted_at         time,
    foreign key (user_permission_id) references user_permissions (id),
    foreign key (resource_id) references article_resources (id)

);
comment on table permission_article_resource_relations is '权限与文章资源关联表(M:N)';
comment on column permission_article_resource_relations.resource_id is '资源对应的id';
comment on column permission_article_resource_relations.user_permission_id is '资源对应的权限id';
