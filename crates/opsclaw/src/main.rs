use clap::{Parser, Subcommand};
mod channels_router;
mod discord_adapter;
mod ipc_socket;
mod mcp_stdio;
mod setup_wizard;
mod skill_install;
mod slack_adapter;
mod slack_approval;
mod slack_collaboration;
mod squad_responder;
mod squad_runtime;
mod telegram_adapter;
use channels_router::{route_platform_event, ChannelPlatform, ChannelRouteDecision};
use discord_adapter::{
    build_embed, handle_live_event as handle_discord_live_event, is_role_authorized,
    resolve_bot_token as resolve_discord_bot_token, route_discord_payload, DiscordLiveDecision,
    DiscordRouteDecision, HttpDiscordApi,
};
use slack_adapter::{
    build_install_url, handle_live_event, resolve_bot_token as resolve_slack_bot_token,
    retry_after_seconds, route_for_bot, HttpSlackApi, SlackInstallConfig, SlackLiveDecision,
};
use slack_approval::{
    build_approval_card, card_to_block_kit_json, parse_interaction_decision, ApprovalDecision,
};
use slack_collaboration::{
    build_intro_message, plan_visible_discussion, prepare_response_for_slack, AgentProfile,
    SlackResponsePayload,
};
use squad_runtime::{process_inbound_event, run_stdio_loop, RuntimeOutboundEvent};
use std::path::Path;
use telegram_adapter::{
    build_inline_keyboard, is_group_chat, resolve_bot_token, route_telegram_update,
    run_live_session, HttpTelegramApi, TelegramInlineButton, TelegramLiveConfig,
    TelegramLiveOutcome, TelegramRouteDecision,
};

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
    Channels {
        #[command(subcommand)]
        command: ChannelsCommands,
    },
    Discord {
        #[command(subcommand)]
        command: DiscordCommands,
    },
    Telegram {
        #[command(subcommand)]
        command: TelegramCommands,
    },
    Slack {
        #[command(subcommand)]
        command: SlackCommands,
    },
    Run {
        #[command(subcommand)]
        command: RunCommands,
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
enum ChannelsCommands {
    RouteEvent {
        #[arg(long)]
        platform: String,
        #[arg(long)]
        payload_json: String,
        #[arg(long)]
        identity: Option<String>,
    },
}

#[derive(Subcommand)]
enum DiscordCommands {
    RouteEvent {
        #[arg(long)]
        payload_json: String,
    },
    LiveEvent {
        #[arg(long)]
        payload_json: String,
        #[arg(long)]
        bot_token: Option<String>,
        #[arg(long, default_value = "DISCORD_BOT_TOKEN")]
        bot_token_env: String,
        #[arg(long, value_enum, default_value_t = setup_wizard::Template::SreSquad)]
        template: setup_wizard::Template,
    },
    BuildEmbed {
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: String,
    },
    Authorize {
        #[arg(long)]
        required_role: String,
        #[arg(long)]
        roles_json: String,
    },
}

#[derive(Subcommand)]
enum TelegramCommands {
    RouteEvent {
        #[arg(long)]
        payload_json: String,
        #[arg(long)]
        bot_username: String,
    },
    Live {
        #[arg(long)]
        bot_username: String,
        #[arg(long)]
        bot_token: Option<String>,
        #[arg(long, default_value = "TELEGRAM_BOT_TOKEN")]
        bot_token_env: String,
        #[arg(long, value_enum, default_value_t = setup_wizard::Template::SreSquad)]
        template: setup_wizard::Template,
        #[arg(long, default_value_t = 10)]
        poll_timeout_seconds: u16,
        #[arg(long)]
        max_updates: Option<usize>,
    },
    BuildKeyboard {
        #[arg(long)]
        buttons_json: String,
    },
    ChatSupport {
        #[arg(long)]
        chat_type: String,
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
    LiveEvent {
        #[arg(long)]
        bot_user_id: String,
        #[arg(long)]
        payload_json: String,
        #[arg(long)]
        bot_token: Option<String>,
        #[arg(long, default_value = "SLACK_BOT_TOKEN")]
        bot_token_env: String,
        #[arg(long, value_enum, default_value_t = setup_wizard::Template::SreSquad)]
        template: setup_wizard::Template,
    },
}

#[derive(Subcommand)]
enum RunCommands {
    RouteEvent {
        #[arg(long)]
        platform: String,
        #[arg(long)]
        payload_json: String,
        #[arg(long)]
        identity: Option<String>,
        #[arg(long, value_enum, default_value_t = setup_wizard::Template::SreSquad)]
        template: setup_wizard::Template,
    },
    Stdio {
        #[arg(long, value_enum, default_value_t = setup_wizard::Template::SreSquad)]
        template: setup_wizard::Template,
        #[arg(long)]
        max_events: Option<usize>,
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
        Some(Commands::Channels { command }) => match command {
            ChannelsCommands::RouteEvent {
                platform,
                payload_json,
                identity,
            } => {
                let parsed_platform = match ChannelPlatform::parse(platform.as_str()) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("channels route-event failed: {err}");
                        std::process::exit(1);
                    }
                };

                match route_platform_event(
                    parsed_platform,
                    payload_json.as_str(),
                    identity.as_deref(),
                ) {
                    Ok(decision) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&channels_route_to_json(decision))
                                .expect("channels route output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("channels route-event failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
        },
        Some(Commands::Discord { command }) => match command {
            DiscordCommands::RouteEvent { payload_json } => {
                match route_discord_payload(payload_json.as_str()) {
                    Ok(decision) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&discord_route_to_json(decision))
                                .expect("discord route output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("discord route-event failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
            DiscordCommands::LiveEvent {
                payload_json,
                bot_token,
                bot_token_env,
                template,
            } => {
                let resolved_token = match resolve_discord_bot_token(
                    bot_token.as_deref(),
                    Some(bot_token_env.as_str()),
                    "DISCORD_BOT_TOKEN",
                ) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("discord live-event failed: {err}");
                        std::process::exit(1);
                    }
                };

                let mut api = match HttpDiscordApi::new(resolved_token) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("discord live-event failed: {err}");
                        std::process::exit(1);
                    }
                };

                match handle_discord_live_event(
                    &mut api,
                    payload_json.as_str(),
                    template_slug(&template),
                ) {
                    Ok(decision) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&discord_live_decision_to_json(decision))
                                .expect("discord live-event output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("discord live-event failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
            DiscordCommands::BuildEmbed { title, description } => {
                match build_embed(title.as_str(), description.as_str()) {
                    Ok(embed) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&embed)
                                .expect("discord embed output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("discord build-embed failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
            DiscordCommands::Authorize {
                required_role,
                roles_json,
            } => {
                let roles: Vec<String> = match serde_json::from_str(roles_json.as_str()) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("discord authorize failed: invalid roles_json: {err}");
                        std::process::exit(1);
                    }
                };

                let authorized = is_role_authorized(required_role.as_str(), &roles);
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "required_role": required_role,
                        "authorized": authorized
                    }))
                    .expect("discord authorize output serialization should succeed")
                );
            }
        },
        Some(Commands::Telegram { command }) => match command {
            TelegramCommands::RouteEvent {
                payload_json,
                bot_username,
            } => match route_telegram_update(payload_json.as_str(), bot_username.as_str()) {
                Ok(decision) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&telegram_route_to_json(decision))
                            .expect("telegram route output serialization should succeed")
                    );
                }
                Err(err) => {
                    eprintln!("telegram route-event failed: {err}");
                    std::process::exit(1);
                }
            },
            TelegramCommands::Live {
                bot_username,
                bot_token,
                bot_token_env,
                template,
                poll_timeout_seconds,
                max_updates,
            } => {
                let resolved_token = match resolve_bot_token(
                    bot_token.as_deref(),
                    Some(bot_token_env.as_str()),
                    "TELEGRAM_BOT_TOKEN",
                ) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("telegram live failed: {err}");
                        std::process::exit(1);
                    }
                };

                let mut api = match HttpTelegramApi::new(resolved_token) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("telegram live failed: {err}");
                        std::process::exit(1);
                    }
                };

                let config = TelegramLiveConfig {
                    bot_username,
                    template: template_slug(&template).to_string(),
                    max_updates,
                    poll_timeout_seconds,
                };

                eprintln!(
                    "telegram live: polling started (template={}, max_updates={})",
                    config.template,
                    config
                        .max_updates
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "unbounded".to_string())
                );

                match run_live_session(&mut api, &config) {
                    Ok(outcome) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&telegram_live_to_json(outcome))
                                .expect("telegram live output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("telegram live failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
            TelegramCommands::BuildKeyboard { buttons_json } => {
                let buttons: Vec<Vec<TelegramInlineButton>> =
                    match serde_json::from_str(buttons_json.as_str()) {
                        Ok(value) => value,
                        Err(err) => {
                            eprintln!(
                                "telegram build-keyboard failed: invalid buttons_json: {err}"
                            );
                            std::process::exit(1);
                        }
                    };

                match build_inline_keyboard(buttons) {
                    Ok(keyboard) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&keyboard)
                                .expect("telegram keyboard output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("telegram build-keyboard failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
            TelegramCommands::ChatSupport { chat_type } => {
                let group_supported = is_group_chat(chat_type.as_str());
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "chat_type": chat_type,
                        "group_supported": group_supported
                    }))
                    .expect("telegram chat-support output serialization should succeed")
                );
            }
        },
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
            } => {
                match prepare_response_for_slack(text.as_str(), max_chars, snippet_name.as_str()) {
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
                }
            }
            SlackCommands::LiveEvent {
                bot_user_id,
                payload_json,
                bot_token,
                bot_token_env,
                template,
            } => {
                let resolved_token = match resolve_slack_bot_token(
                    bot_token.as_deref(),
                    Some(bot_token_env.as_str()),
                    "SLACK_BOT_TOKEN",
                ) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("slack live-event failed: {err}");
                        std::process::exit(1);
                    }
                };

                let mut api = match HttpSlackApi::new(resolved_token) {
                    Ok(value) => value,
                    Err(err) => {
                        eprintln!("slack live-event failed: {err}");
                        std::process::exit(1);
                    }
                };

                match handle_live_event(
                    &mut api,
                    payload_json.as_str(),
                    bot_user_id.as_str(),
                    template_slug(&template),
                ) {
                    Ok(decision) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&slack_live_decision_to_json(decision))
                                .expect("slack live-event output serialization should succeed")
                        );
                    }
                    Err(err) => {
                        eprintln!("slack live-event failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
        },
        Some(Commands::Run { command }) => match command {
            RunCommands::RouteEvent {
                platform,
                payload_json,
                identity,
                template,
            } => match process_inbound_event(
                platform.as_str(),
                payload_json.as_str(),
                identity.as_deref(),
                template_slug(&template),
            ) {
                Ok(Some(route)) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&runtime_route_to_json(route))
                            .expect("runtime route output serialization should succeed")
                    );
                }
                Ok(None) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&serde_json::json!({
                            "decision": "ignore"
                        }))
                        .expect("runtime ignore output serialization should succeed")
                    );
                }
                Err(err) => {
                    eprintln!("run route-event failed: {err}");
                    std::process::exit(1);
                }
            },
            RunCommands::Stdio {
                template,
                max_events,
            } => {
                let stdin = std::io::stdin();
                let stdout = std::io::stdout();
                let mut writer = stdout.lock();
                let reader = stdin.lock();

                match run_stdio_loop(reader, &mut writer, template_slug(&template), max_events) {
                    Ok(outcome) => {
                        eprintln!(
                            "run stdio: events_processed={} responses_emitted={}",
                            outcome.events_processed, outcome.responses_emitted
                        );
                    }
                    Err(err) => {
                        eprintln!("run stdio failed: {err}");
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

fn discord_route_to_json(route: DiscordRouteDecision) -> serde_json::Value {
    match route {
        DiscordRouteDecision::SlashCommand(command) => serde_json::json!({
            "decision": "slash_command",
            "command_name": command.command_name,
            "roles": command.roles,
            "channel_id": command.channel_id
        }),
        DiscordRouteDecision::Ignore => serde_json::json!({
            "decision": "ignore"
        }),
    }
}

fn discord_live_decision_to_json(decision: DiscordLiveDecision) -> serde_json::Value {
    match decision {
        DiscordLiveDecision::Posted { channel_id, text } => serde_json::json!({
            "decision": "posted",
            "channel_id": channel_id,
            "text": text
        }),
        DiscordLiveDecision::Ignore => serde_json::json!({
            "decision": "ignore"
        }),
    }
}

fn telegram_route_to_json(route: TelegramRouteDecision) -> serde_json::Value {
    match route {
        TelegramRouteDecision::Command(command) => serde_json::json!({
            "decision": "command",
            "chat_id": command.chat_id,
            "chat_type": command.chat_type,
            "command_name": command.command_name,
            "text": command.text,
            "is_group": command.is_group
        }),
        TelegramRouteDecision::Ignore => serde_json::json!({
            "decision": "ignore"
        }),
    }
}

fn channels_route_to_json(route: ChannelRouteDecision) -> serde_json::Value {
    match route {
        ChannelRouteDecision::Routed(event) => serde_json::json!({
            "decision": "routed",
            "platform": event.platform,
            "route_kind": event.route_kind,
            "target_ref": event.target_ref,
            "text": event.text
        }),
        ChannelRouteDecision::Ignore => serde_json::json!({
            "decision": "ignore"
        }),
    }
}

fn runtime_route_to_json(route: RuntimeOutboundEvent) -> serde_json::Value {
    serde_json::json!({
        "decision": "routed",
        "platform": route.platform,
        "target_ref": route.target_ref,
        "route_kind": route.route_kind,
        "text": route.text
    })
}

fn template_slug(template: &setup_wizard::Template) -> &'static str {
    match template {
        setup_wizard::Template::SreSquad => "sre-squad",
        setup_wizard::Template::DevOpsTeam => "dev-ops-team",
        setup_wizard::Template::IncidentResponse => "incident-response",
    }
}

fn telegram_live_to_json(outcome: TelegramLiveOutcome) -> serde_json::Value {
    serde_json::json!({
        "updates_processed": outcome.updates_processed,
        "replies_sent": outcome.replies_sent,
        "last_update_id": outcome.last_update_id
    })
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

fn slack_live_decision_to_json(decision: SlackLiveDecision) -> serde_json::Value {
    match decision {
        SlackLiveDecision::UrlVerification { challenge } => serde_json::json!({
            "decision": "url_verification",
            "challenge": challenge
        }),
        SlackLiveDecision::Replied {
            channel,
            thread_ts,
            text,
        } => serde_json::json!({
            "decision": "replied",
            "channel": channel,
            "thread_ts": thread_ts,
            "text": text
        }),
        SlackLiveDecision::Ignore => serde_json::json!({
            "decision": "ignore"
        }),
    }
}
