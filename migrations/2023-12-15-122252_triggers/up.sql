CREATE TABLE IF NOT EXISTS `logs` (
    `id` int NOT NULL AUTO_INCREMENT,
    `tag` varchar(20) NOT NULL,
    `message` text,
	PRIMARY KEY (`id`)
);

CREATE TRIGGER user_roles_update_trigger
BEFORE UPDATE ON `user_roles`
FOR EACH ROW
BEGIN
    INSERT INTO `logs`(`tag`, `message`)
    VALUES ('user_roles_upd', CONCAT(CONVERT(OLD.user_id, CHAR), '(', CONVERT(OLD.role_id, CHAR), ')=>', CONVERT(NEW.user_id, CHAR), '(', CONVERT(NEW.role_id, CHAR), ')'));
END;

CREATE TRIGGER user_roles_insert_trigger
AFTER INSERT ON `user_roles`
FOR EACH ROW
BEGIN
    INSERT INTO `logs`(`tag`, `message`)
    VALUES ('user_roles_ins', CONCAT(CONVERT(NEW.user_id, CHAR), '(', CONVERT(NEW.role_id, CHAR), ')'));
END;

CREATE TRIGGER user_roles_delete_trigger
BEFORE DELETE ON `user_roles`
FOR EACH ROW
BEGIN
    INSERT INTO `logs`(`tag`, `message`)
    VALUES ('user_roles_del', CONCAT(CONVERT(OLD.user_id, CHAR), '(', CONVERT(OLD.role_id, CHAR), ')'));
END;