CREATE TABLE `users` (
	`id` int NOT NULL AUTO_INCREMENT,
	`name` varchar(255) NOT NULL,
	`email` varchar(255) NOT NULL,
	`password` varchar(128) NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT `UC_users_email` UNIQUE (`email`)
);

CREATE TABLE `access_levels` (
	`id` int NOT NULL AUTO_INCREMENT,
	`user_id` int NOT NULL,
	`name` varchar(40) NOT NULL,
	`level` int NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT `UC_access_levels_user_level` UNIQUE (`user_id`, `level`),
	CONSTRAINT `FK_passwords_user_id` FOREIGN KEY (`user_id`)
    	REFERENCES `users`(`id`)
);

CREATE TABLE `sessions` (
	`id` int NOT NULL AUTO_INCREMENT,
	`user_id` int NOT NULL,
	`key` binary(64) NOT NULL,
	`start` timestamp NOT NULL,
	`end` timestamp NOT NULL,
	`valid` boolean NOT NULL DEFAULT true,
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_sessions_user_id` FOREIGN KEY (`user_id`)
    	REFERENCES `users`(`id`)
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
	`event_description` text,
	`duration` int NOT NULL COMMENT 'How long in minutes does an event last',
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_event_templates_user_id` FOREIGN KEY (`user_id`)
    	REFERENCES `users`(`id`),
	CONSTRAINT `FK_event_templates_access_level_id` FOREIGN KEY (`user_id`, `access_level`)
    	REFERENCES `access_levels`(`user_id`, `level`) ON UPDATE CASCADE
);

CREATE TABLE `schedules` (
	`id` int NOT NULL AUTO_INCREMENT,
	`user_id` int NOT NULL,
	`access_level` int NOT NULL,
	`template_id` int NOT NULL,
	`name` varchar(255) NOT NULL,
	`description` text,
	`first_day` date NOT NULL,
	`last_day` date,
	`deleted` boolean NOT NULL DEFAULT false,
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_schedules_user_id` FOREIGN KEY (`user_id`)
    	REFERENCES `users`(`id`),
	CONSTRAINT `FK_schedules_access_level_id` FOREIGN KEY (`user_id`, `access_level`)
    	REFERENCES `access_levels`(`user_id`, `level`) ON UPDATE CASCADE,
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

	-- Diesel doesn't support enums for mysql
	`visibility` tinyint NOT NULL COMMENT '0 - hide completelly (Default); 1 - hide name & description; 2 - hide description; 3 - show all',

	`name` varchar(255) NOT NULL,
	`description` text,
	`start` timestamp NOT NULL,
	`end` timestamp NOT NULL,
	`plan_id` int,
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_events_user_id` FOREIGN KEY (`user_id`)
    	REFERENCES `users`(`id`),
	CONSTRAINT `FK_events_access_level_id` FOREIGN KEY (`user_id`, `access_level`)
		REFERENCES `access_levels`(`user_id`, `level`) ON UPDATE CASCADE,
	CONSTRAINT `FK_events_plan_id` FOREIGN KEY (`plan_id`)
    	REFERENCES `event_plans`(`id`)
);

CREATE TABLE `permissions` (
	`id` int NOT NULL AUTO_INCREMENT,

    `user_id` int NOT NULL,
    `access_level` int NOT NULL,

    `allow_share` boolean NOT NULL,
    
    `access_levels_create` boolean NOT NULL,
    `access_levels_read` boolean NOT NULL,
    `access_levels_update` boolean NOT NULL,
    `access_levels_delete` boolean NOT NULL,
    
    `events_create` boolean NOT NULL,
    `events_read` boolean NOT NULL,
    `events_update` boolean NOT NULL,
    `events_delete` boolean NOT NULL,
    
    `event_templates_create` boolean NOT NULL,
    `event_templates_read` boolean NOT NULL,
    `event_templates_update` boolean NOT NULL,
    `event_templates_delete` boolean NOT NULL,
    
    `schedules_create` boolean NOT NULL,
    `schedules_read` boolean NOT NULL,
    `schedules_update` boolean NOT NULL,
    `schedules_delete` boolean NOT NULL,
    
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_permissions_access_level_id` FOREIGN KEY (`user_id`, `access_level`)
    	REFERENCES `access_levels`(`user_id`, `level`) ON UPDATE CASCADE
);

CREATE TABLE `granted_permissions` (
	`id` int NOT NULL AUTO_INCREMENT,
	`giver_user_id` int NOT NULL,
	`receiver_user_id` int NOT NULL,
    `permissions_id` int NOT NULL,
	PRIMARY KEY (`id`),
	CONSTRAINT `FK_granted_permissions_giver_user_id` FOREIGN KEY (`giver_user_id`)
    	REFERENCES `users`(`id`),
	CONSTRAINT `FK_granted_permissions_receiver_user_id` FOREIGN KEY (`receiver_user_id`)
    	REFERENCES `users`(`id`),
	CONSTRAINT `FK_granted_permissions_permissions_id` FOREIGN KEY (`permissions_id`)
    	REFERENCES `permissions`(`id`)
);
