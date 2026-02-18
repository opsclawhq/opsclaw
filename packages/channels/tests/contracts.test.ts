import { describe, expect, test } from "bun:test";
import { routeChannelEvent } from "../src/index";

describe("routeChannelEvent", () => {
  test("builds deterministic route keys", () => {
    const routed = routeChannelEvent({
      platform: "discord",
      channelId: "ops-war-room",
      text: "@bot investigate latency",
    });

    expect(routed.routeKey).toBe("discord:ops-war-room");
    expect(routed.event.text).toContain("investigate");
  });
});
