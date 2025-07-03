# Cursor Monorepo

Монорепо проект с Rust бэкендом и React фронтендом, копирующий функциональность JSONPlaceholder.

## Структура проекта

```
cursor-monorepo/
├── backend/          # Rust API сервис
├── frontend/         # React приложение
├── docker-compose.yml
└── README.md
```

## Технологии

### Backend (Rust)
- **Framework**: Axum
- **Database**: PostgreSQL + Diesel ORM
- **Authentication**: JWT
- **Containerization**: Docker

### Frontend (React)
- **Framework**: React + TypeScript
- **Styling**: TailwindCSS
- **Build Tool**: Vite

## Быстрый старт

1. Клонируйте репозиторий
2. Запустите Docker контейнеры: `docker-compose up -d`
3. Установите зависимости и запустите сервисы

## API Endpoints

API копирует функциональность JSONPlaceholder:
- `GET /users` - получить всех пользователей
- `GET /users/:id` - получить пользователя по ID
- `POST /users` - создать пользователя
- `PUT /users/:id` - обновить пользователя
- `DELETE /users/:id` - удалить пользователя
- `POST /auth/login` - авторизация
- `POST /auth/register` - регистрация 