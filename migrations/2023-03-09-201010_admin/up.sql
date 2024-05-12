INSERT INTO `users` (`id`, `name`, `email`)
VALUES(1, "admin", "admin@aspid.xyz");

INSERT INTO `passwords`(`user_id`, `name`, `password`, `access_level`, `edit_right`)
VALUES(1, "Full", "1", 255, true);

# Grant SuperAdmin & Admin roles
INSERT INTO `user_roles`(`user_id`, `role_id`)
VALUES (1, 1), (1, 2);