<?php

namespace App\Core\Http;

class BaseController
{
    public function responseJson($payload, int $statusCode = 200)
    {
        header('Content-type: application/json');
        http_response_code($statusCode);

        echo json_encode($payload);
    }
}