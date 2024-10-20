-- Add migration script here

ALTER TABLE user_resources ADD  COLUMN method VARCHAR(12) NOT NULL;
COMMENT ON COLUMN  user_resources.method IS '资源的请求方法';

ALTER TABLE article_resources ADD  COLUMN method VARCHAR(12) NOT NULL;
COMMENT ON COLUMN article_resources.method IS '资源的请求方法';
