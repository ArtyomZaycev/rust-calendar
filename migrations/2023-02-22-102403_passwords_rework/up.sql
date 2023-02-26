ALTER TABLE `passwords`
    ADD `name` VARCHAR(40) NOT NULL
    AFTER `user_id`;