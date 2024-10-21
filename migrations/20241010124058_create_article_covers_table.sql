-- Add migration script here
-- 创建文章图片(文章在列表时显示的图片)
create table article_covers
(
    id         serial
        constraint article_covers_pk
            primary key,
    hash       varchar(128)                   not null,
    path       varchar(256)                   not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    article_id int                            not null,
    foreign key (article_id) references articles (id)
);
comment on table article_covers is '文章在列表时显示的图片';
comment on column article_covers.hash is '图片对应的hash';
comment on column article_covers.path is '图片对应标签';
comment on column article_covers.article_id is '图片对应文章';
