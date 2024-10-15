-- Add migration script here

------------------  user_role_permission_relations 关联表
-- 最高管理员组拥有最高角色
insert into user_role_group_relations(user_group_id, user_role_id)
values (1, 1);

-- 用户组对应用户角色
insert into user_role_group_relations(user_group_id, user_role_id)
values (2, 2);
insert into user_role_group_relations(user_group_id, user_role_id)
values (4, 3);
insert into user_role_group_relations(user_group_id, user_role_id)
values (5, 4);
insert into user_role_group_relations(user_group_id, user_role_id)
values (6, 5);

-- 文章组对应文章角色
insert into user_role_group_relations(user_group_id, user_role_id)
values (3, 6);
insert into user_role_group_relations(user_group_id, user_role_id)
values (7, 7);
insert into user_role_group_relations(user_group_id, user_role_id)
values (8, 8);
insert into user_role_group_relations(user_group_id, user_role_id)
values (9, 9);

-- 文章组的组对应文章组角色
insert into user_role_group_relations(user_group_id, user_role_id)
values (10, 10);
insert into user_role_group_relations(user_group_id, user_role_id)
values (11, 11);
insert into user_role_group_relations(user_group_id, user_role_id)
values (12, 12);
insert into user_role_group_relations(user_group_id, user_role_id)
values (13, 13);

-- 文章标签的组对应文章标签角色
insert into user_role_group_relations(user_group_id, user_role_id)
values (14, 14);
insert into user_role_group_relations(user_group_id, user_role_id)
values (15, 15);
insert into user_role_group_relations(user_group_id, user_role_id)
values (16, 16);
insert into user_role_group_relations(user_group_id, user_role_id)
values (17, 17);
