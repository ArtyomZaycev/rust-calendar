INSERT INTO `users` (`id`, `name`, `email`, `password`)
VALUES(2, "demo", "demo@aspid.xyz", SHA2("1", 512));

INSERT INTO `access_levels`(`user_id`, `name`, `level`)
VALUES(2, "Full", 255);
