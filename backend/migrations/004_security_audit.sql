-- migrations/004_security_audit.sql
-- Security events and alerts logging

-- Security events table for audit logging
CREATE TABLE IF NOT EXISTS security_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID REFERENCES sessions(id) ON DELETE SET NULL,
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    description TEXT,
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Enable Row-Level Security
ALTER TABLE security_events ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view own security events"
    ON security_events FOR SELECT
    USING (auth.uid() = user_id);

CREATE POLICY "Admins can view all security events"
    ON security_events FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM auth.users
            WHERE id = auth.uid() AND role = 'admin'
        )
    );

-- Indexes
CREATE INDEX idx_security_events_user_id ON security_events(user_id);
CREATE INDEX idx_security_events_session_id ON security_events(session_id);
CREATE INDEX idx_security_events_created_at ON security_events(created_at DESC);
CREATE INDEX idx_security_events_event_type ON security_events(event_type);

-- Security alerts table
CREATE TABLE IF NOT EXISTS security_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    alert_type TEXT NOT NULL,
    severity TEXT NOT NULL CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    description TEXT,
    resolved BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMP WITH TIME ZONE
);

-- Enable Row-Level Security
ALTER TABLE security_alerts ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view own security alerts"
    ON security_alerts FOR SELECT
    USING (auth.uid() = user_id);

CREATE POLICY "Admins can manage all security alerts"
    ON security_alerts
    USING (
        EXISTS (
            SELECT 1 FROM auth.users
            WHERE id = auth.uid() AND role = 'admin'
        )
    );

-- Indexes
CREATE INDEX idx_security_alerts_user_id ON security_alerts(user_id);
CREATE INDEX idx_security_alerts_created_at ON security_alerts(created_at DESC);
CREATE INDEX idx_security_alerts_severity ON security_alerts(severity);
CREATE INDEX idx_security_alerts_resolved ON security_alerts(resolved) WHERE resolved = FALSE;

-- Audit log table for all database changes
CREATE TABLE IF NOT EXISTS audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE SET NULL,
    table_name TEXT NOT NULL,
    operation TEXT NOT NULL CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE')),
    old_values JSONB,
    new_values JSONB,
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX idx_audit_log_table_name ON audit_log(table_name);
CREATE INDEX idx_audit_log_created_at ON audit_log(created_at DESC);
CREATE INDEX idx_audit_log_operation ON audit_log(operation);

-- Function to log authentication attempts
CREATE OR REPLACE FUNCTION log_auth_attempt(
    p_user_id UUID,
    p_success BOOLEAN,
    p_ip_address INET
) RETURNS void AS $$
BEGIN
    INSERT INTO security_events (user_id, event_type, description, ip_address, created_at)
    VALUES (
        p_user_id,
        CASE WHEN p_success THEN 'login_success' ELSE 'login_failed' END,
        CASE WHEN p_success THEN 'Successful login' ELSE 'Failed login attempt' END,
        p_ip_address,
        NOW()
    );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to clean up expired sessions
CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS void AS $$
BEGIN
    UPDATE sessions
    SET status = 'expired'
    WHERE status = 'active' AND expires_at < NOW();
    
    DELETE FROM sessions
    WHERE status = 'expired' AND created_at < NOW() - INTERVAL '90 days';
END;
$$ LANGUAGE plpgsql;

-- Function to detect suspicious activity
CREATE OR REPLACE FUNCTION check_suspicious_activity(
    p_user_id UUID,
    p_session_id UUID
) RETURNS BOOLEAN AS $$
DECLARE
    v_suspicious BOOLEAN := FALSE;
    v_concurrent_count INT;
BEGIN
    -- Check for too many concurrent sessions
    SELECT COUNT(*)
    INTO v_concurrent_count
    FROM sessions
    WHERE user_id = p_user_id AND status = 'active' AND expires_at > NOW();
    
    IF v_concurrent_count > 5 THEN
        v_suspicious := TRUE;
    END IF;
    
    -- Update session if suspicious
    IF v_suspicious THEN
        UPDATE sessions
        SET is_suspicious = TRUE
        WHERE id = p_session_id;
        
        INSERT INTO security_events (session_id, user_id, event_type, description, created_at)
        VALUES (p_session_id, p_user_id, 'suspicious_activity', 'Suspicious activity detected', NOW());
    END IF;
    
    RETURN v_suspicious;
END;
$$ LANGUAGE plpgsql;
