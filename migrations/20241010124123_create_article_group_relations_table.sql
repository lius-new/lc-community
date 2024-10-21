-- Add migration script here
-- 创建分组文章关联表
create table article_groups_relations
(
    id               serial
        constraint article_groups_relations_pk
            primary key,
    article_group_id int                            not null,
    article_id       int                            not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    foreign key (article_group_id) references article_groups (id),
    foreign key (article_id) references articles (id)
);
comment on table article_groups_relations is '分组文章关联表(M:N)';
comment on column article_groups_relations.article_group_id is '分组id';
comment on column article_groups_relations.article_id is '文章id';
