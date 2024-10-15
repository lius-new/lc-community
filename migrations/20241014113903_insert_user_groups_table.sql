-- Add migration script here
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


