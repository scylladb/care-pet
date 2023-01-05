<?php

namespace App\Core\Commands;

use App\Core\Commands\Base\AbstractCommand;
use App\Core\Database\Connector;

class MigrateCommand extends AbstractCommand
{

    public function handle(array $args): int
    {
        $this->info('Fetching Migrations...');
        $connector = new Connector(config('database'));

        $keyspaceCQL = $this->getMigrations()[0];
        $this->info('Preparing Keyspace ' . config('database.keyspace'));
        $connector->prepare($keyspaceCQL)->execute();

        $connector = $connector
            ->setKeyspace(config('database.keyspace'));

        foreach ($this->getMigrations() as $migrationFile) {
            $connector->prepare(file_get_contents($migrationFile))->execute();
            $this->info(sprintf('Migrated: %s', $migrationFile));
        }

        $this->info('Done :D');
        return self::SUCCESS;
    }

    /**
     * @return array|false
     */
    public function getMigrations()
    {
        return glob(basePath('/migrations/*.cql'));
    }
}