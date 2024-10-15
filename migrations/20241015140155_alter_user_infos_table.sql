-- Add migration script here
alter table user_infos alter column password type varchar(128);
