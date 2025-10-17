import { truncateText } from "@repo/common/utils";
import type { Track } from "@repo/lastfm";
import {
  MediaGallery,
  MediaGalleryItem,
  Section,
  Separator,
  TextDisplay,
  Thumbnail,
} from "seyfert";
import { Spacing } from "seyfert/lib/types";

export interface TrackPlaysData {
  track: Track;
  username: string;
  artistName: string;
  playcount: string;
  userplaycount: string;
  largeImage: string;
}

function createCompactView({
  track,
  username,
  artistName,
  playcount,
  userplaycount,
  largeImage,
}: TrackPlaysData) {
  return [
    new TextDisplay().setContent(
      `**[${truncateText(track.name, 40)}](${track.url})**`,
    ),
    new TextDisplay().setContent(
      `-# ${truncateText(artistName || "", 30)} - ${truncateText(
        track.album?.["#text"] || "Unknown Album",
        50,
      )}`,
    ),
    new MediaGallery().addItems(
      new MediaGalleryItem().setMedia(largeImage || ""),
    ),
    new Separator().setDivider(true).setSpacing(Spacing.Small),
    new TextDisplay().setContent(
      `-# **${username}** plays: \`${userplaycount}\` | global: \`${playcount}\``,
    ),
  ];
}

function createDetailedView({
  track,
  username,
  artistName,
  playcount,
  userplaycount,
  largeImage,
}: TrackPlaysData) {
  return [
    new Section()
      .setComponents([
        new TextDisplay().setContent(
          `### [${truncateText(track.name, 70)}](${track.url})`,
        ),
        new TextDisplay().setContent(
          `-# ${truncateText(artistName || "", 60)}`,
        ),
        new TextDisplay().setContent(
          `-# **${truncateText(track.album?.["#text"] || "Unknown Album", 60)}**`,
        ),
      ])
      .setAccessory(new Thumbnail().setMedia(largeImage || "")),
    new Separator().setDivider(true).setSpacing(Spacing.Small),
    new TextDisplay().setContent(
      `-# **${username}** plays: \`${userplaycount}\` · Global plays: \`${playcount}\``,
    ),
  ];
}

function createMinimalView({
  track,
  username,
  artistName,
  playcount,
  userplaycount,
}: TrackPlaysData) {
  return [
    new TextDisplay().setContent(
      `**${username}** has \`${userplaycount}\` plays for **[${truncateText(track.name, 50)}](${track.url})** by **${truncateText(artistName, 40)}** \n-# (${playcount} global)`,
    ),
  ];
}

const styleBuilders = {
  Compact: createCompactView,
  Detailed: createDetailedView,
  Minimal: createMinimalView,
};

export function buildTrackPlaysView(
  style: "Compact" | "Detailed" | "Minimal",
  data: TrackPlaysData,
) {
  // If the style does not exist, use 'Detailed' as fallback.
  const builder = styleBuilders[style] || styleBuilders.Detailed;
  return builder(data);
}
