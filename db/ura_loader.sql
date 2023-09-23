CREATE TABLE IF NOT EXISTS `USER` (
    `id` BIGINT(20) NOT NULL,
    `token` CHAR(36) NOT NULL,
    `permissions` INT(20) NOT NULL,
    UNIQUE (`token`),
    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `USER_MACS` (
    `mac` CHAR(17) NOT NULL,
    `id` BIGINT(20) NOT NULL,
    PRIMARY KEY (`mac`),
    FOREIGN KEY (`id`) REFERENCES USER(`id`)
);