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

### Предварительные требования
- Docker и Docker Compose
- Rust 1.75+ (для разработки)
- Node.js 20+ (для разработки)

### Запуск с Docker Compose
```bash
# Клонируйте репозиторий
git clone https://github.com/InnoMikitaKutas/cursor-monorepo.git
cd cursor-monorepo

# Запустите все сервисы
docker-compose up -d

# Проверьте статус сервисов
docker-compose ps
```

### Локальная разработка

#### Backend (Rust)
```bash
cd backend

# Установите Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Настройте переменные окружения
export DATABASE_URL=postgres://cursor_user:cursor_password@localhost:5432/cursor_db
export JWT_SECRET=your-super-secret-jwt-key-change-in-production

# Запустите миграции
diesel migration run

# Запустите сервер
cargo run
```

#### Frontend (React)
```bash
cd frontend

# Установите зависимости
npm install

# Запустите dev сервер
npm run dev
```

### Тестирование
```bash
# Backend тесты
cd backend
cargo test

# Frontend тесты
cd frontend
npm test
```

## API Endpoints

API копирует функциональность JSONPlaceholder:
- `GET /users` - получить всех пользователей
- `GET /users/:id` - получить пользователя по ID
- `POST /users` - создать пользователя
- `PUT /users/:id` - обновить пользователя
- `DELETE /users/:id` - удалить пользователя
- `POST /auth/login` - авторизация
- `POST /auth/register` - регистрация

## Тестовые данные

После запуска миграций база данных заполняется тестовыми пользователями из JSONPlaceholder:

### Тестовые пользователи для авторизации:
- **Email**: `test@example.com`, **Пароль**: `password123`
- **Email**: `admin@example.com`, **Пароль**: `password123`

### Демо пользователи (10 записей):
- Leanne Graham (@Bret)
- Ervin Howell (@Antonette)
- Clementine Bauch (@Samantha)
- И другие...

Каждый пользователь имеет полную информацию: адрес с координатами и информацию о компании. 