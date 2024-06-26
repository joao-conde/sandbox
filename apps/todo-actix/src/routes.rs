use crate::{
    app::AppData,
    db,
    error::ApiError,
    todo::{CreateTodo, UpdateTodo},
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[get("/todos")]
pub async fn list_todos(app_data: Data<AppData>) -> Result<HttpResponse, ApiError> {
    let todos = db::list_todos(&app_data.db_pool).await?;
    let response = HttpResponse::Ok().json(todos);
    Ok(response)
}

#[get("/todos/{id}")]
pub async fn get_todo(app_data: Data<AppData>, id: Path<i64>) -> Result<HttpResponse, ApiError> {
    let todo = db::get_todo(&app_data.db_pool, *id).await?;
    let response = HttpResponse::Ok().json(todo);
    Ok(response)
}

#[post("/todos")]
pub async fn create_todo(
    app_data: Data<AppData>,
    todo: Json<CreateTodo>,
) -> Result<HttpResponse, ApiError> {
    let created = db::create_todo(&app_data.db_pool, todo.into_inner()).await?;
    let response = HttpResponse::Ok().json(created);
    Ok(response)
}

#[put("/todos/{id}")]
pub async fn update_todo(
    app_data: Data<AppData>,
    id: Path<i64>,
    todo: Json<UpdateTodo>,
) -> Result<HttpResponse, ApiError> {
    let updated = db::update_todo(&app_data.db_pool, *id, todo.into_inner()).await?;
    let response = HttpResponse::Ok().json(updated);
    Ok(response)
}

#[delete("/todos/{id}")]
pub async fn delete_todo(app_data: Data<AppData>, id: Path<i64>) -> Result<HttpResponse, ApiError> {
    let deleted = db::delete_todo(&app_data.db_pool, *id).await?;
    let response = HttpResponse::Ok().json(deleted);
    Ok(response)
}

#[cfg(test)]
mod test {
    use crate::{
        test::{make_request, BoxBodyTest},
        todo::{CreateTodo, Todo, UpdateTodo},
    };
    use actix_web::{http::StatusCode, test};
    use sqlx::SqlitePool;

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn list_todos(pool: SqlitePool) {
        let request = test::TestRequest::get().uri("/todos");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body: Vec<Todo> = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(
            body,
            vec![
                Todo {
                    id: 1,
                    title: "todo1".to_string(),
                    description: "description1".to_string()
                },
                Todo {
                    id: 2,
                    title: "todo2".to_string(),
                    description: "description2".to_string()
                },
                Todo {
                    id: 3,
                    title: "todo3".to_string(),
                    description: "description3".to_string()
                }
            ]
        );
    }

    #[sqlx::test]
    async fn list_todos_empty(pool: SqlitePool) {
        let request = test::TestRequest::get().uri("/todos");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body: Vec<Todo> = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(body, vec![]);
    }

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn get_todo(pool: SqlitePool) {
        let request = test::TestRequest::get().uri("/todos/2");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body: Todo = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(
            body,
            Todo {
                id: 2,
                title: "todo2".to_string(),
                description: "description2".to_string()
            }
        );
    }

    #[sqlx::test]
    async fn get_todo_not_found(pool: SqlitePool) {
        let request = test::TestRequest::get().uri("/todos/2");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body = response.into_body().as_str().await;
        assert_eq!(status_code, StatusCode::NOT_FOUND);
        assert_eq!(body, "Not Found");
    }

    #[sqlx::test]
    async fn create_todo(pool: SqlitePool) {
        let request = test::TestRequest::post()
            .uri("/todos")
            .set_json(CreateTodo {
                title: "title".to_string(),
                description: "description".to_string(),
            });
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body: Todo = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(
            body,
            Todo {
                id: 1,
                title: "title".to_string(),
                description: "description".to_string()
            }
        );
    }

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn update_todo(pool: SqlitePool) {
        let request = test::TestRequest::put()
            .uri("/todos/2")
            .set_json(UpdateTodo {
                title: "title".to_string(),
                description: "description".to_string(),
            });
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body: Todo = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(
            body,
            Todo {
                id: 2,
                title: "title".to_string(),
                description: "description".to_string()
            }
        );
    }

    #[sqlx::test]
    async fn update_todo_not_found(pool: SqlitePool) {
        let request = test::TestRequest::put()
            .uri("/todos/999")
            .set_json(UpdateTodo {
                title: "title".to_string(),
                description: "description".to_string(),
            });
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body = response.into_body().as_str().await;
        assert_eq!(status_code, StatusCode::NOT_FOUND);
        assert_eq!(body, "Not Found");
    }

    #[sqlx::test(fixtures("test/fixtures/todos.sql"))]
    async fn delete_todo(pool: SqlitePool) {
        let request = test::TestRequest::delete().uri("/todos/2");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body: Todo = response.into_body().deserialize().await;
        assert_eq!(status_code, StatusCode::OK);
        assert_eq!(
            body,
            Todo {
                id: 2,
                title: "todo2".to_string(),
                description: "description2".to_string()
            }
        );
    }

    #[sqlx::test]
    async fn delete_todo_not_found(pool: SqlitePool) {
        let request = test::TestRequest::delete().uri("/todos/999");
        let response = make_request(pool, request).await;

        let status_code = response.status();
        let body = response.into_body().as_str().await;
        assert_eq!(status_code, StatusCode::NOT_FOUND);
        assert_eq!(body, "Not Found");
    }
}
