-- Add migration script here
-- 创建用户文章关联表
create table user_article_relations
(
    id         serial
        constraint user_article_relations_pk
            primary key,
    uuid       varchar(64)                    not null,
    article_id int                            not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    foreign key (uuid) references users (uuid),
    foreign key (article_id) references articles (id)
);
comment on table user_article_relations is '用户文章关联表(M:N)';
comment on column user_article_relations.uuid is '用户uuid';
comment on column  user_article_relations.article_id is '文章id';
