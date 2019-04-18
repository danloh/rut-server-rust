-- Your SQL goes here

ALTER TABLE ruts ADD COLUMN slug VARCHAR;

UPDATE ruts SET slug = id;

ALTER TABLE items ADD COLUMN slug VARCHAR;

UPDATE items SET slug = id;

ALTER TABLE ruts ALTER COLUMN slug SET NOT NULL;
ALTER TABLE items ALTER COLUMN slug SET NOT NULL;
ALTER TABLE ruts ADD CONSTRAINT ruts_slug_key UNIQUE (slug);
ALTER TABLE items ADD CONSTRAINT items_slug_key UNIQUE (slug);
