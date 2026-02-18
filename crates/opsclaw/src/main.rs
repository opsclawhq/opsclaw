use clap::{Parser, Subcommand};
mod ipc_socket;
mod mcp_stdio;
mod slack_approval;
mod slack_adapter;
mod skill_install;
use slack_approval::{
    build_approval_card, card_to_block_kit_json, parse_interaction_decision, ApprovalDecision,
};
use slack_adapter::{build_install_url, retry_after_seconds, route_for_bot, SlackInstallConfig};
use std::path::Path;

#[derive(Parser)]
#[command(name = "opsclaw", version)]
struct Cli {
    #[arg(long)]
    verbose: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ipc {
        #[command(subcommand)]
        command: IpcCommands,
    },
    Slack {
        #[command(subcommand)]
        command: SlackCommands,
    },
    Mcp {
        #[command(subcommand)]
        command: McpCommands,
    },
    Skill {
        #[command(subcommand)]
        command: SkillCommands,
    },
}

#[derive(Subcommand)]
enum SkillCommands {
    Install { path: String },
}

#[derive(Subcommand)]
enum McpCommands {
    ServeStdio,
}

#[derive(Subcommand)]
enum IpcCommands {
    ServeSockets {
        #[arg(long, default_value = ".opsclaw/sockets")]
        dir: String,
    },
}

#[derive(Subcommand)]
enum SlackCommands {
    InstallUrl {
        #[arg(long)]
        client_id: String,
        #[arg(long = "scope", value_delimiter = ',')]
        scopes: Vec<String>,
        #[arg(long = "user-scope", value_delimiter = ',')]
        user_scopes: Vec<String>,
        #[arg(long)]
        redirect_uri: Option<String>,
        #[arg(long)]
        state: String,
    },
    RouteEvent {
        #[arg(long)]
        bot_user_id: String,
        #[arg(long)]
        payload_json: String,
    },
    RetryAfter {
        #[arg(long)]
        status_code: u16,
        #[arg(long)]
        retry_after: Option<String>,
    },
    BuildApprovalCard {
        #[arg(long)]
        run_id: String,
        #[arg(long)]
        command: String,
        #[arg(long)]
        rollback_template: Option<String>,
    },
    ParseInteraction {
        #[arg(long)]
        payload_json: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Ipc {
            command: IpcCommands::ServeSockets { dir },
        }) => {
            if let Err(err) = ipc_socket::serve_ipc_sockets(Path::new(dir.as_str())) {
                eprintln!("ipc socket server failed: {err}");
                std::process::exit(1);
            }
        }
        Some(Commands::Slack { command }) => match command {
            SlackCommands::InstallUrl {
                client_id,
                scopes,
                user_scopes,
                redirect_uri,
                state,
            } => {
                let config = SlackInstallConfig {
                    client_id,
                    scopes,
                    user_scopes,
                    redirect_uri,
                    state,
                };
                match build_install_url(&config) {
                    Ok(url) => println!("{url}"),
                    Err(err) => {
                        eprintln!("slack install url generation failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
            SlackCommands::RouteEvent {
                bot_user_id,
                payload_json,
            } => match route_for_bot(payload_json.as_str(), bot_user_id.as_str()) {
                Ok(route) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&route_to_json(route))
                            .expect("route json serialization should succeed")
                    );
                }
                Err(err) => {
                    eprintln!("slack route-event failed: {err}");
                    std::process::exit(1);
                }
            },
            SlackCommands::RetryAfter {
                status_code,
                retry_after,
            } => {
                let seconds = retry_after_seconds(status_code, retry_after.as_deref());
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "status_code": status_code,
                        "retry_after_seconds": seconds
                    }))
                    .expect("retry output serialization should succeed")
                );
            }
            SlackCommands::BuildApprovalCard {
                run_id,
                command,
                rollback_template,
            } => match build_approval_card(
                run_id.as_str(),
                command.as_str(),
                rollback_template.as_deref(),
            ) {
                Ok(card) => {
                    let block_kit = card_to_block_kit_json(&card);
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "run_id": card.run_id,
                            "command": card.command,
                            "expected_effect": card.expected_effect,
                            "blast_radius": card.blast_radius,
                            "rollback_steps": card.rollback_steps,
                            "approve_action_id": card.approve_action_id,
                            "reject_action_id": card.reject_action_id,
                            "slack_payload": block_kit
                        }))
                        .expect("approval-card output serialization should succeed")
                    );
                }
                Err(err) => {
                    eprintln!("slack build-approval-card failed: {err}");
                    std::process::exit(1);
                }
            },
            SlackCommands::ParseInteraction { payload_json } => {
                match parse_interaction_decision(payload_json.as_str()) {
                    Ok(parsed) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&serde_json::json!({
                                "run_id": parsed.run_id,
                                "decision": approval_decision_label(parsed.decision)
                            }))
                            .expect("interaction output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("slack parse-interaction failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
        },
        Some(Commands::Mcp {
            command: McpCommands::ServeStdio,
        }) => {
            if let Err(err) = mcp_stdio::serve_stdio() {
                eprintln!("mcp stdio server failed: {err}");
                std::process::exit(1);
            }
        }
        Some(Commands::Skill {
            command: SkillCommands::Install { path },
        }) => match skill_install::install_skill_to_default_location(path.as_ref()) {
            Ok(installed_path) => {
                println!("installed skill at {}", installed_path.display());
            }
            Err(err) => {
                eprintln!("skill install failed: {err}");
                std::process::exit(1);
            }
        },
        None => {
            if cli.verbose {
                println!("opsclaw: no subcommand provided");
            }
        }
    }
}

fn approval_decision_label(decision: ApprovalDecision) -> &'static str {
    match decision {
        ApprovalDecision::Approve => "approve",
        ApprovalDecision::Reject => "reject",
    }
}

fn route_to_json(route: slack_adapter::SlackRouteDecision) -> serde_json::Value {
    match route {
        slack_adapter::SlackRouteDecision::UrlVerification { challenge } => serde_json::json!({
            "decision": "url_verification",
            "challenge": challenge
        }),
        slack_adapter::SlackRouteDecision::Mention(mention) => serde_json::json!({
            "decision": "mention",
            "channel": mention.channel,
            "thread_ts": mention.thread_ts,
            "cleaned_text": mention.cleaned_text,
            "user_id": mention.user_id
        }),
        slack_adapter::SlackRouteDecision::Ignore => serde_json::json!({
            "decision": "ignore"
        }),
    }
}
