CREATE TABLE `users` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `email` varchar(255) UNIQUE NOT NULL,
  `phone` varchar(255) DEFAULT NULL
);

CREATE TABLE `passwords` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `user_id` int NOT NULL,
  `password` varchar(255) NOT NULL,
  `access_level` int NOT NULL,
  `edit_right` boolean NOT NULL
);

CREATE TABLE `sessions` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `user_id` int NOT NULL,
  `key` binary(64) NOT NULL,
  `access_level` int NOT NULL,
  `edit_right` boolean NOT NULL,
  `start` timestamp NOT NULL,
  `end` timestamp NOT NULL,
  `valid` boolean NOT NULL DEFAULT true
);

CREATE TABLE `roles` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `name` varchar(255) UNIQUE NOT NULL,
  `description` varchar(255)
);

CREATE TABLE `user_roles` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `user_id` int NOT NULL,
  `role_id` int NOT NULL,
  `granted` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE `events` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `user_id` int NOT NULL,
  `name` varchar(255) NOT NULL,
  `description` varchar(255),
  `start` timestamp NOT NULL,
  `end` timestamp NOT NULL,
  `access_level` int NOT NULL
);

CREATE INDEX `passwords_index_0` ON `passwords` (`user_id`);

CREATE UNIQUE INDEX `passwords_index_1` ON `passwords` (`user_id`, `access_level`, `edit_right`);

CREATE UNIQUE INDEX `passwords_index_2` ON `passwords` (`user_id`, `password`);

CREATE INDEX `sessions_index_3` ON `sessions` (`user_id`);

CREATE UNIQUE INDEX `sessions_index_4` ON `sessions` (`user_id`, `key`);

CREATE INDEX `events_index_5` ON `events` (`user_id`);

ALTER TABLE `passwords` COMMENT = 'access_level=1000 - full access';

ALTER TABLE `passwords` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

ALTER TABLE `sessions` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

ALTER TABLE `user_roles` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

ALTER TABLE `user_roles` ADD FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`) ON DELETE RESTRICT;

ALTER TABLE `events` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;
