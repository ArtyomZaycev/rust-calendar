# Diesel doesn't support enums for mysql
# Visibility
# 0 - hide completelly (Default)
# 1 - hide name & description
# 2 - hide description
# 3 - show all

ALTER TABLE `events`
  ADD `visibility` tinyint NOT NULL DEFAULT 0
  AFTER `access_level`;