use crate::schemas::group::Group;
use crate::web::auth::User;
use crate::web::result::InternalResult;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use rand::seq::SliceRandom;
use rand::Rng;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::cell::LazyCell;
use uuid::Uuid;

const WORDS_STRING_LIST: &str = include_str!("../../static/all_words_cleaned.txt");
#[allow(clippy::declare_interior_mutable_const)]
const WORDS_ARRAY: LazyCell<Vec<&str>> = LazyCell::new(|| WORDS_STRING_LIST.lines().collect());

pub fn groups_controller() -> Router<AppState> {
    Router::new()
        .route("/", post(create_group).get(joined_groups))
        .route(
            "/:group",
            get(get_group).post(join_group).delete(leave_group),
        )
        .route("/:group/:user", delete(kick_member))
        .route("/:group/members", get(get_group_members))
        .route("/:group/week", get(get_week_data))
}

fn generate_code() -> String {
    let mut rng = rand::thread_rng();

    #[allow(clippy::borrow_interior_mutable_const)]
    let words = WORDS_ARRAY
        .choose_multiple(&mut rng, 2)
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    let num = rng.gen_range(0..=9);
    let [first_word, second_word] = &words[..] else {
        unreachable!();
    };

    format!("{first_word}{num}{second_word}")
}

#[derive(Deserialize)]
struct CreateGroupPayload {
    name: String,
}

async fn create_group(
    user: User,
    State(AppState { pool, .. }): State<AppState>,
    Json(CreateGroupPayload { name }): Json<CreateGroupPayload>,
) -> InternalResult<Json<Group>> {
    let code = generate_code();
    sqlx::query_as::<_, Group>(
        // language=postgresql
        "INSERT INTO groups (name, code, owner_id) VALUES ($1, $2, $3) RETURNING *",
    )
        .bind(name)
        .bind(code)
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

#[derive(Serialize, FromRow)]
struct GetGroupBody {
    id: Uuid,
    name: String,
    members: i32, // doesn't support unsigned ints
}

async fn get_group(
    Path(code): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> InternalResult<Json<Option<GetGroupBody>>> {
    sqlx::query_as::<_, GetGroupBody>(
        // language=postgresql
        "
    SELECT g.id, g.name, COUNT(gm.user_id) as members
       FROM groups g
           LEFT JOIN group_memberships gm ON g.id = gm.group_id
     WHERE g.code = $1
     GROUP BY g.id
",
    )
        .bind(code)
        .fetch_optional(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn join_group(
    Path(code): Path<String>,
    user: User,
    State(AppState { pool, .. }): State<AppState>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    sqlx::query(
        // language=postgresql
        "
        INSERT INTO group_memberships (group_id, user_id) VALUES (
                (SELECT id FROM groups WHERE code = $1),
                $2
            )
        ",
    )
        .bind(code)
        .bind(user.id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "you are already a member of this group",
            )
        })
}

#[derive(FromRow, Serialize)]
struct TrimmedUser {
    id: Uuid,
    first_name: String,
    last_name: String,
}

async fn get_group_members(
    user: User,
    Path(code): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> InternalResult<Json<Vec<TrimmedUser>>> {
    sqlx::query_as::<_, TrimmedUser>(
        // language=postgresql
        "
        SELECT p.id, p.first_name, p.last_name 
        FROM profiles p
        JOIN group_memberships gm ON p.id = gm.user_id
        JOIN groups g ON gm.group_id = g.id
        WHERE g.code = $1 LIMIT 1
        AND EXISTS (
            SELECT 1
            FROM group_memberships gm2
            JOIN groups g2 ON gm2.group_id = g2.id
            WHERE g2.code = $1 
            AND gm2.user_id = $2
        )
    ",
    )
        .bind(code)
        .bind(user.id)
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn leave_group(
    user: User,
    Path(code): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> InternalResult<StatusCode> {
    sqlx::query(
        // language=postgresql
        "
        DELETE FROM group_memberships
        WHERE group_id = (SELECT id FROM groups WHERE code = $1)
        AND user_id = $2
    ",
    )
        .bind(code)
        .bind(user.id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::OK)
        .map_err(Into::into)
}

async fn kick_member(
    user: User,
    Path((code, target_user_id)): Path<(String, Uuid)>,
    State(AppState { pool, .. }): State<AppState>,
) -> InternalResult<StatusCode> {
    sqlx::query(
        // language=postgresql
        "
        DELETE FROM group_memberships
        WHERE group_id = (SELECT id FROM groups WHERE code = $1 AND owner_id = $2)
        AND user_id = $3
    ",
    )
        .bind(code)
        .bind(user.id)
        .bind(target_user_id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::OK)
        .map_err(Into::into)
}

#[derive(Serialize, FromRow)]
struct WeekData {
    days: Vec<DayData>,
    total_weeks: i64,
}

#[derive(Serialize, FromRow)]
struct DayData {
    day: chrono::NaiveDate,
    ratings: Vec<f32>,
}

#[derive(Deserialize)]
struct WeekDataQuery {
    skip: Option<i32>,
}

async fn get_week_data(
    user: User,
    Query(query): Query<WeekDataQuery>,
    Path(code): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> InternalResult<Json<WeekData>> {
    let skip = query.skip.unwrap_or_default();

    let group_member_ids = sqlx::query_as::<_, (Uuid,)>(
        // language=postgresql
        "
        WITH found_group AS (
            SELECT g.id FROM groups g 
            JOIN group_memberships gm ON g.id = gm.group_id AND gm.user_id = $1
            WHERE g.code = $2
        )
        SELECT gm.user_id 
        FROM group_memberships gm
        WHERE gm.group_id IN (SELECT id FROM found_group)
        ",
    )
        .bind(user.id)
        .bind(&code)
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|(id, )| id)
        .collect::<Vec<Uuid>>();

    let days = sqlx::query_as::<_, DayData>(
        // language=postgresql
        "
        WITH week_entries AS (
            SELECT date, emotion_scale
            FROM entries
            WHERE author = ANY($1)
            AND date > NOW() - INTERVAL '7 days'
        )
        SELECT 
            date as day,
            array_agg(emotion_scale ORDER BY emotion_scale) as ratings
        FROM week_entries
        GROUP BY date
        ORDER BY date
        OFFSET $2
        ",
    )
        .bind(&group_member_ids)
        .bind(skip)
        .fetch_all(&pool)
        .await?;

    let total_weeks = sqlx::query_scalar::<_, i64>(
        // language=postgresql
        "
        WITH week_entries AS (
            SELECT date
            FROM entries
            WHERE author = ANY($1)
            AND date > NOW() - INTERVAL '7 days'
        )
        SELECT COUNT(DISTINCT date_trunc('week', date))
        FROM week_entries
        ",
    )
        .bind(&group_member_ids)
        .fetch_one(&pool)
        .await?;

    Ok(Json(WeekData { days, total_weeks }))
}


#[derive(Serialize, FromRow)]
struct SelfGroup {
    id: Uuid,
    name: String,
    code: String,
}

async fn joined_groups(
    user: User,
    State(AppState { pool, .. }): State<AppState>,
) -> InternalResult<Json<Vec<SelfGroup>>> {
    sqlx::query_as::<_, SelfGroup>(
        // language=postgresql
        "

        SELECT gm.group_id as group_id FROM group_memberships gm
        JOIN groups g ON gm.group_id = g.id
        WHERE gm.user_id = $1
    ",
    )
        .bind(user.id)
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}