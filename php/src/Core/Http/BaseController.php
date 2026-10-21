<?php

namespace App\Core\Http;

use Cassandra\Uuid;
use InvalidArgumentException;

class BaseController
{
    private const UUID_PATTERN = '/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i';

    // Kernel returns the exception code (HTTP 400) and the message as JSON.
    protected function uuid(string $value, string $name): Uuid
    {
        if (preg_match(self::UUID_PATTERN, $value) !== 1) {
            throw new InvalidArgumentException(sprintf('The %s must be a UUID.', $name), 400);
        }

        return new Uuid($value);
    }

    public function responseJson($payload, int $statusCode = 200): void
    {
        header('Content-type: application/json');
        http_response_code($statusCode);

        echo json_encode($payload);
    }
}
