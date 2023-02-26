ALTER TABLE `events`
    DROP FOREIGN KEY `events_ibfk_2`;
ALTER TABLE `events`
    DROP COLUMN `schedule_id`;

DROP TABLE IF EXISTS `schedules`;