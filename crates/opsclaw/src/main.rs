use clap::{Parser, Subcommand};
mod ipc_socket;
mod mcp_stdio;
mod setup_wizard;
mod slack_collaboration;
mod slack_approval;
mod slack_adapter;
mod skill_install;
use slack_collaboration::{
    build_intro_message, plan_visible_discussion, prepare_response_for_slack, AgentProfile,
    SlackResponsePayload,
};
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
    Init {
        #[arg(long, value_enum, default_value_t = setup_wizard::Template::SreSquad)]
        template: setup_wizard::Template,
        #[arg(long)]
        api_key: Option<String>,
        #[arg(long)]
        slack_workspace: Option<String>,
        #[arg(long, default_value_t = false)]
        write_plan: bool,
        #[arg(long, default_value = ".opsclaw/setup-wizard-plan.json")]
        output: String,
    },
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
    IntroMessage {
        #[arg(long)]
        agent_json: String,
    },
    PlanDiscussion {
        #[arg(long)]
        task: String,
        #[arg(long)]
        agents_json: String,
    },
    PrepareResponse {
        #[arg(long)]
        text: String,
        #[arg(long, default_value_t = 3500)]
        max_chars: usize,
        #[arg(long, default_value = "opsclaw-response.txt")]
        snippet_name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init {
            template,
            api_key,
            slack_workspace,
            write_plan,
            output,
        }) => {
            let has_api_key = api_key
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty());
            let has_slack_workspace = slack_workspace
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty());

            let plan = setup_wizard::build_wizard_plan(template, has_api_key, has_slack_workspace);

            if write_plan {
                if let Err(err) = setup_wizard::write_wizard_plan(Path::new(output.as_str()), &plan)
                {
                    eprintln!("setup wizard failed: {err}");
                    std::process::exit(1);
                }
            }

            println!(
                "{}",
                serde_json::to_string_pretty(&plan)
                    .expect("setup wizard output serialization should succeed")
            );
        }
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
            SlackCommands::IntroMessage { agent_json } => {
                let profile: AgentProfile = match serde_json::from_str(agent_json.as_str()) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("slack intro-message failed: invalid agent_json: {err}");
                        std::process::exit(1);
                    }
                };

                match build_intro_message(&profile) {
                    Ok(message) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&serde_json::json!({
                                "message": message
                            }))
                            .expect("intro-message output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("slack intro-message failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
            SlackCommands::PlanDiscussion { task, agents_json } => {
                let agents: Vec<AgentProfile> = match serde_json::from_str(agents_json.as_str()) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("slack plan-discussion failed: invalid agents_json: {err}");
                        std::process::exit(1);
                    }
                };

                match plan_visible_discussion(task.as_str(), &agents) {
                    Ok(plan) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&serde_json::json!({
                                "assignee": plan.assignee,
                                "escalation_required": plan.escalation_required,
                                "turns": plan.turns
                            }))
                            .expect("plan-discussion output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("slack plan-discussion failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
            SlackCommands::PrepareResponse {
                text,
                max_chars,
                snippet_name,
            } => match prepare_response_for_slack(text.as_str(), max_chars, snippet_name.as_str()) {
                Ok(payload) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&slack_response_to_json(payload))
                            .expect("prepare-response output serialization should succeed")
                    );
                }
                Err(err) => {
                    eprintln!("slack prepare-response failed: {err}");
                    std::process::exit(1);
                }
            },
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

fn slack_response_to_json(payload: SlackResponsePayload) -> serde_json::Value {
    match payload {
        SlackResponsePayload::Inline { text } => serde_json::json!({
            "mode": "inline",
            "text": text
        }),
        SlackResponsePayload::Snippet {
            preview,
            file_name,
            content,
        } => serde_json::json!({
            "mode": "snippet",
            "preview": preview,
            "file_name": file_name,
            "content": content
        }),
    }
}
