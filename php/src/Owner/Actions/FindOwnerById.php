<?php

namespace App\Owner\Actions;

use App\Core\Database\Connector;
use App\Owner\Exceptions\OwnerException;
use App\Owner\OwnerDTO;
use App\Owner\OwnerRepository;
use Exception;

class FindOwnerById
{
    /** @var OwnerRepository */
    private $repository;

    public function __construct(OwnerRepository $repository)
    {
        $this->repository = $repository;
    }

    public function handle(string $ownerId): OwnerDTO
    {
        $row = $this->repository->getById($ownerId);

        if ($row->count() == 0) {
            throw OwnerException::notFound($ownerId);
        }

        return OwnerDTO::make($row->first());
    }
}