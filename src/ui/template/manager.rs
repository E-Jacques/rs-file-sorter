pub struct TemplateManager;
impl TemplateManager {
    #[cfg(not(target_os = "windows"))]
    const PATH: &str = "rsc/templates.json";

    #[cfg(target_os = "windows")]
    const PATH: &str = "rsc\\templates.json";

    pub fn list() -> Vec<super::template::Template> {
        std::fs::read_to_string(Self::PATH)
            .ok()
            .and_then(|content| {
                serde_json::from_str::<Vec<super::template::Template>>(&content).ok()
            })
            .unwrap_or_default()
    }

    pub fn save(template: super::template::Template) -> Result<(), String> {
        let mut curr = TemplateManager::list();
        curr.push(template);

        let content = serde_json::to_string(&curr).map_err(|err| err.to_string())?;
        std::fs::write(Self::PATH, content).map_err(|err| err.to_string())?;

        Ok(())
    }
}
