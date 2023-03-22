ALTER TABLE `passwords` MODIFY `password` VARCHAR(128) NOT NULL;

UPDATE `passwords`
SET `password`=SHA2(`password`, 512);