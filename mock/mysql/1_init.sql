CREATE TABLE IF NOT EXISTS `users`(
    `id` VARCHAR(20) NOT NULL,
    `name` VARCHAR(50) NOT NULL,
    `created_at` TIMESTAMP DEFAULT NOW() NOT NULL,
    PRIMARY KEY (`id`)
);