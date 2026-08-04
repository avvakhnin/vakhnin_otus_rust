

Запуск модульных тестова

    cargo run

Проверки качества кода
    
    cargo clippy
    cargo fmt --check

Для более удобнного прокона примеров можно установить утилиту just

    cargo install just

Запуск всех примеров

    just run-ex

(см Justfile)

Домашнее задание 1. Заготовка для библиотеки "Умный дом"


включение/выключение умной розетки.

Запуск примера использования

    cargo run --example smart_home_report

или

    just run-1

Объяснение примера использования в файле

    ./examples/smart_home_report.rs

Домашнее задание 2. 

Добавление/удаление устройств

    cargo run --example smart_home_configurationш

или

    just run-2

Обработка ошибок при доступе к устройствам
    
    cargo run --example smart_home_tool_getter

или

    just run-3
