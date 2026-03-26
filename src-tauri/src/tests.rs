#[cfg(test)]
mod tests {
    use crate::health::{HealthStatus, Components};
    use crate::db::Mindmap;
    use crate::sandbox::{SandboxStatus, SandboxInfo};

    #[test]
    fn test_health_status_default() {
        let status = HealthStatus::default();
        assert_eq!(status.status, "healthy");
        assert!(status.components.frontend);
        assert!(status.components.backend);
        assert!(!status.components.sandbox);
        assert!(!status.components.database);
    }

    #[test]
    fn test_components_structure() {
        let components = Components {
            frontend: true,
            backend: true,
            sandbox: false,
            database: false,
        };
        assert!(components.frontend);
        assert!(components.backend);
        assert!(!components.sandbox);
        assert!(!components.database);
    }

    #[test]
    fn test_mindmap_structure() {
        let mindmap = Mindmap {
            id: Some(1),
            title: "测试思维导图".to_string(),
            content: "{}".to_string(),
            created_at: "2024-03-20T00:00:00".to_string(),
            updated_at: "2024-03-20T00:00:00".to_string(),
        };
        assert_eq!(mindmap.id, Some(1));
        assert_eq!(mindmap.title, "测试思维导图");
        assert_eq!(mindmap.content, "{}");
    }

    #[test]
    fn test_sandbox_status_variants() {
        assert_eq!(format!("{:?}", SandboxStatus::Uninitialized), "Uninitialized");
        assert_eq!(format!("{:?}", SandboxStatus::Downloading), "Downloading");
        assert_eq!(format!("{:?}", SandboxStatus::Ready), "Ready");
        assert_eq!(format!("{:?}", SandboxStatus::Running), "Running");
        assert_eq!(format!("{:?}", SandboxStatus::Error), "Error");
    }

    #[test]
    fn test_sandbox_info_structure() {
        let info = SandboxInfo {
            status: SandboxStatus::Ready,
            version: "1.0.0".to_string(),
            path: "/path/to/sandbox".to_string(),
            port: Some(8080),
        };
        assert_eq!(info.status, SandboxStatus::Ready);
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.path, "/path/to/sandbox");
        assert_eq!(info.port, Some(8080));
    }

    #[test]
    fn test_command_result_ok() {
        use crate::commands::CommandResult;
        let result: CommandResult<i64> = CommandResult::ok(42);
        assert!(result.success);
        assert_eq!(result.data, Some(42));
        assert_eq!(result.error, None);
    }

    #[test]
    fn test_command_result_err() {
        use crate::commands::CommandResult;
        let result: CommandResult<i64> = CommandResult::err("测试错误".to_string());
        assert!(!result.success);
        assert_eq!(result.data, None);
        assert_eq!(result.error, Some("测试错误".to_string()));
    }
}
