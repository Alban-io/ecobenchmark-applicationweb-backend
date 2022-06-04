use crate::error::Error;
use crate::model::task::Task;
use crate::model::list::list::{ListList, ListWithTasks};
use crate::service::database::Pool;
use actix_web::{get, web, HttpResponse};
use uuid::Uuid;

#[get("/api/accounts/{account_id}/lists")]
async fn handler(
    pool: web::Data<Pool>,
    account_id: web::Path<Uuid>,
    payload: web::Query<ListList>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.acquire().await?;
    let lists = payload.execute(&mut conn, account_id.into_inner()).await?;
    let mut result = Vec::with_capacity(lists.len());
    for list in lists {
        let tasks = Task::find_for_list(&mut conn, &list.id).await?;
        result.push(ListWithTasks {
            inner: list,
            tasks,
        });
    }
    Ok(HttpResponse::Ok().json(result))
}
