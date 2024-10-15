-- Add migration script here

------------------ user_roles 角色 初始化
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
