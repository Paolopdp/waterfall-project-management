use crate::errors::ServiceError;
use crate::models::lifecycle::{LifecyclePhase, PhaseDetails, PhaseTransition};
use log::info;
use sqlx::PgPool;
use uuid::Uuid;

pub struct LifecycleService;

impl LifecycleService {
    pub async fn transition_phase(
        transition: PhaseTransition,
        approver_id: Uuid,
        pool: &PgPool,
    ) -> Result<PhaseDetails, ServiceError> {
        // Start transaction
        let mut tx = pool.begin().await?;

        // Create phase transition record
        let phase_details = sqlx::query_as!(
            PhaseDetails,
            r#"
            INSERT INTO phase_transitions (
                project_id, phase, description, attachments, approved_by
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, project_id, phase as "phase: LifecyclePhase",
                      description, attachments, approved_by, created_at, updated_at
            "#,
            transition.project_id,
            transition.phase as LifecyclePhase,
            transition.description,
            transition.attachments.as_ref().map(|v| &**v),
            approver_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // Update project's current phase
        sqlx::query!(
            r#"
            UPDATE projects
            SET current_phase = $1,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            "#,
            transition.phase as LifecyclePhase,
            transition.project_id
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        info!(
            "Project {} transitioned to phase {:?}",
            transition.project_id, transition.phase
        );

        Ok(phase_details)
    }

    pub async fn get_phase_details(
        phase_id: Uuid,
        pool: &PgPool,
    ) -> Result<PhaseDetails, ServiceError> {
        let details = sqlx::query_as!(
            PhaseDetails,
            r#"
            SELECT id, project_id, phase as "phase: LifecyclePhase",
                   description, attachments, approved_by, created_at, updated_at
            FROM phase_transitions
            WHERE id = $1
            "#,
            phase_id
        )
        .fetch_one(pool)
        .await?;

        Ok(details)
    }

    pub async fn get_project_lifecycle(
        project_id: Uuid,
        pool: &PgPool,
    ) -> Result<Vec<PhaseDetails>, ServiceError> {
        let history = sqlx::query_as!(
            PhaseDetails,
            r#"
            SELECT id, project_id, phase as "phase: LifecyclePhase",
                   description, attachments, approved_by, created_at, updated_at
            FROM phase_transitions
            WHERE project_id = $1
            ORDER BY created_at ASC
            "#,
            project_id
        )
        .fetch_all(pool)
        .await?;

        Ok(history)
    }
}
