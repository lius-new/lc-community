-- Add migration script here

------------------  user_role_permission_relations 关联表
-- 超级管理角色对应的权限。
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (1, 1);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (1, 2);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (1, 3);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (1, 4);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (1, 5);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (1, 6);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (1, 7);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (1, 8);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (1, 9);

-- 最高管理用户角色对应权限。
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (2, 2);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (2, 3);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (2, 4);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (2, 5);

---- 可编辑管理用户角色对应权限。
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (3, 3);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (3, 4);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (3, 5);

-- 可查看管理用户角色对应权限
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (4, 4);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (4, 5);

-- 普通用户对应的权限
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (5, 5);

---- 最高管理文章角色对应权限。
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (6, 6);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (6, 7);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (6, 8);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (6, 9);

-- 可编辑管理文章角色对应权限。
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (7, 7);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (7, 8);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (7, 9);

-- 可查看(管理)文章角色对应权限。(如可以修改文章可见性之类。)
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (8, 8);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (8, 9);

-- 普通浏览文章对应角色权限
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (9, 9);

---- 最高管理文章分组角色对应权限。
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (10, 10);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (10, 11);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (10, 12);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (10, 13);

-- 可编辑管理文章分组角色对应权限。
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (11, 11);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (11, 12);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (11, 13);

-- 可查看(管理)文章分组角色对应权限。(如可以修改文章分组可见性之类。)
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (12, 12);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (12, 13);

-- 普通浏览文章分组对应角色权限
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (13, 13);

---- 最高管理文章分组角色对应权限。
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (14, 14);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (14, 15);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (14, 16);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (14, 17);

-- 可编辑管理文章分组角色对应权限。
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (15, 15);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (15, 16);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (15, 17);

-- 可查看(管理)文章分组角色对应权限。(如可以修改文章分组可见性之类。)
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (16, 16);
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (16, 17);

-- 普通浏览文章分组对应角色权限
insert into user_role_permission_relations(user_role_id, user_permission_id)
values (17, 17);
