INSERT INTO `users` (`id`, `name`, `email`)
VALUES(2, "demo", "demo@aspid.xyz");

INSERT INTO `passwords`(`user_id`, `name`, `password`, `access_level`, `edit_right`)
VALUES(2, "Full", SHA2("1", 512), 1000, true);
