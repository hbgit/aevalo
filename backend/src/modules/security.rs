// modules/security.rs
//! Security module for anomaly detection and threat monitoring

use sqlx::PgPool;
use crate::error::AppError;
use chrono::Utc;

/// Session anomaly detector
pub struct AnomalyDetector;

impl AnomalyDetector {
    /// Detects multiple concurrent sessions from different locations
    pub async fn detect_concurrent_sessions(
        user_id: &str,
        pool: &PgPool,
    ) -> Result<bool, AppError> {
        let active_sessions: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sessions \
             WHERE user_id = $1 AND status = 'active' AND expires_at > NOW()"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Maximum 5 concurrent sessions per user
        Ok(active_sessions.0 > 5)
    }

    /// Detects impossible travel (login from geographically distant locations)
    pub async fn detect_impossible_travel(
        user_id: &str,
        new_ip: &str,
        pool: &PgPool,
    ) -> Result<bool, AppError> {
        let last_session: Option<(String, i64)> = sqlx::query_as(
            "SELECT ip_address, \
             EXTRACT(EPOCH FROM (NOW() - created_at))::int8 as seconds_ago \
             FROM sessions \
             WHERE user_id = $1 AND status = 'active' \
             ORDER BY created_at DESC LIMIT 1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some((last_ip, seconds_ago)) = last_session {
            // If login occurred less than 10 minutes from a different IP block,
            // it's physically impossible
            if seconds_ago < 600 && !Self::is_same_ip_block(&last_ip, new_ip) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Flags a session as suspicious
    pub async fn flag_suspicious_session(
        session_id: &str,
        reason: &str,
        pool: &PgPool,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE sessions SET is_suspicious = TRUE WHERE id = $1"
        )
        .bind(session_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Log the security event
        sqlx::query(
            "INSERT INTO security_events (session_id, event_type, description, created_at) \
             VALUES ($1, 'suspicious_activity', $2, NOW())"
        )
        .bind(session_id)
        .bind(reason)
        .execute(pool)
        .await
        .ok();

        Ok(())
    }

    /// Checks if two IPs are in the same block (simple check)
    fn is_same_ip_block(ip1: &str, ip2: &str) -> bool {
        let parts1: Vec<&str> = ip1.split('.').collect();
        let parts2: Vec<&str> = ip2.split('.').collect();

        if parts1.len() != 4 || parts2.len() != 4 {
            return false;
        }

        // Consider same if first 3 octets match
        parts1[0] == parts2[0] && parts1[1] == parts2[1] && parts1[2] == parts2[2]
    }

    /// Logs a security event
    pub async fn log_security_event(
        session_id: Option<&str>,
        user_id: Option<&str>,
        event_type: &str,
        description: &str,
        ip_address: Option<&str>,
        pool: &PgPool,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO security_events (session_id, user_id, event_type, description, ip_address, created_at) \
             VALUES ($1, $2, $3, $4, $5, NOW())"
        )
        .bind(session_id)
        .bind(user_id)
        .bind(event_type)
        .bind(description)
        .bind(ip_address)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Creates a security alert
    pub async fn create_alert(
        user_id: &str,
        alert_type: &str,
        severity: &str,
        pool: &PgPool,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO security_alerts (user_id, alert_type, severity, resolved, created_at) \
             VALUES ($1, $2, $3, FALSE, NOW())"
        )
        .bind(user_id)
        .bind(alert_type)
        .bind(severity)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_ip_block() {
        assert!(AnomalyDetector::is_same_ip_block("192.168.1.1", "192.168.1.100"));
        assert!(!AnomalyDetector::is_same_ip_block("192.168.1.1", "192.168.2.1"));
        assert!(!AnomalyDetector::is_same_ip_block("invalid", "192.168.1.1"));
    }
}
