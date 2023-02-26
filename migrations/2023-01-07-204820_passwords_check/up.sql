ALTER TABLE `passwords`
ADD CONSTRAINT chk_passwords_access_level CHECK (`access_level`>=0 AND `access_level`<=1000);