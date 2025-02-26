## SpecStory

## Rust CosaService Trait Implementation (2/25/2025, 11:02:37 PM)

_**User**_

このファイルのCosaServiceのメソッドでtraitを実装してください

---

_**Assistant**_

了解しました。`CosanService` のメソッドにトレイト境界を追加するには、`where` 句を使用します。以下に、`router/router.rs` ファイルの `get_user` メソッドを更新し、必要なトレイト境界を追加する方法を示します。

```rust:router/router.rs
    async fn get_user(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(user_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
        info!("Get user");
        info!(token = ?token);

        let protagonist = service.get_user(i64::try_from(user_id).unwrap()).await;

        match protagonist {
            Ok(protagonist) => Ok((http::StatusCode::OK, Json(protagonist))),
            Err(err) => {
                if err.to_string().contains("no rows") {
                    Err((
                        http::StatusCode::NOT_FOUND,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "User not found".to_string(),
                        }),
                    ))
                } else {
                    Err((
                        http::StatusCode::INTERNAL_SERVER_ERROR,
                        Json(response::ErrorResponse {
                            error: err.to_string(),
                            message: "Internal Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
    }
```

この例では、`get_user` メソッドに `where R: interface::RepositoryTrait` 句を追加しました。

すべてのメソッドに適用するには、以下のように変更します。

```rust:router/router.rs
    async fn get_user(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(user_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn create_user(
        State(service): State<CosanService<R>>,
        Json(body): Json<request::CreateUserRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::CreateUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn update_user(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Json(body): Json<request::UpdateUserRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::UpdateUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn delete_user(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(protagonist_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::DeleteUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn get_user_by_login_id_and_password(
        State(service): State<CosanService<R>>,
        Path(login_request): Path<request::GetUserRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn get_word(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::GetWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn create_word(
        State(service): State<CosanService<R>>,
        Json(body): Json<request::CreateWordRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::CreateWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn update_word(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Json(body): Json<request::UpdateWordRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::UpdateWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn delete_word(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::DeleteWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn get_user_word_by_user_id_and_word_id(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(request): Path<request::GetUserWordRequest>,
    ) -> Result<
        (http::StatusCode, Json<response::GetUserWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn get_user_word_by_user_id(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(user_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<Vec<response::GetUserWordResponse>>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn get_user_word_by_word_id(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<Vec<response::GetUserWordResponse>>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn create_user_word(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Json(body): Json<request::CreateUserWordRequest>,
    ) -> Result<
        (
            http::StatusCode,
            Json<response::CreateUserWordRelationResponse>,
        ),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }

    async fn delete_user_word(
        State(service): State<CosanService<R>>,
        Extension(token): Extension<Arc<util::auth::Token>>,
        Path(user_word_id): Path<u64>,
    ) -> Result<
        (http::StatusCode, Json<response::DeleteUserWordResponse>),
        (http::StatusCode, Json<response::ErrorResponse>),
    >
    where
        R: interface::RepositoryTrait,
    {
// ... existing code ...
    }
}
```

`AppRouter` 構造体の定義箇所と、`health_check` メソッドには既に `where R: interface::RepositoryTrait` が存在するため、変更は不要です。


---



---

