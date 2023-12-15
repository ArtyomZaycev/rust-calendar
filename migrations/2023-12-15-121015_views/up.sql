CREATE VIEW `admin_users`
AS SELECT DISTINCT `users`.*
FROM `users`
INNER JOIN `user_roles` ON `user_roles`.`user_id`=`users`.`id` AND (`user_roles`.`role_id`=1 OR `user_roles`.`role_id`=2);

CREATE VIEW `superadmin_users`
AS SELECT DISTINCT `users`.*
FROM `users`
INNER JOIN `user_roles` ON `user_roles`.`user_id`=`users`.`id` AND `user_roles`.`role_id`=1;

CREATE VIEW `completed_events`
AS SELECT `events`.*
FROM `events`
WHERE `end` > CURRENT_TIMESTAMP();