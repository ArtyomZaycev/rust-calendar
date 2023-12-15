CREATE FUNCTION events_amount_past_month (
    uid INT
) RETURNS INT
BEGIN
	DECLARE cnt INT;
  SELECT COUNT(*) INTO cnt
  FROM `events`
  WHERE `user_id`=uid AND
  		`start` >= DATE_SUB(CURRENT_TIMESTAMP(), INTERVAL 1 MONTH) AND
		`end` <= CURRENT_TIMESTAMP();
	RETURN cnt;
END;

CREATE FUNCTION is_user_active (
    uid INT
) RETURNS BOOLEAN
BEGIN
	DECLARE act BOOLEAN;
  SELECT IF(COUNT(`sessions`.`id`) > 0, TRUE, FALSE) INTO act
  FROM `sessions`
  INNER JOIN `passwords` ON `sessions`.`password_id`=`passwords`.`id` AND `user_id`=uid
  WHERE `sessions`.`start` >= DATE_SUB(CURRENT_TIMESTAMP(), INTERVAL 1 MONTH);
	RETURN act;
END;
