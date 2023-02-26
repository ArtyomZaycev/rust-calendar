INSERT INTO `users` (`id`, `name`, `email`)
VALUES(1, "admin", "admin@aspid.xyz");

INSERT INTO `passwords`(`user_id`, `password`, `access_level`)
VALUES(1, "1", 1000);

# Grant SuperAdmin & Admin roles
INSERT INTO `user_roles`(`user_id`, `role_id`)
VALUES (1, 1), (1, 2);