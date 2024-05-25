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
