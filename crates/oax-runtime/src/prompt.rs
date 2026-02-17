use oax_core::soul::{load_soul_file, SoulProfile};
use std::path::Path;

pub fn compose_system_prompt(base_instructions: &str, soul: &SoulProfile) -> String {
    format!(
        "## Identity\nName: {name}\nRole: {role}\nPersonality: {personality}\nCommunication Style: {communication_style}\nAvatar: {avatar}\n\n## Base Instructions\n{base}\n\n## Soul Instructions\n{soul_prompt}",
        name = soul.name,
        role = soul.role,
        personality = soul.personality,
        communication_style = soul.communication_style,
        avatar = soul.avatar,
        base = base_instructions,
        soul_prompt = soul.system_prompt,
    )
}

pub fn compose_system_prompt_from_file(
    base_instructions: &str,
    soul_path: &Path,
) -> Result<String, String> {
    let soul = load_soul_file(soul_path)?;
    Ok(compose_system_prompt(base_instructions, &soul))
}

#[cfg(test)]
mod tests {
    use oax_core::soul::{load_soul_file, preset_soul_paths, SoulProfile};

    use super::{compose_system_prompt, compose_system_prompt_from_file};

    #[test]
    fn compose_system_prompt_includes_identity_and_base_sections() {
        let soul = SoulProfile {
            name: "Remy".to_string(),
            role: "SRE".to_string(),
            personality: "Calm and analytical".to_string(),
            communication_style: "concise".to_string(),
            avatar: "remy.png".to_string(),
            system_prompt: "Focus on reliability and clear escalation.".to_string(),
        };

        let prompt = compose_system_prompt("Always prefer safe, reversible actions.", &soul);
        assert!(prompt.contains("Name: Remy"));
        assert!(prompt.contains("Role: SRE"));
        assert!(prompt.contains("Always prefer safe, reversible actions."));
        assert!(prompt.contains("Focus on reliability"));
    }

    #[test]
    fn compose_system_prompt_from_file_loads_and_injects_soul_profile() {
        let paths = preset_soul_paths();
        let remy = paths
            .iter()
            .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("remy.md"))
            .expect("remy preset should exist");

        let prompt =
            compose_system_prompt_from_file("Escalate uncertainty clearly.", remy.as_path())
                .expect("prompt should compose from soul file");
        assert!(prompt.contains("Name: Remy"));
        assert!(prompt.contains("Escalate uncertainty clearly."));
    }

    #[test]
    fn different_souls_produce_different_prompt_outputs() {
        let paths = preset_soul_paths();
        let remy_path = paths
            .iter()
            .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("remy.md"))
            .expect("remy preset should exist");
        let ferris_path = paths
            .iter()
            .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("ferris.md"))
            .expect("ferris preset should exist");

        let remy = load_soul_file(remy_path).expect("remy should load");
        let ferris = load_soul_file(ferris_path).expect("ferris should load");
        let remy_prompt = compose_system_prompt("Use the same base instruction.", &remy);
        let ferris_prompt = compose_system_prompt("Use the same base instruction.", &ferris);

        assert_ne!(remy_prompt, ferris_prompt);
        assert!(remy_prompt.contains("Name: Remy"));
        assert!(ferris_prompt.contains("Name: Ferris"));
    }
}
