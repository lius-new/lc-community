------------------ group 组 初始化
-- 最高管理员组
insert into user_groups(id, name, description)
values (1, 'ROOT_GROUP', '超级管理员组，拥有对系统的一切权限');

-- 普通管理员组
insert into user_groups(id, name, description, parent_group_id)
values (2, 'USER_ADMIN_GROUP', '管理员组，拥有管理用户权限.(对象为用户)', 1);
insert into user_groups(id, name, description, parent_group_id)
values (3, 'ARTICLE_ADMIN_GROUP', '管理员组，拥有管理文章权限.(对象为文章)', 1);

-- 用户组(细分)
insert into user_groups(id, name, description, parent_group_id)
values (4, 'USER_ADMIN_WRITE_GROUP', '管理员编辑组，拥有编辑用户信息权限.(对象为用户)', 2);
insert into user_groups(id, name, description, parent_group_id)
values (5, 'USER_ADMIN_READ_GROUP', '管理员查看组，拥有查看用户的权限.(对象为用户)', 2);
insert into user_groups(id, name, description, parent_group_id)
values (6, 'USER_VIEW_GROUP', '用户普通组，拥有查看用户本人信息的权限.(对象为用户)', 5);

-- 文章组(细分)
insert into user_groups(id, name, description, parent_group_id)
values (7, 'ARTICLE_ADMIN_WRITE_GROUP', '文章管理员编辑组，拥有编辑文章权限.(对象为文章)', 3);
insert into user_groups(id, name, description, parent_group_id)
values (8, 'ARTICLE_ADMIN_READ_GROUP', '文章管理员查看组，拥有查看文章权限.(对象为文章)', 3);
insert into user_groups(id, name, description, parent_group_id)
values (9, 'ARTICLE_READ_GROUP', '文章查看组，拥有查看文章权限.(对象为文章)', 8);

-- 文章分组的组(文章对应不同的类别，对应类别而分的组，只有对应的权限才可以管理组，编辑组，查看组.)
insert into user_groups(id, name, description, parent_group_id)
values (10, 'ARTICLE_GROUP_ADMIN_GROUP', '文章组对应的管理员组，拥有管理文章组权限.(对象为文章组)', 1);
insert into user_groups(id, name, description, parent_group_id)
values (11, 'ARTICLE_GROUP_ADMIN_WRITE_GROUP', '文章组对应的管理员组(只可编辑)，拥有管理文章组权限.(对象为文章组)', 10);
insert into user_groups(id, name, description, parent_group_id)
values (12, 'ARTICLE_GROUP_ADMIN_READ_GROUP', '文章组对应的管理员组(只可查看)，拥有管理文章组权限.(对象为文章组)', 10);
insert into user_groups(id, name, description, parent_group_id)
values (13, 'ARTICLE_GROUP__GROUP', '文章组对应的普通组(只可查看)，拥有查看文章组权限.(对象为文章组)', 12);

-- 文章标签组
insert into user_groups(id, name, description, parent_group_id)
values (14, 'ARTICLE_TAG_ADMIN_GROUP', '文章标签对应的管理员组，拥有管理文章标签权限.(对象为文章标签)', 1);
insert into user_groups(id, name, description, parent_group_id)
values (15, 'ARTICLE_TAG_ADMIN_WRITE_GROUP', '文章标签对应的管理员组(只可编辑)，拥有管理文章标签权限.(对象为文章标签)',
        14);
insert into user_groups(id, name, description, parent_group_id)
values (16, 'ARTICLE_TAG_ADMIN_READ_GROUP', '文章标签对应的管理员组(只可查看)，拥有管理文章标签权限.(对象为文章标签)',
        14);
insert into user_groups(id, name, description, parent_group_id)
values (17, 'ARTICLE_TAG_GROUP', '文章标签对应的普通组(只可查看)，拥有查看文章标签权限.(对象为文章标签)', 16);


------------------ role 角色 初始化
insert into user_roles(id, name, description)
values (1, 'ROOT_ROLE', '最高角色.(该角色拥有最高权限)');

insert into user_roles(id, name, description, parent_role_id)
values (2, 'USER_ADMIN_ROLE', '管理角色.(该角色拥有最高管理用户权限)', 1);

insert into user_roles(id, name, description, parent_role_id)
values (3, 'USER_ADMIN_WRITE_ROLE', '管理角色.(该角色拥有编辑用户权限)', 2);

insert into user_roles(id, name, description, parent_role_id)
values (4, 'USER_ADMIN_READ_ROLE', '管理角色.(该角色拥有查看用户权限)', 2);

insert into user_roles(id, name, description, parent_role_id)
values (5, 'USER_ROLE', '普通角色.(该角色拥有普通用户权限)', 4);

insert into user_roles(id, name, description, parent_role_id)
values (6, 'ARTICLE_ADMIN_ROLE', '管理角色.(该角色拥有最高管理文章权限)', 1);

