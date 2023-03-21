<?php


if (!function_exists('basePath')) {
    function basePath(string $path = '')
    {
        $directoryPieces = explode('src', __DIR__);
        return substr($directoryPieces[0], 0, -1) . $path;
    }
}

if (!function_exists('config')) {
    /**
     * @throws \Exception
     */
    function config(string $key = '')
    {
        $config = require(basePath('/config/app.php'));

        if (empty($key)) {
            return $config;
        }

        $keys = explode('.', $key);

        foreach ($keys as $key) {
            if (!isset($config[$key])) {
                throw new Exception('key not found');
            }
            $config = $config[$key];
        }

        return $config;
    }
}

if (!function_exists('env')) {
    function env(string $envKey, string $default = '')
    {
        return $_ENV[$envKey] ?: $default;
    }
}

if (!function_exists('str_contains')) {
    function str_contains(string $haystack, string $needle): bool
    {
        return strpos($haystack, $needle) !== false;
    }
}
