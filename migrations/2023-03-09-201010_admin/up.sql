INSERT INTO `users` (`id`, `name`, `email`, `password`)
VALUES(1, "admin", "admin@aspid.xyz", SHA2("1", 512));

INSERT INTO `access_levels`(`user_id`, `name`, `level`)
VALUES(1, "Full", 255);

-- Grant SuperAdmin & Admin roles
INSERT INTO `user_roles`(`user_id`, `role_id`)
VALUES (1, 1), (1, 2);