insert into user_roles(id, name, description, parent_role_id)
values (7, 'ARTICLE_ADMIN_WRITE_ROLE', '管理角色.(该角色拥有编辑文章权限)', 6);

insert into user_roles(id, name, description, parent_role_id)
values (8, 'ARTICLE_ADMIN_READ_ROLE', '管理角色.(该角色拥有查看文章权限)', 6);

insert into user_roles(id, name, description, parent_role_id)
values (9, 'ARTICLE_ROLE', '普通角色.(该角色拥有查看文章权限)', 8);

insert into user_roles(id, name, description, parent_role_id)
values (10, 'ARTICLE_GROUP_ADMIN_ROLE', '管理角色.(该角色拥有管理文章分组权限)', 1);
insert into user_roles(id, name, description, parent_role_id)
values (11, 'ARTICLE_GROUP_ADMIN_WRITE_ROLE', '管理角色.(该角色拥有编辑文章分组权限)', 10);
insert into user_roles(id, name, description, parent_role_id)
values (12, 'ARTICLE_GROUP_ADMIN_READ_ROLE', '管理角色.(该角色拥有查看文章分组权限)', 10);
insert into user_roles(id, name, description, parent_role_id)
values (13, 'ARTICLE_GROUP_ROLE', '普通角色.(该角色拥有查看文章分组权限)', 11);


insert into user_roles(id, name, description, parent_role_id)
values (14, 'ARTICLE_TAG_ADMIN_ROLE', '管理角色.(该角色拥有管理文章标签权限)', 1);
insert into user_roles(id, name, description, parent_role_id)
values (15, 'ARTICLE_TAG_ADMIN_WRITE_ROLE', '管理角色.(该角色拥有编辑标签分组权限)', 14);
insert into user_roles(id, name, description, parent_role_id)
values (16, 'ARTICLE_TAG_ADMIN_READ_ROLE', '管理角色.(该角色拥有查看标签分组权限)', 14);
insert into user_roles(id, name, description, parent_role_id)
values (17, 'ARTICLE_TAG_ROLE', '普通角色.(该角色拥有查看标签分组权限)', 16);


------------------ permission 权限 初始化
insert into user_permissions(id, name, description)
values (1, 'ROOT_PERMISSION', '最高权限.(拥有最高权限)');

insert into user_permissions(id, name, description, parent_permission_id)
values (2, 'USER_ADMIN_PERMISSION', '管理角色.(该角色拥有最高管理用户权限)', 1);

insert into user_permissions(id, name, description, parent_permission_id)
values (3, 'USER_ADMIN_WRITE_PERMISSION', '拥有编辑用户数据权限', 2);

insert into user_permissions(id, name, description, parent_permission_id)
values (4, 'USER_ADMIN_READ_PERMISSION', '拥有查看用户权限', 2);

insert into user_permissions(id, name, description, parent_permission_id)
values (5, 'USER_PERMISSION', '拥有普通用户权限', 4);

insert into user_permissions(id, name, description, parent_permission_id)
values (6, 'ARTICLE_ADMIN_PERMISSION', '最高管理文章权限', 1);
insert into user_permissions(id, name, description, parent_permission_id)
values (7, 'ARTICLE_ADMIN_WRITE_PERMISSION', '拥有编辑文章权限', 6);
insert into user_permissions(id, name, description, parent_permission_id)
values (8, 'ARTICLE_ADMIN_READ_PERMISSION', '拥有查看文章权限', 6);
insert into user_permissions(id, name, description, parent_permission_id)
values (9, 'ARTICLE_PERMISSION', '拥有查看文章权限', 8);

insert into user_permissions(id, name, description, parent_permission_id)
values (10, 'ARTICLE_GROUP_ADMIN_PERMISSION', '最高管理文章分组权限', 1);
insert into user_permissions(id, name, description, parent_permission_id)
values (11, 'ARTICLE_GROUP_ADMIN_WRITE_PERMISSION', '拥有编辑文章分组权限', 10);
insert into user_permissions(id, name, description, parent_permission_id)
values (12, 'ARTICLE_GROUP_ADMIN_READ_PERMISSION', '拥有查看文章分组权限', 10);
insert into user_permissions(id, name, description, parent_permission_id)
values (13, 'ARTICLE_PERMISSION', '拥有查看文章分组权限', 12);

insert into user_permissions(id, name, description, parent_permission_id)
values (14, 'ARTICLE_TAG_ADMIN_PERMISSION', '最高管理文章标签权限', 1);
insert into user_permissions(id, name, description, parent_permission_id)
values (15, 'ARTICLE_TAG_ADMIN_WRITE_PERMISSION', '拥有编辑文章标签权限', 14);
insert into user_permissions(id, name, description, parent_permission_id)
values (16, 'ARTICLE_TAG_ADMIN_READ_PERMISSION', '拥有查看文章标签权限', 15);
insert into user_permissions(id, name, description, parent_permission_id)
values (17, 'ARTICLE_TAG_PERMISSION', '拥有查看文章标签权限', 16);

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
