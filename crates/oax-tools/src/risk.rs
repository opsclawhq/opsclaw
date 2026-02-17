use crate::shell::is_read_only_command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskClass {
    Read,
    SafeWrite,
    Destructive,
    Forbidden,
}

pub fn classify_command_risk(command: &str) -> RiskClass {
    let normalized = command.trim().to_lowercase();

    if is_forbidden(&normalized) {
        return RiskClass::Forbidden;
    }

    if is_read_only_command(&normalized) {
        return RiskClass::Read;
    }

    if is_destructive(&normalized) {
        return RiskClass::Destructive;
    }

    RiskClass::SafeWrite
}

fn is_forbidden(command: &str) -> bool {
    const FORBIDDEN_PREFIXES: [&str; 5] = [
        "rm -rf /",
        "sudo reboot",
        "shutdown -h",
        "mkfs",
        "dd if=/dev/zero",
    ];

    FORBIDDEN_PREFIXES
        .iter()
        .any(|prefix| command.starts_with(prefix))
}

fn is_destructive(command: &str) -> bool {
    const DESTRUCTIVE_MARKERS: [&str; 6] = [
        " delete ",
        " destroy",
        "drop database",
        "terraform destroy",
        "git push --force",
        "truncate ",
    ];

    DESTRUCTIVE_MARKERS
        .iter()
        .any(|marker| command.contains(marker))
        || command.starts_with("kubectl delete")
}

#[cfg(test)]
mod tests {
    use super::{classify_command_risk, RiskClass};

    #[test]
    fn read_only_commands_are_classified_as_read() {
        assert_eq!(
            classify_command_risk("kubectl get pods"),
            RiskClass::Read
        );
        assert_eq!(classify_command_risk("git diff"), RiskClass::Read);
    }

    #[test]
    fn safe_writes_are_classified_separately() {
        assert_eq!(
            classify_command_risk("kubectl scale deploy/api --replicas=4"),
            RiskClass::SafeWrite
        );
    }

    #[test]
    fn destructive_commands_are_classified_as_destructive() {
        assert_eq!(
            classify_command_risk("kubectl delete pod api-1"),
            RiskClass::Destructive
        );
        assert_eq!(classify_command_risk("terraform destroy -auto-approve"), RiskClass::Destructive);
    }

    #[test]
    fn forbidden_commands_are_explicitly_blocked() {
        assert_eq!(classify_command_risk("rm -rf /"), RiskClass::Forbidden);
        assert_eq!(classify_command_risk("sudo reboot"), RiskClass::Forbidden);
    }
}
