import React from "react";

interface ConversationViewerPanelProps {
  conversations: Array<{ conversation_id: string; title: string }>;
  selectedConversationId: string;
  onSelectConversation: (conversationId: string) => void;
  transcript: {
    conversation_id: string;
    title: string;
    entries: Array<{
      kind: string;
      actor: string;
      text?: string;
      command?: string;
      output?: string;
      occurred_at: string;
    }>;
  } | null;
}

export function ConversationViewerPanel({
  conversations,
  selectedConversationId,
  onSelectConversation,
  transcript
}: ConversationViewerPanelProps) {
  return (
    <section className="panel panel-wide">
      <h2>Conversation Viewer</h2>
      <label>
        Conversation:
        <select
          value={selectedConversationId}
          onChange={(event) => onSelectConversation(event.target.value)}
          className="conversation-select"
        >
          {conversations.map((item) => (
            <option key={item.conversation_id} value={item.conversation_id}>{item.title}</option>
          ))}
        </select>
      </label>

      {!transcript ? <p className="subtle">Conversation not found.</p> : null}

      {transcript ? (
        <ul className="transcript-list">
          {transcript.entries.map((entry, index) => (
            <li key={`${entry.occurred_at}-${index}`} className="transcript-entry">
              <p className="subtle">{entry.occurred_at} · {entry.actor} · {entry.kind}</p>
              {entry.kind === "tool_call" ? (
                <>
                  <p><code>{entry.command}</code></p>
                  <pre>{entry.output}</pre>
                </>
              ) : (
                <p>{entry.text}</p>
              )}
            </li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}
