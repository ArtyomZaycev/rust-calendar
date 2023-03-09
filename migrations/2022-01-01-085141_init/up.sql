CREATE TABLE `users` (
	`id` int NOT NULL AUTO_INCREMENT,
	`name` varchar(255) NOT NULL,
	`email` varchar(255) NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT `UC_users_email` UNIQUE (`email`)
);

CREATE TABLE `passwords` (
	`id` int NOT NULL AUTO_INCREMENT,
	`user_id` int NOT NULL,
	`name` varchar(40) NOT NULL,
	`password` varchar(40) NOT NULL,
	`access_level` int NOT NULL,
	`edit_right` boolean NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT `UC_passwords_user_access` UNIQUE (`user_id`, `access_level`, `edit_right`),
	CONSTRAINT `UC_passwords_user_password` UNIQUE (`user_id`, `password`),
	CONSTRAINT `FK_passwords_user_id` FOREIGN KEY (`user_id`)
    REFERENCES `users`(`id`)
);

CREATE TABLE `sessions` (
	`id` int NOT NULL AUTO_INCREMENT,
	`password_id` int NOT NULL,
	`key` binary(64) NOT NULL,
	`start` timestamp NOT NULL,
	`end` timestamp NOT NULL,
	`valid` boolean NOT NULL DEFAULT true,
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_sessions_password_id` FOREIGN KEY (`password_id`)
    REFERENCES `passwords`(`id`)
);

CREATE TABLE `roles` (
	`id` int NOT NULL AUTO_INCREMENT,
	`name` varchar(40) NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT `UC_roles_name` UNIQUE (`name`)
);

CREATE TABLE `user_roles` (
	`id` int NOT NULL AUTO_INCREMENT,
	`user_id` int NOT NULL,
	`role_id` int NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT `UC_user_roles` UNIQUE (`user_id`, `role_id`),
	CONSTRAINT `FK_user_roles_user_id` FOREIGN KEY (`user_id`)
    REFERENCES `users`(`id`),
	CONSTRAINT `FK_user_roles_role_id` FOREIGN KEY (`role_id`)
    REFERENCES `roles`(`id`)
);

CREATE TABLE `event_templates` (
	`id` int NOT NULL AUTO_INCREMENT,
	`user_id` int NOT NULL,
	`access_level` int NOT NULL,
	`name` varchar(255) NOT NULL,
	`event_name` varchar(255) NOT NULL,
	`event_description` varchar(255),
	`duration` int NOT NULL COMMENT 'How long in minutes does an event last',
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_event_templates_user_id` FOREIGN KEY (`user_id`)
    REFERENCES `users`(`id`),
	CONSTRAINT `FK_event_templates_access_level_id` FOREIGN KEY (`user_id`, `access_level`)
    REFERENCES `passwords`(`user_id`, `access_level`) ON UPDATE CASCADE
);

CREATE TABLE `schedules` (
	`id` int NOT NULL AUTO_INCREMENT,
	`user_id` int NOT NULL,
	`access_level` int NOT NULL,
	`template_id` int NOT NULL,
	`name` varchar(255) NOT NULL,
	`description` varchar(255),
	`first_day` date NOT NULL,
	`last_day` date,
	`deleted` boolean NOT NULL DEFAULT false,
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_schedules_user_id` FOREIGN KEY (`user_id`)
    REFERENCES `users`(`id`),
	CONSTRAINT `FK_schedules_access_level_id` FOREIGN KEY (`user_id`, `access_level`)
    REFERENCES `passwords`(`user_id`, `access_level`) ON UPDATE CASCADE,
	CONSTRAINT `FK_schedules_template_id` FOREIGN KEY (`template_id`)
    REFERENCES `event_templates`(`id`)
);

CREATE TABLE `event_plans` (
	`id` int NOT NULL AUTO_INCREMENT,
	`schedule_id` int NOT NULL,
	`weekday` tinyint NOT NULL,
	`time` smallint NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_event_plans_schedule_id` FOREIGN KEY (`schedule_id`)
    REFERENCES `schedules`(`id`)
);

CREATE TABLE `events` (
	`id` int NOT NULL AUTO_INCREMENT,
	`user_id` int NOT NULL,
	`access_level` int NOT NULL,

	# Diesel doesn't support enums for mysql
	# Visibility
	# 0 - hide completelly (Default)
	# 1 - hide name & description
	# 2 - hide description
	# 3 - show all
	`visibility` tinyint NOT NULL,

	`name` varchar(255) NOT NULL,
	`description` varchar(255),
	`start` timestamp NOT NULL,
	`end` timestamp NOT NULL,
	`plan_id` int,
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_events_user_id` FOREIGN KEY (`user_id`)
    REFERENCES `users`(`id`),
	CONSTRAINT `FK_events_access_level_id` FOREIGN KEY (`user_id`, `access_level`)
		REFERENCES `passwords`(`user_id`, `access_level`) ON UPDATE CASCADE,
	CONSTRAINT `FK_events_plan_id` FOREIGN KEY (`plan_id`)
    REFERENCES `event_plans`(`id`)
);