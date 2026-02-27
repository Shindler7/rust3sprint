# API Schema — Blog Service

Сервер предоставляет два сервиса API: `http` и `gRPC`.

## Ресурсы

### Авторизация (auth)

#### HTTP API

| Метод  | Эндпоинт             | Описание                                            |
|--------|----------------------|-----------------------------------------------------|
| `POST` | `/api/auth/register` | Регистрация пользователя                            |
| `POST` | `/api/auth/login`    | Авторизация пользователя (ответ содержит JWT-токен) |

Сервер имеет ограничения по формату допустимых логинов и паролей.

Логины нечувствительные к регистру и уникальны.

**Примеры запросов**

- регистрация пользователя

```shell
curl --location 'http://localhost:8080/api/auth/register' \
--header 'Content-Type: application/json' \
--data-raw '{
"username": "user",
"email": "email@email.com",
"password": "My_secret_Password"
}'
```

- авторизация пользователя

```shell
curl --location 'http://localhost:8080/api/auth/login' \
--header 'Content-Type: application/json' \
--data-raw '{
"username": "user",
"password": "My_secret_Password"
}'
```

#### gRPC

**Методы**

`GRPC localhost:8080/blog.BlogService`

* Register(RegisterRequest) -> AuthResponse
* Login(LoginRequest) -> AuthResponse

**Protobuf‑определение**

```text
// Данные о пользователе.
message User {
  int64 id = 1;
  string username = 2;
  string email = 3;
}

// Запрос на регистрацию пользователя.
message RegisterRequest {
  string username = 1;
  string email = 2;
  string password = 3;
}

// Авторизация пользователя.
message LoginRequest {
  string username = 1;
  string password = 2;
}

// Успешный ответ при регистрации и авторизации.
message AuthResponse {
  User user = 1;
  string token = 2;
}
```

### Публикации (posts)

#### HTTP API

| Метод     | Эндпоинт               | Описание                                            |
|-----------|------------------------|-----------------------------------------------------|
| `GET`     | `/api/posts/`          | Получение списка последних публикаций, с пагинацией |
| `GET`     | `/api/posts/{post_id}` | Получение публикации по её id                       |
| `POST`*   | `/api/posts`           | Создание публикации                                 |
| `PUT`*    | `/api/posts/{post_id}` | Обновление публикации по её id                      |
| `DELETE`* | `api/posts/{post_id}`  | Удаление публикации по её id                        |

\* — требуется JWT-токен (авторизация) для операции.

**Примеры запросов**

- получение списка публикаций

```shell
curl --location 'http://localhost:8080/api/posts?limit=10&offset=0'
```

- получение публикации по id

```shell
curl --location 'http://localhost:8080/api/posts/4'
```

- создание публикации

```shell
curl --location 'http://localhost:8080/api/posts' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer eyJ0eXA...' \
--data '{
    "title": "Житель открыл портал в параллельный мир",
    "content": "При разогреве пиццы образовался хронодырный коллапс."
}'
```

- обновление (изменение) публикации

```shell
curl --location --request PUT 'http://localhost:8080/api/posts/1' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer eyJ0eXA...' \
--data '{
    "title": "Сенсация: щекотка пяток признана видом спорта",
    "content": "В столице прошёл первый чемпионат по профессиональной щекотке пяток."
}'
```

- удаление публикации

```shell
curl --location --request DELETE 'http://localhost:8080/api/posts/3' \
--header 'Authorization: Bearer eyJ0eXA...'
```

#### gRPC

**Методы**

`GRPC localhost:8080/blog.BlogService`

* CreatePost(CreatePostRequest) → PostResponse
* GetPost(GetPostRequest) → PostResponse
* UpdatePost(UpdatePostRequest) → PostResponse
* DeletePost(DeletePostRequest) → DeletePostResponse
* ListPosts(ListPostsRequest) → ListPostsResponse

**Protobuf‑определение**

```text
// Данные о публикации (посте).
message Post {
  int64 id = 1;
  int64 author_id = 2;
  string title = 3;
  string content = 4;
  int64 created_at = 5;
  optional int64 updated_at = 6;
}

// Получить отдельный пост.
message GetPostRequest {
  int64 id = 1;
}

// Создание публикации (поста).
message CreatePostRequest {
  string title = 1;
  string content = 2;
}

// Обновление публикации (поста).
message UpdatePostRequest {
  int64 id = 1;
  optional string title = 2;
  optional string content = 3;
}

// Успешный ответ при взаимодействии с постами.
message PostResponse {
  Post post = 1;
}

// Удалить публикацию.
message DeletePostRequest {
  int64 id = 1;
}

// Успешный ответ при удалении публикации.
message DeletePostResponse {
  bool success = 1;
}

// Запрос на предоставление списка публикаций.
message ListPostsRequest {
  int32 limit = 1;
  int32 offset = 2;
}

// Успешный ответ на запрос списка публикаций.
message ListPostsResponse {
  repeated Post posts = 1;
  int32 total = 2;
  int32 limit = 3;
  int32 offset = 4;
}
```
