## SpecStory

## Resolving Rust Async Trait Bound Error (2/26/2025, 8:25:10 AM)

_**User**_

For the code present, we get this error:
```
the trait bound `impl Future<Output = UserWordRepository>: UserWordRepositoryTrait` is not satisfied
the trait `UserWordRepositoryTrait` is implemented for `UserWordRepository`
```
How can I resolve this? If you propose a fix, please make it concise.

---

_**Assistant**_

The error occurs because `new_database` is an async function, and it returns a `Future<Output = Pool<Postgres>>` instead of `Pool<Postgres>`. The repository constructors expect a `Pool<Postgres>` to be passed in. You need to await the result of calling `new_database`.

```rust:src/bin/api.rs
    let cosan_service = CosanService::new(
        repository::UserRepository::new(new_database(&env.cosan_pg_url).await),
        repository::WordRepository::new(new_database(&env.cosan_pg_url).await),
        repository::UserWordRepository::new(new_database(&env.cosan_pg_url).await),
    );

    let router = AppRouter::new(cosan_service, env.secret_key);
    router.serve().await.unwrap();
}
```

Additionally, the `AppRouter::new` function expects an `Arc<CosanService<_,_,_>>`, but it is receiving a `CosanService`. The `serve` method is also not found, and should be called on the result of `AppRouter::new`, not assigned to a variable.

```rust:src/bin/api.rs
    use std::sync::Arc;

    // ...

    let cosan_service = CosanService::new(
        repository::UserRepository::new(new_database(&env.cosan_pg_url).await),
        repository::WordRepository::new(new_database(&env.cosan_pg_url).await),
        repository::UserWordRepository::new(new_database(&env.cosan_pg_url).await),
    );

    AppRouter::new(Arc::new(cosan_service), env.secret_key)
        .serve()
        .await
        .unwrap();
}
```


---

_**User**_

For the code present, we get this error:
```
no function or associated item named `new` found for struct `UserWordRepository` in the current scope
items from traits can only be used if the trait is in scope
```
How can I resolve this? If you propose a fix, please make it concise.

---

_**Assistant**_

The error message `no function or associated item named \`new\` found for struct \`UserWordRepository\` in the current scope` indicates that the `new` function is not defined directly on the `UserWordRepository` struct, or that a necessary trait providing it is not in scope. You also have this issue with `UserRepository` and `WordRepository`.

Looking at your `lib/src/driver/repository.rs` file will likely show that the `new` method is implemented within a trait, and you need to import that trait to bring the method into scope.

Assuming your repository traits in `lib/src/driver/repository.rs` are named something like `UserRepositoryTrait`, `WordRepositoryTrait`, and `UserWordRepositoryTrait`, you would need to import them into your `src/bin/api.rs` file.

```rust:src/bin/api.rs
use lib::{
    domain::service::CosanService,
    driver::{database::new_database, repository},
    router::router::AppRouter,
    util::slog::new_logger,
};
use repository::{UserRepositoryTrait, WordRepositoryTrait, UserWordRepositoryTrait};
```

This brings the necessary traits into scope, allowing you to use the `new` function.


---

_**User**_

For the code present, we get this error:
```
mismatched types
expected `Pool<Postgres>`, found future
```
How can I resolve this? If you propose a fix, please make it concise.

---

_**Assistant**_

You're attempting to call `.close()` on `pg_pool`, which is of type `Pool<Postgres>`, and then pass the result to the repository constructors. However, `close()` returns a `Future`, not the `Pool` itself. The repositories need the `Pool` to execute queries. You should *not* close the pool here. Instead, pass the `pg_pool` directly to the repository constructors. The pool will be closed when it's dropped, which will happen when `main` returns (or when the `Arc` is no longer in use, if you were to clone it).

```rust:src/bin/api.rs
    let pg_pool = new_database(&env.cosan_pg_url).await.unwrap();
    let cosan_service = CosanService::new(
        repository::UserRepository::new(pg_pool.clone()),
        repository::WordRepository::new(pg_pool.clone()),
        repository::UserWordRepository::new(pg_pool.clone()),
    );

    let router = AppRouter::new(Arc::new(cosan_service), Arc::new(env.secret_key));
    router.serve().await.unwrap();
}
```
Because the `Pool` type implements `Clone`, you should clone the pool for each repository.


---



---

