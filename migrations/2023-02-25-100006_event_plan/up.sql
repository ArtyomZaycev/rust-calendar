CREATE TABLE `schedules` (
  `id` int PRIMARY KEY NOT NULL AUTO_INCREMENT,
  `user_id` int NOT NULL,
  `start` timestamp NOT NULL,
  `weekday_filter` int NOT NULL DEFAULT 0b1111111 COMMENT 'Bitmask of weekdays starting from monday',
  `day_period` int COMMENT 'Number of days between events',
  `time_period` int COMMENT 'Number of minutes between events',
  `event_duration` int NOT NULL COMMENT 'How long in minutes does event last',
  `deleted` boolean NOT NULL DEFAULT FALSE
);

ALTER TABLE `schedules` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE;

ALTER TABLE `events`
  ADD `schedule_id` int DEFAULT NULL
  AFTER `access_level`;

ALTER TABLE `events` ADD FOREIGN KEY (`schedule_id`) REFERENCES `schedules` (`id`) ON DELETE RESTRICT;