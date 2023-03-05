# Event template - easy template for the event, will be used in the future to impruve UX
# Event plan - when to create some event based on schedule
# Schedule - group of scheduled events

CREATE TABLE `event_templates` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `user_id` int NOT NULL,
  `name` varchar(255) NOT NULL,
  `event_name` varchar(255) NOT NULL,
  `event_description` varchar(255),
  `duration` int NOT NULL COMMENT 'How long in minutes does an event last',
  `access_level` int NOT NULL
);

CREATE TABLE `event_plans` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `schedule_id` int NOT NULL,
  `weekday` tinyint NOT NULL,
  `time` smallint NOT NULL
);

CREATE TABLE `schedules` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `user_id` int NOT NULL,
  `template_id` int NOT NULL,
  `name` varchar(255) NOT NULL,
  `description` varchar(255),
  `first_day` date NOT NULL,
  `last_day` date,
  `access_level` int NOT NULL,
  `deleted` boolean NOT NULL DEFAULT false
);

ALTER TABLE `event_templates` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

ALTER TABLE `event_plans` ADD FOREIGN KEY (`schedule_id`) REFERENCES `schedules` (`id`) ON DELETE RESTRICT;

ALTER TABLE `schedules` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;
ALTER TABLE `schedules` ADD FOREIGN KEY (`template_id`) REFERENCES `event_templates` (`id`) ON DELETE RESTRICT;

ALTER TABLE `events`
  ADD `plan_id` int DEFAULT NULL
  AFTER `access_level`;

ALTER TABLE `events` ADD FOREIGN KEY (`plan_id`) REFERENCES `event_plans` (`id`) ON DELETE SET NULL;