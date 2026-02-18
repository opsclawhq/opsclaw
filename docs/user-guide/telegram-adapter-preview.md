# Telegram Live Squad Guide (`opsclaw telegram`)

Phase 5 `05-06` adds a live Telegram polling runtime so you can connect a real bot and chat with an OpsClaw squad experience.

## 1. Create and Configure a Telegram Bot

1. Open Telegram and message [@BotFather](https://t.me/BotFather).
2. Run `/newbot`, choose a bot name and username.
3. Copy the generated bot token.
4. Optional but recommended: in BotFather run `/setprivacy` and disable privacy mode if you want richer group-chat behavior.

## 2. Export Token and Start OpsClaw Live Loop

```bash
export TELEGRAM_BOT_TOKEN="<your-bot-token>"

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

## 3. Talk to the Squad

- In a private chat with the bot:
  - `/start` for intro
  - `/help` for commands
  - `/squad` for active members
  - free text for routed squad response
- In group chats:
  - mention `@<bot-username>` in your message
  - slash commands can target bot username (example: `/squad@<bot-username>`)

## 4. Contract Helpers (Still Available)

```bash
cargo run -p opsclaw -- telegram route-event --payload-json '{"message":{"chat":{"id":42,"type":"group"},"text":"/status@opsclaw_bot"}}' --bot-username opsclaw_bot
cargo run -p opsclaw -- telegram build-keyboard --buttons-json '[[{"text":"Approve","callback_data":"approve:run-1"}]]'
cargo run -p opsclaw -- telegram chat-support --chat-type supergroup
```

## Notes

- Current runtime transport is long polling (`getUpdates`) for local/self-hosted simplicity.
- `--max-updates` is useful for deterministic test runs; omit it for continuous operation.
