const READ_ONLY_PREFIXES: [&str; 8] = [
    "kubectl get",
    "kubectl describe",
    "kubectl logs",
    "git status",
    "git diff",
    "ls",
    "cat",
    "echo",
];

pub fn is_read_only_command(_cmd: &str) -> bool {
    let normalized = _cmd.trim().to_lowercase();
    READ_ONLY_PREFIXES
        .iter()
        .any(|prefix| normalized.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::is_read_only_command;

    #[test]
    fn allows_known_read_only_commands() {
        assert!(is_read_only_command("kubectl get pods -n prod"));
        assert!(is_read_only_command("git status"));
        assert!(is_read_only_command("ls -la"));
    }

    #[test]
    fn blocks_destructive_or_mutating_commands() {
        assert!(!is_read_only_command("kubectl delete pod api-1"));
        assert!(!is_read_only_command("git push origin main"));
        assert!(!is_read_only_command("rm -rf /tmp/x"));
    }
}
