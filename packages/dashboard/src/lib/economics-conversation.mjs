function roundToTwo(value) {
  return Math.round(value * 100) / 100;
}

export function buildEconomicsRows(snapshot) {
  const agents = snapshot?.agents ?? [];
  return agents.map((agent) => ({
    agent_id: agent.agent_id,
    tokens_consumed: agent.tokens_consumed,
    api_spend_usd: roundToTwo(agent.api_spend_usd),
    incidents_handled: agent.incidents_handled,
    minutes_saved: agent.minutes_saved
  }));
}

export function buildRoiSummary(snapshot, valuePerHourUsd = 120) {
  const rows = buildEconomicsRows(snapshot);
  const total_spend_usd = roundToTwo(rows.reduce((sum, row) => sum + row.api_spend_usd, 0));
  const total_minutes_saved = rows.reduce((sum, row) => sum + row.minutes_saved, 0);
  const estimated_value_usd = roundToTwo((total_minutes_saved / 60) * valuePerHourUsd);
  const roi_ratio = total_spend_usd === 0 ? 0 : roundToTwo(estimated_value_usd / total_spend_usd);

  return {
    total_spend_usd,
    total_minutes_saved,
    estimated_value_usd,
    roi_ratio
  };
}

export function buildConversationTranscript(conversationId, conversations) {
  const source = conversations.find((item) => item.conversation_id === conversationId);
  if (!source) {
    return null;
  }

  const entries = [...source.entries].sort(
    (a, b) => new Date(a.occurred_at).getTime() - new Date(b.occurred_at).getTime()
  );

  return {
    conversation_id: source.conversation_id,
    title: source.title,
    entries
  };
}
