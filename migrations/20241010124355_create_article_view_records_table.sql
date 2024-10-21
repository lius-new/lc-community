-- Add migration script here
-- 创建文章浏览记录表(该表记录用户访问文章的记录)
create table article_view_records
(
    id         serial
        constraint article_view_records_pk
            primary key,
    uuid       varchar(64)                    not null,
    article_id int                            not null,
    created_at timestamp with time zone default now() not null,
    updated_at timestamp,
    deleted_at timestamp,
    foreign key (uuid) references users (uuid),
    foreign key (article_id) references articles (id)
);

comment on table article_view_records is '文章浏览记录表';
comment on column article_view_records.uuid is '用户uuid';
comment on column article_view_records.article_id is '文章id';
