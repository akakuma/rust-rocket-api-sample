-- Your SQL goes here
CREATE TABLE
IF NOT EXISTS `rust_db`.`users`
(
  `id` VARCHAR(40) NOT NULL,
  `name` VARCHAR(50) NOT NULL,
  `email` VARCHAR(50) NOT NULL,
  PRIMARY KEY (`id`)
)
ENGINE = InnoDB;
