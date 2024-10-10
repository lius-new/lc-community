-- Add migration script here
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