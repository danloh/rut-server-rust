-- Your SQL goes here

ALTER TABLE ruts ADD COLUMN slug VARCHAR;

UPDATE ruts SET slug = id;

ALTER TABLE items ADD COLUMN slug VARCHAR;

UPDATE items SET slug = id;

ALTER TABLE ruts ALTER COLUMN slug SET NOT NULL;
ALTER TABLE items ALTER COLUMN slug SET NOT NULL;
ALTER TABLE ruts ADD CONSTRAINT ruts_slug_key UNIQUE (slug);
ALTER TABLE items ADD CONSTRAINT items_slug_key UNIQUE (slug);


ALTER TABLE ruts RENAME COLUMN author_id TO author;

UPDATE staritems SET flag = 1 where flag = 'todo';
UPDATE staritems SET flag = 1 where flag = 'Todo';
UPDATE staritems SET flag = 2 where flag = 'doing';
UPDATE staritems SET flag = 2 where flag = 'Doing';
UPDATE staritems SET flag = 3 where flag = 'done';
UPDATE staritems SET flag = 3 where flag = 'Done';

ALTER TABLE staritems 
 ALTER COLUMN flag SET DATA TYPE smallint USING flag :: smallint,
 ALTER COLUMN flag DROP DEFAULT;

ALTER TABLE staritems 
ALTER COLUMN rate DROP DEFAULT,
ALTER COLUMN rate SET DATA TYPE smallint USING rate :: smallint;

ALTER TABLE collects 
ALTER COLUMN item_order SET DATA TYPE smallint USING item_order :: smallint;
