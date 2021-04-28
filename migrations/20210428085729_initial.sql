-- Add migration script here
CREATE TABLE `user` (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `username` VARCHAR(100) NOT NULL COLLATE 'utf8mb4_unicode_ci',
    `password` VARCHAR(100) NOT NULL COLLATE 'utf8mb4_unicode_ci',
    PRIMARY KEY (`id`) USING BTREE
)
COLLATE='utf8mb4_unicode_ci'
ENGINE=InnoDB;

CREATE TABLE `session` (
    `user_id` INT(11) NOT NULL,
    `session` VARCHAR(100) NOT NULL COLLATE 'utf8mb4_unicode_ci',
    `data` VARCHAR(2000) NOT NULL COLLATE 'utf8mb4_unicode_ci',
    PRIMARY KEY (`user_id`) USING BTREE,
    CONSTRAINT `user_id_session` FOREIGN KEY (`user_id`) REFERENCES `shortener`.`user` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
)
COLLATE='utf8mb4_unicode_ci'
ENGINE=InnoDB;

CREATE TABLE `shortcut` (
    `id` VARCHAR(50) NOT NULL COLLATE 'utf8mb4_unicode_ci',
    `destination` VARCHAR(1000) NOT NULL COLLATE 'utf8mb4_unicode_ci',
    `datetime` DATETIME NOT NULL,
    `created_by` INT(11) NULL DEFAULT NULL,
    `delete_code` VARCHAR(50) NOT NULL COLLATE 'utf8mb4_unicode_ci',
    PRIMARY KEY (`id`) USING BTREE,
    INDEX `user_id_shortcut` (`created_by`) USING BTREE,
    CONSTRAINT `user_id_shortcut` FOREIGN KEY (`created_by`) REFERENCES `shortener`.`user` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
)
COLLATE='utf8mb4_unicode_ci'
ENGINE=InnoDB;
