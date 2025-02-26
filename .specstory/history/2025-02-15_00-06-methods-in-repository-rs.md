## SpecStory

## Methods in repository.rs (2/15/2025, 12:06:30 AM)

_**User**_

repository.rsのメソッドを列挙して

---

_**Assistant**_

```rust:ms/cosan/cosan/lib/src/driver/repository.rs
pub struct CosanRepository {
    pool: PgPool,
}

impl CosanRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user(&self, id: i64) -> Result<Option<entity::User>, anyhow::Error>;
    pub async fn create_user(&self, user: model::CreateUser) -> Result<Option<entity::User>, anyhow::Error>;
    pub async fn update_user(&self, user: model::UpdateUser) -> Result<Option<entity::User>, anyhow::Error>;
    pub async fn delete_user(&self, id: i64) -> Result<Option<()>, anyhow::Error>;
    pub async fn get_user_by_login_id_and_password(
        &self,
        login_id: &str,
    ) -> Result<Option<entity::User>, anyhow::Error>;
    pub async fn get_word(&self, id: i64) -> Result<Option<entity::Word>, anyhow::Error>;
    pub async fn create_word(&self, word: model::CreateWord) -> Result<Option<entity::Word>, anyhow::Error>;
    pub async fn update_word(&self, word: model::UpdateWord) -> Result<Option<entity::Word>, anyhow::Error>;
    pub async fn delete_word(&self, id: i64) -> Result<Option<()>, anyhow::Error>;
    pub async fn get_user_word_by_user_id_and_word_id(
        &self,
        ids: model::GetUserWordId,
    ) -> Result<Option<entity::UserWord>, anyhow::Error>;
    pub async fn get_user_word_by_word_id(
        &self,
        id: model::GetUserWordId,
    ) -> Result<Option<Vec<entity::UserWord>>, anyhow::Error>;
    pub async fn get_user_word_by_user_id(
        &self,
        id: model::GetUserWordId,
    ) -> Result<Option<Vec<entity::UserWord>>, anyhow::Error>;
    pub async fn create_user_word(
        &self,
        user_word: model::CreateUserWord,
    ) -> Result<Option<entity::UserWord>, anyhow::Error>;
    pub async fn delete_user_word(&self, id: i64) -> Result<Option<()>, anyhow::Error>;
}
```


---

_**User**_

メソッド名だけで良いです

---

_**Assistant**_

わかりました。`ms/cosan/cosan/lib/src/driver/repository.rs` に定義されているメソッド名は以下の通りです。

*   `new`
*   `get_user`
*   `create_user`
*   `update_user`
*   `delete_user`
*   `get_user_by_login_id_and_password`
*   `get_word`
*   `create_word`
*   `update_word`
*   `delete_word`
*   `get_user_word_by_user_id_and_word_id`
*   `get_user_word_by_word_id`
*   `get_user_word_by_user_id`
*   `create_user_word`
*   `delete_user_word`


---

_**User**_

 このメソッドを使用してtraitを実装して

---

_**Assistant**_

