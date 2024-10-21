-- Add migration script here
-- 创建博客文章标签关联表
create table article_tag_relations
(
    id         serial
        constraint article_tag_relations_pk
            primary key,
    article_id int                            not null,
    tag_id     int                            not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    foreign key (article_id) references articles (id),
    foreign key (tag_id) references article_tags (id)
);
comment on table article_tag_relations is '文章标签关联表(M:N)';
comment on column article_tag_relations.article_id is '文章id';
comment on column article_tag_relations.tag_id is '标签id';
