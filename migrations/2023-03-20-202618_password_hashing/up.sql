ALTER TABLE `passwords` MODIFY `password` NOT NULL VARCHAR(128);

UPDATE `passwords`
SET `password`=SHA2(`password`, 512);