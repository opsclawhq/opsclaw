# Telegram Live Squad Guide (`opsclaw telegram`)

Phase 5 `05-07` + `05-15` adds a live Telegram polling runtime plus setup verification so you can connect a real bot and validate credentials before chatting with an OpsClaw squad.

## 1. Create and Configure a Telegram Bot

1. Open Telegram and message [@BotFather](https://t.me/BotFather).
2. Run `/newbot`, choose a bot name and username.
3. Copy the generated bot token.
4. Optional but recommended: in BotFather run `/setprivacy` and disable privacy mode if you want richer group-chat behavior.

## 2. Verify Bot Credentials and Connectivity

```bash
export TELEGRAM_BOT_TOKEN="<your-bot-token>"

cargo run -p opsclaw -- telegram verify \
  --expected-bot-username "<your-bot-username-without-@>" \
  --template sre-squad
```

Optional connectivity ping to a chat you have already opened with the bot:

```bash
cargo run -p opsclaw -- telegram verify \
  --expected-bot-username "<your-bot-username-without-@>" \
  --ping-chat-id <numeric-chat-id> \
  --template sre-squad
```

## 3. Start OpsClaw Live Loop

```bash
cargo run -p opsclaw -- telegram live \
  --bot-username "<your-bot-username-without-@>" \
  --template sre-squad
```

Alternate token source:

```bash
cargo run -p opsclaw -- telegram live \
  --bot-username "<bot-username>" \
  --bot-token "<token>" \
  --template dev-ops-team
```

For smoke tests or CI, bound the loop:

```bash
cargo run -p opsclaw -- telegram live \
  --bot-username "<bot-username>" \
  --bot-token "<token>" \
  --max-updates 10
```

## 4. Talk to the Squad

- In a private chat with the bot:
  - `/start` for intro
  - `/help` for commands
  - `/squad` for active members
  - free text for routed squad response
- In group chats:
  - mention `@<bot-username>` in your message
  - slash commands can target bot username (example: `/squad@<bot-username>`)

## 5. Contract Helpers (Still Available)

```bash
cargo run -p opsclaw -- telegram route-event --payload-json '{"message":{"chat":{"id":42,"type":"group"},"text":"/status@opsclaw_bot"}}' --bot-username opsclaw_bot
cargo run -p opsclaw -- telegram build-keyboard --buttons-json '[[{"text":"Approve","callback_data":"approve:run-1"}]]'
cargo run -p opsclaw -- telegram chat-support --chat-type supergroup
```

## Notes

- Current runtime transport is long polling (`getUpdates`) for local/self-hosted simplicity.
- `--max-updates` is useful for deterministic test runs; omit it for continuous operation.
- Telegram bot creation still happens in BotFather; `telegram verify` validates that your created bot credentials are usable from OpsClaw.
