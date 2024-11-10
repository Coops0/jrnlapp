use crate::impl_service;
use crate::schemas::group::Group;
use crate::schemas::user::User;
use serde::Serialize;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, FromRow, PgPool};
use uuid::Uuid;

pub struct GroupService(PgPool);
impl_service!(GroupService);

#[derive(Serialize, FromRow)]
pub struct GetGroupAndMembersBody {
    pub id: Uuid,
    pub name: String,
    pub members: i64,
}

#[derive(FromRow)]
pub struct TrimmedUser {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, FromRow)]
pub struct SelfGroup {
    pub id: Uuid,
    pub name: String,
    pub code: String,
}

impl GroupService {
    pub async fn get_owned_groups_count(&self, user: &User) -> Result<i64, Error> {
        sqlx::query_scalar(
            // language=postgresql
            "SELECT COUNT(*) FROM groups WHERE owner_id = $1",
        )
        .bind(user.id)
        .fetch_one(&self.0)
        .await
    }

    pub async fn create_group(&self, user: &User, name: &str) -> Result<Group, Error> {
        let code = Group::generate_code();
        sqlx::query_as(
            // language=postgresql
            "INSERT INTO groups (name, code, owner_id) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(name)
        .bind(code)
        .bind(user.id)
        .fetch_one(&self.0)
        .await
    }

    pub async fn get_group_and_members_maybe_by_code(
        &self,
        code: &str,
    ) -> Result<Option<GetGroupAndMembersBody>, Error> {
        sqlx::query_as(
            // language=postgresql
            "
    SELECT g.id, g.name, g.owner_id, COUNT(gm.user_id) as members
       FROM groups g
           LEFT JOIN group_memberships gm ON g.id = gm.group_id
     WHERE g.code = $1
     GROUP BY g.id LIMIT 1
            ",
        )
        .bind(code)
        .fetch_optional(&self.0)
        .await
    }

    pub async fn get_joined_groups_count(&self, user: &User) -> Result<i64, Error> {
        sqlx::query_scalar(
            // language=postgresql
            "SELECT COUNT(*) FROM group_memberships WHERE user_id = $1",
        )
        .bind(user.id)
        .fetch_one(&self.0)
        .await
    }

    pub async fn join_group(&self, code: &str, user: &User) -> Result<PgQueryResult, Error> {
        sqlx::query(
            // language=postgresql
            "
        INSERT INTO group_memberships (group_id, user_id) VALUES (
                (SELECT id FROM groups WHERE code = $1 LIMIT 1),
                $2
            )
        ",
        )
        .bind(code)
        .bind(user.id)
        .execute(&self.0)
        .await
    }

    pub async fn get_joined_group_by_code(&self, user: &User, code: &str) -> Result<Group, Error> {
        sqlx::query_as(
            // language=postgresql
            "
        SELECT * FROM groups WHERE code = $1
        AND EXISTS (
            SELECT 1
            FROM group_memberships gm
            WHERE gm.group_id = groups.id
            AND gm.user_id = $2
        )
        LIMIT 1
        ",
        )
        .bind(code)
        .bind(user.id)
        .fetch_one(&self.0)
        .await
    }

    pub async fn get_group_members(&self, group: &Group) -> Result<Vec<TrimmedUser>, Error> {
        sqlx::query_as(
            // language=postgresql
            "
        SELECT u.id, u.name FROM users u
        JOIN group_memberships gm ON u.id = gm.user_id
        WHERE gm.group_id = $1
    ",
        )
        .bind(group.id)
        .fetch_all(&self.0)
        .await
    }

    pub async fn get_group_by_code(&self, code: &str) -> Result<Group, Error> {
        sqlx::query_as(
            // language=postgresql
            "SELECT * FROM groups WHERE code = $1 LIMIT 1",
        )
        .bind(code)
        .fetch_one(&self.0)
        .await
    }

    pub async fn leave_group(&self, user: &User, group: &Group) -> Result<PgQueryResult, Error> {
        sqlx::query(
            // language=postgresql
            "
        DELETE FROM group_memberships
        WHERE group_id = $1
        AND user_id = $2
    ",
        )
        .bind(group.id)
        .bind(user.id)
        .execute(&self.0)
        .await
    }

    pub async fn get_group_members_count(&self, group: &Group) -> Result<i64, Error> {
        sqlx::query_scalar(
            // language=postgresql
            "SELECT COUNT(*) FROM group_memberships WHERE group_id = $1",
        )
        .bind(group.id)
        .fetch_one(&self.0)
        .await
    }

    pub async fn delete_group(&self, group: &Group) -> Result<PgQueryResult, Error> {
        sqlx::query(
            // language=postgresql
            "DELETE FROM groups WHERE id = $1",
        )
        .bind(group.id)
        .execute(&self.0)
        .await
    }

    pub async fn assign_group_new_owner(&self, group: &Group) -> Result<PgQueryResult, Error> {
        sqlx::query(
            // language=postgresql
            "
            UPDATE groups SET owner_id = (
               SELECT user_id FROM group_memberships WHERE group_id = $1 LIMIT 1
            )
            WHERE id = $1
            ",
        )
        .bind(group.id)
        .execute(&self.0)
        .await
    }

    pub async fn kick_group_by_code_member(
        &self,
        code: &str,
        owner: &User,
        target_user_id: &Uuid,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query(
            // language=postgresql
            "
        DELETE FROM group_memberships
        WHERE group_id = (SELECT id FROM groups WHERE code = $1 AND owner_id = $2)
        AND user_id = $3
    ",
        )
        .bind(code)
        .bind(owner.id)
        .bind(target_user_id)
        .execute(&self.0)
        .await
    }

    pub async fn get_joined_groups(&self, user: &User) -> Result<Vec<SelfGroup>, Error> {
        sqlx::query_as(
            // language=postgresql
            "
    SELECT *, gm.group_id as group_id FROM group_memberships gm
        JOIN groups g ON gm.group_id = g.id
        WHERE gm.user_id = $1
    ",
        )
        .bind(user.id)
        .fetch_all(&self.0)
        .await
    }
}