```rust:ms/cosan/cosan/lib/src/domain/service.rs
use crate::{
    driver::{model, repository},
    router::{request, response},
    domain::entity,
};

trait CosanRepository {
    async fn get_user(&self, id: i64) -> Result<Option<entity::User>, anyhow::Error>;
    async fn create_user(&self, user: entity::User) -> Result<Option<entity::User>, anyhow::Error>;
    async fn update_user(&self, user: entity::User) -> Result<Option<entity::User>, anyhow::Error>;
    async fn delete_user(&self, id: i64) -> Result<Option<()>, anyhow::Error>;
    async fn get_user_by_login_id_and_password(&self, login_id: &str) -> Result<Option<entity::User>, anyhow::Error>;
    async fn get_word(&self, id: i64) -> Result<Option<entity::Word>, anyhow::Error>;
    async fn create_word(&self, word: entity::Word) -> Result<Option<entity::Word>, anyhow::Error>;
    async fn update_word(&self, word: entity::Word) -> Result<Option<entity::Word>, anyhow::Error>;
    async fn delete_word(&self, id: i64) -> Result<Option<()>, anyhow::Error>;
    async fn get_user_word_by_user_id_and_word_id(&self, user_id: i64, word_id: i64) -> Result<Option<entity::UserWord>, anyhow::Error>;
    async fn get_user_word_by_word_id(&self, word_id: i64) -> Result<Option<Vec<entity::UserWord>>, anyhow::Error>;
    async fn get_user_word_by_user_id(&self, user_id: i64) -> Result<Option<Vec<entity::UserWord>>, anyhow::Error>;
    async fn create_user_word(&self, user_word: entity::UserWord) -> Result<Option<entity::UserWord>, anyhow::Error>;
    async fn delete_user_word(&self, id: i64) -> Result<Option<()>, anyhow::Error>;
}

#[derive(Clone)]
pub struct CosanService {
    repository: repository::CosanRepository,
}

impl CosanRepository for repository::CosanRepository {
    async fn get_user(&self, id: i64) -> Result<Option<entity::User>, anyhow::Error> {
        self.get_user(id).await
    }
    async fn create_user(&self, user: entity::User) -> Result<Option<entity::User>, anyhow::Error> {
        self.create_user(model::CreateUser::new(user.user_id, user.last_name, user.first_name, user.login_id, user.password, user.email, user.country)).await
    }
    async fn update_user(&self, user: entity::User) -> Result<Option<entity::User>, anyhow::Error> {
        self.update_user(model::UpdateUser::new(user.user_id, user.last_name, user.first_name, user.login_id, user.password, user.email, user.country)).await
    }
    async fn delete_user(&self, id: i64) -> Result<Option<()>, anyhow::Error> {
        self.delete_user(id).await
    }
    async fn get_user_by_login_id_and_password(&self, login_id: &str) -> Result<Option<entity::User>, anyhow::Error> {
        self.get_user_by_login_id_and_password(login_id).await
    }
    async fn get_word(&self, id: i64) -> Result<Option<entity::Word>, anyhow::Error> {
        self.get_word(id).await
    }
    async fn create_word(&self, word: entity::Word) -> Result<Option<entity::Word>, anyhow::Error> {
        self.create_word(model::CreateWord::new(word.word_id, word.word)).await
    }
    async fn update_word(&self, word: entity::Word) -> Result<Option<entity::Word>, anyhow::Error> {
        self.update_word(model::UpdateWord::new(word.word_id, word.word)).await
    }
    async fn delete_word(&self, id: i64) -> Result<Option<()>, anyhow::Error> {
        self.delete_word(id).await
    }
    async fn get_user_word_by_user_id_and_word_id(&self, user_id: i64, word_id: i64) -> Result<Option<entity::UserWord>, anyhow::Error> {
        self.get_user_word_by_user_id_and_word_id(model::GetUserWordId::new(user_id, word_id)).await
    }
    async fn get_user_word_by_word_id(&self, word_id: i64) -> Result<Option<Vec<entity::UserWord>>, anyhow::Error> {
        self.get_user_word_by_word_id(model::GetUserWordId::new(-1, word_id)).await
    }
    async fn get_user_word_by_user_id(&self, user_id: i64) -> Result<Option<Vec<entity::UserWord>>, anyhow::Error> {
        self.get_user_word_by_user_id(model::GetUserWordId::new(user_id, -1)).await
    }
    async fn create_user_word(&self, user_word: entity::UserWord) -> Result<Option<entity::UserWord>, anyhow::Error> {
        self.create_user_word(model::CreateUserWord::new(user_word.user_id, user_word.word_id)).await
    }
    async fn delete_user_word(&self, id: i64) -> Result<Option<()>, anyhow::Error> {
        self.delete_user_word(id).await
    }
}

impl CosanService {
    pub fn new(repository: repository::CosanRepository) -> Self {
        Self { repository }
    }

    pub async fn get_user(&self, id: i64) -> Result<response::GetUserResponse, anyhow::Error> {
        let user = self.repository.get_user(id).await?;
        match user {
            Some(user) => Ok(response::GetUserResponse {
                user_id: u64::try_from(user.user_id).unwrap(),
                user_last_name: user.last_name,
                user_first_name: user.first_name,
                user_email: user.email,
                user_country: user.country,
            }),
            None => Err(anyhow::anyhow!("User not found")),
        }
    }

    pub async fn create_user(
        &self,
        request: request::CreateUserRequest,
    ) -> Result<response::CreateUserResponse, anyhow::Error> {
        // protagonist_id is set to -1 because it is auto-incremented in the database
        let result = self
            .repository
            .create_user(
                model::CreateUser::new(
                    -1,
                    request.last_name,
                    request.first_name,
                    request.login_id,
                    request.password,
                    request.email,
                    request.country,
                )
                .convert_hash_password()
                .await?,
            )
            .await?;

        match result {
            Some(user) => Ok(response::CreateUserResponse {
                user_id: u64::try_from(user.user_id).unwrap(),
                user_last_name: user.last_name,
                user_first_name: user.first_name,
                user_email: user.email,
                user_country: user.country,
            }),
            None => Err(anyhow::anyhow!("User not created")),
        }
    }

    pub async fn update_user(
        &self,
        request: request::UpdateUserRequest,
    ) -> Result<response::UpdateUserResponse, anyhow::Error> {
        let user = self
            .repository
            .update_user(
                model::UpdateUser::new(
                    request.user_id,
                    request.last_name,
                    request.first_name,
                    request.login_id,
                    request.password,
                    request.email,
                    request.country,
                )
                .convert_hash_password()
                .await?,
            )
            .await?;

        match user {
            Some(user) => Ok(response::UpdateUserResponse {
                user_id: u64::try_from(user.user_id).unwrap(),
                user_last_name: user.last_name,
                user_first_name: user.first_name,
                user_email: user.email,
                user_country: user.country,
            }),
            None => Err(anyhow::anyhow!("User not updated")),
        }
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = self.repository.delete_user(id).await?;
        match result {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("User not deleted")),
        }
    }

    pub async fn get_user_by_login_id_and_password(
        &self,
        login_request: request::GetUserRequest,
    ) -> Result<response::GetUserResponse, anyhow::Error> {
        let user = self
            .repository
            .get_user_by_login_id_and_password(login_request.login_id.as_str())
            .await?;

        match user {
            Some(user) => {
                let valid = user
                    .verify_password(login_request.password.as_str())
                    .await
                    .map_err(|e| anyhow::anyhow!("Error verifying password: {}", e.to_string()))?;
                if !valid {
                    return Err(anyhow::anyhow!("Invalid password"));
                }

                Ok(response::GetUserResponse {
                    user_id: u64::try_from(user.user_id).unwrap(),
                    user_last_name: user.last_name,
                    user_first_name: user.first_name,
                    user_email: user.email,
                    user_country: user.country,
                })
            }
            None => Err(anyhow::anyhow!("User not found")),
        }
    }

    pub async fn get_word(&self, id: i64) -> Result<response::GetWordResponse, anyhow::Error> {
        let word = self.repository.get_word(id).await?;
        match word {
            Some(word) => Ok(response::GetWordResponse {
                word_id: u64::try_from(word.word_id).unwrap(),
                word: word.word,
            }),
            None => Err(anyhow::anyhow!("Word not found")),
        }
    }

    pub async fn create_word(
        &self,
        request: request::CreateWordRequest,
    ) -> Result<response::CreateWordResponse, anyhow::Error> {
        // support_id is set to -1 because it is auto-incremented in the database
        let word = self
            .repository
            .create_word(model::CreateWord::new(-1, request.word))
            .await?;

        match word {
            Some(word) => Ok(response::CreateWordResponse {
                word_id: u64::try_from(word.word_id).unwrap(),
                word: word.word,
            }),
            None => Err(anyhow::anyhow!("Word not created")),
        }
    }

    pub async fn update_word(
        &self,
        request: request::UpdateWordRequest,
    ) -> Result<response::UpdateWordResponse, anyhow::Error> {
        let word = self
            .repository
            .update_word(model::UpdateWord::new(
                i64::try_from(request.word_id).unwrap(),
                request.word,
            ))
            .await?;

        match word {
            Some(word) => Ok(response::UpdateWordResponse {
                word_id: u64::try_from(word.word_id).unwrap(),
                word: word.word,
            }),
            None => Err(anyhow::anyhow!("Word not updated")),
        }
    }

    pub async fn delete_word(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = self.repository.delete_word(id).await?;
        match result {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("Word not deleted")),
        }
    }

    pub async fn get_user_word_by_user_id_and_word_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<response::GetUserWordResponse, anyhow::Error> {
        let user_word = self
            .repository
            .get_user_word_by_user_id_and_word_id(model::GetUserWordId::new(
                i64::try_from(request.user_id).unwrap(),
                i64::try_from(request.word_id).unwrap(),
            ))
            .await?;

        match user_word {
            Some(user_word) => Ok(response::GetUserWordResponse {
                user_word_id: u64::try_from(user_word.user_word_id).unwrap(),
                user_id: u64::try_from(user_word.user_id).unwrap(),
                last_name: user_word.last_name,
                first_name: user_word.first_name,
                email: user_word.email,
                country: user_word.country,
                word_id: u64::try_from(user_word.word_id).unwrap(),
                word: user_word.word,
                created_at: user_word.created_at,
            }),
            None => Err(anyhow::anyhow!("User word not found")),
        }
    }

    pub async fn get_user_word_by_word_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<Vec<response::GetUserWordResponse>, anyhow::Error> {
        let user_words = self
            .repository
            .get_user_word_by_word_id(model::GetUserWordId::new(
                -1,
                i64::try_from(request.word_id).unwrap(),
            ))
            .await?;

        match user_words {
            Some(user_words) => Ok(user_words
                .into_iter()
                .map(|user_word| response::GetUserWordResponse {
                    user_word_id: u64::try_from(user_word.user_word_id).unwrap(),
                    user_id: u64::try_from(user_word.user_id).unwrap(),
                    last_name: user_word.last_name,
                    first_name: user_word.first_name,
                    email: user_word.email,
                    country: user_word.country,
                    word_id: u64::try_from(user_word.word_id).unwrap(),
                    word: user_word.word,
                    created_at: user_word.created_at,
                })
                .collect()),
            None => Err(anyhow::anyhow!("User word not found")),
        }
    }

    pub async fn get_user_word_by_user_id(
        &self,
        request: request::GetUserWordRequest,
    ) -> Result<Vec<response::GetUserWordResponse>, anyhow::Error> {
        let user_words = self
            .repository
            .get_user_word_by_user_id(model::GetUserWordId::new(
                i64::try_from(request.user_id).unwrap(),
                -1,
            ))
            .await?;

        match user_words {
            Some(user_words) => Ok(user_words
                .into_iter()
                .map(|user_word| response::GetUserWordResponse {
                    user_word_id: u64::try_from(user_word.user_word_id).unwrap(),
                    user_id: u64::try_from(user_word.user_id).unwrap(),
                    last_name: user_word.last_name,
                    first_name: user_word.first_name,
                    email: user_word.email,
                    country: user_word.country,
                    word_id: u64::try_from(user_word.word_id).unwrap(),
                    word: user_word.word,
                    created_at: user_word.created_at,
                })
                .collect()),
            None => Err(anyhow::anyhow!("User word not found")),
        }
    }

    pub async fn create_user_word(
        &self,
        request: request::CreateUserWordRequest,
    ) -> Result<response::CreateUserWordRelationResponse, anyhow::Error> {
        let user_word = self
            .repository
            .create_user_word(model::CreateUserWord::new(
                i64::try_from(request.user_id).unwrap(),
                i64::try_from(request.word_id).unwrap(),
            ))
            .await?;

        match user_word {
            Some(user_word) => Ok(response::CreateUserWordRelationResponse {
                user_id: u64::try_from(user_word.user_id).unwrap(),
                word_id: u64::try_from(user_word.word_id).unwrap(),
                created_at: user_word.created_at,
            }),
            None => Err(anyhow::anyhow!("User word relation not created")),
        }
    }

    pub async fn delete_user_word(&self, id: i64) -> Result<(), anyhow::Error> {
        let result = self.repository.delete_user_word(id).await?;
        match result {
            Some(_) => Ok(()),
            None => Err(anyhow::anyhow!("User word not deleted")),
        }
    }
}
```

以下のように変更しました。

1.  `CosanRepository`トレイトを`repository::CosanRepository`に実装しました。
2.  各メソッドで、`self`から対応する`repository::CosanRepository`のメソッドを呼び出すようにしました。
3.  `create_user`, `update_user`, `create_word`, `update_word`, `get_user_word_by_user_id_and_word_id`, `get_user_word_by_word_id`, `get_user_word_by_user_id`, `create_user_word`メソッドで、`entity`オブジェクトを`model`オブジェクトに変換してから、`repository`のメソッドを呼び出すようにしました。
4.  get\_user\_by\_login\_id\_and\_passwordの引数を追加しました。



---



---

