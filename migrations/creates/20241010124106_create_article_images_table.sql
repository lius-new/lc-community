-- Add migration script here
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