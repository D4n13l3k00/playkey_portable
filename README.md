# PlaykeyPortable

## Портативный клиент облачного гейминга Playkey

`WORKS ONLY ON WINDOWS`

![Preview](https://i.imgur.com/314q81B.png)

## Фичи

- Прост в использовании
- Работает без браузера (но нужен токен от API (можно отловить через браузер) (мб потом добавлю авторизацию, если смогу найти решение с рекапчей) )
- Сжатие клиента во время билда (меньше вес итогового билда)
- Максимальная портативность (автораспаковка клиента на время игровой сессии и последующее его удаление)
- По идее должно работать на x32 (тут уже вопрос к [EnigmaVB](https://www.enigmaprotector.com/assets/files/enigmavb.exe))

## Билд

- Клонировать репозиторий
- Упаковать оригинальный клиент Playkey через [EnigmaVB](https://www.enigmaprotector.com/assets/files/enigmavb.exe)
- Расположить клиент в `resources/client.exe` клонированного репозитория
- `.\build.bat` для сборки на x64 и x32 или `cargo build` для билда под вашу архитектуру

## Лицензия

[![Лицензия AGPL-v3.0](https://www.gnu.org/graphics/license-logos-by-christian-candena-cc-by.png)](LICENSE)
