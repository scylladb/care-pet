<?php

namespace App\Core\Commands;

use App\Core\Commands\Base\AbstractCommand;
use App\Core\Database\Connector;

final class MigrateCommand extends AbstractCommand
{
    /**
     * @var \App\Core\Database\Connector
     */
    private $connector;

    public function __construct(Connector $connector)
    {
        $this->connector = $connector;
    }

    public function __invoke(array $args): int
    {
        $this->info('Fetching Migrations...');
        $this->info('Preparing Keyspace ' . config('database.keyspace'));

        $keyspaceCQL = $this->getMigrations()[0];
        $this->connector->prepare($keyspaceCQL)->execute();

        $this->connector = $this->connector
            ->setKeyspace(config('database.keyspace'));

        foreach ($this->getMigrations() as $migrationFile) {
            $this->connector->prepare(file_get_contents($migrationFile))->execute();
            $this->info(sprintf('Migrated: %s', $migrationFile));
        }

        $this->info('Done :D');
        return self::SUCCESS;
    }

    /** @return array<int, string> */
    public function getMigrations(): array
    {
        return glob(basePath('/migrations/*.cql'));
    }
}
