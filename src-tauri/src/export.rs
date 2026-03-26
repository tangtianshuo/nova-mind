use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub conversation_id: i64,
    pub title: String,
    pub format: ExportFormat,
    pub include_metadata: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportFormat {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "word")]
    Word,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub success: bool,
    pub file_path: Option<String>,
    pub error: Option<String>,
}

pub fn generate_markdown(
    title: &str,
    messages: &[(String, String, String)],
    include_metadata: bool,
) -> String {
    let mut content = String::new();

    if include_metadata {
        content.push_str(&format!("# {}\n\n", title));
        content.push_str(&format!("> 导出时间: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        content.push_str("---\n\n");
    } else {
        content.push_str(&format!("# {}\n\n", title));
    }

    for (role, content_text, timestamp) in messages {
        let role_display = match role.as_str() {
            "user" => "👤 用户",
            "assistant" => "🤖 助手",
            "system" => "⚙️ 系统",
            _ => role.as_str(),
        };

        content.push_str(&format!("## {} - {}\n\n", role_display, timestamp));
        content.push_str(&format!("{}\n\n", content_text.replace("\\n", "\n")));
        content.push_str("---\n\n");
    }

    content
}

pub fn export_to_markdown(
    file_path: &PathBuf,
    title: &str,
    messages: &[(String, String, String)],
    include_metadata: bool,
) -> Result<String, String> {
    let content = generate_markdown(title, messages, include_metadata);

    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;

    Ok(file_path.to_string_lossy().to_string())
}

pub fn generate_word_xml(
    title: &str,
    messages: &[(String, String, String)],
) -> String {
    let mut content = String::new();

    content.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
    content.push_str(r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">"#);
    content.push_str("<w:body>");

    content.push_str(&format!(
        r#"<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>{}</w:t></w:r></w:p>"#,
        escape_xml(title)
    ));

    for (role, content_text, _timestamp) in messages {
        let role_display = match role.as_str() {
            "user" => "用户",
            "assistant" => "助手",
            "system" => "系统",
            _ => role.as_str(),
        };

        content.push_str(&format!(
            r#"<w:p><w:pPr><w:pStyle w:val="Heading2"/></w:pPr><w:r><w:t>{}</w:t></w:r></w:p>"#,
            role_display
        ));

        let paragraphs: Vec<&str> = content_text.split("\\n").collect();
        for para in paragraphs {
            content.push_str(&format!(
                r#"<w:p><w:r><w:t>{}</w:t></w:r></w:p>"#,
                escape_xml(para)
            ));
        }
    }

    content.push_str("</w:body></w:document>");

    content
}

pub fn export_to_word(
    file_path: &PathBuf,
    title: &str,
    messages: &[(String, String, String)],
) -> Result<String, String> {
    let content = generate_word_xml(title, messages);

    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;

    Ok(file_path.to_string_lossy().to_string())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_markdown() {
        let messages = vec![
            ("user".to_string(), "Hello".to_string(), "2024-01-01 12:00:00".to_string()),
            ("assistant".to_string(), "Hi there!".to_string(), "2024-01-01 12:00:01".to_string()),
        ];

        let md = generate_markdown("Test", &messages, false);
        assert!(md.contains("# Test"));
        assert!(md.contains("👤 用户"));
        assert!(md.contains("🤖 助手"));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("<div>"), "&lt;div&gt;");
        assert_eq!(escape_xml("a & b"), "a &amp; b");
    }
}
