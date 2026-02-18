import React from "react";

interface EconomicsPanelProps {
  rows: Array<{
    agent_id: string;
    tokens_consumed: number;
    api_spend_usd: number;
    incidents_handled: number;
    minutes_saved: number;
  }>;
  summary: {
    total_spend_usd: number;
    total_minutes_saved: number;
    estimated_value_usd: number;
    roi_ratio: number;
  };
}

export function EconomicsPanel({ rows, summary }: EconomicsPanelProps) {
  return (
    <section className="panel panel-wide">
      <h2>Economics and ROI</h2>
      <div className="economics-summary">
        <p>Total spend: <strong>${summary.total_spend_usd}</strong></p>
        <p>Minutes saved: <strong>{summary.total_minutes_saved}</strong></p>
        <p>Estimated value: <strong>${summary.estimated_value_usd}</strong></p>
        <p>ROI ratio: <strong>{summary.roi_ratio}x</strong></p>
      </div>
      <table className="economics-table">
        <thead>
          <tr>
            <th>Agent</th>
            <th>Tokens</th>
            <th>Spend (USD)</th>
            <th>Incidents</th>
            <th>Minutes saved</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((row) => (
            <tr key={row.agent_id}>
              <td>{row.agent_id}</td>
              <td>{row.tokens_consumed}</td>
              <td>${row.api_spend_usd}</td>
              <td>{row.incidents_handled}</td>
              <td>{row.minutes_saved}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}
