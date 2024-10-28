-- Add migration script here

alter table articles add column visiable boolean not null default true;
comment on column articles.visiable is '文章可见性';
