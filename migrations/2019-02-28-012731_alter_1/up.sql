-- Your SQL goes here

ALTER TABLE ruts ADD COLUMN user_name VARCHAR NOT NULL DEFAULT '';

ALTER TABLE tagruts RENAME COLUMN tag_id TO tname;

ALTER TABLE tagitems RENAME COLUMN tag_id TO tname;

ALTER TABLE tagetcs RENAME COLUMN tag_id TO tname;

ALTER TABLE tagruts ADD CONSTRAINT tag_rutid_key UNIQUE (tname, rut_id); 

ALTER TABLE tagitems ADD CONSTRAINT tag_itemid_key UNIQUE (tname, item_id); 

ALTER TABLE tagetcs ADD CONSTRAINT tag_ectid_key UNIQUE (tname, etc_id);

ALTER TABLE collects ADD CONSTRAINT rut_item_id_key UNIQUE (rut_id, item_id);

ALTER TABLE starruts ADD CONSTRAINT rut_user_id_key UNIQUE (rut_id, user_id);

ALTER TABLE startags ADD CONSTRAINT tag_user_id_key UNIQUE (tag_id, user_id);
