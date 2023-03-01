ALTER TABLE `events`
    DROP FOREIGN KEY `events_ibfk_2`;
ALTER TABLE `events`
    DROP COLUMN `plan_id`;

DROP TABLE `event_plans`;
DROP TABLE `schedules`;
DROP TABLE `event_templates`;