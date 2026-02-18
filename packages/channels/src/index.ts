export type ChannelPlatform = "slack" | "discord" | "telegram";

export interface ChannelEvent {
  platform: ChannelPlatform;
  channelId: string;
  text: string;
}

export interface RoutedEvent {
  routeKey: string;
  event: ChannelEvent;
}

export function routeChannelEvent(event: ChannelEvent): RoutedEvent {
  return {
    routeKey: `${event.platform}:${event.channelId}`,
    event,
  };
}
