export interface LastFmSessionResponse {
	session?: LastFmSession;
	error?: number;
	message?: string;
}

export interface LastFmSession {
	name: string;
	key: string;
}

export interface LastFmRecentTracksResponse {
	recenttracks: RecentTracks;
}

export interface RecentTracks {
	track: Track | Track[];
}

export interface Track {
	name: string;
	artist: Artist;
	"@attr"?: TrackAttr;
	mbid?: string;
	album?: Album;
	image: Image[];
	streamable: string;
	url: string;
	date?: Date;
}

export interface Album {
	mbid?: string;
	"#text"?: string;
	image?: Image[];
	artist?: string;
	title?: string;
	url?: string;
}

export interface Image {
	size: ImageSizes;
	"#text": string;
}

export type ImageSizes = "small" | "large" | "extralarge";

export interface Artist {
	mbid?: string;
	"#text": string;
}

export interface ArtistInfo {
	image?: Image[];
}

export interface TrackAttr {
	nowplaying?: string;
}

export interface Date {
	uts: string;
	"#text": string;
}

export interface LastFmArtistInfoResponse {
	artist: ArtistInfo;
}

export interface LastFmTrackInfoResponse {
	track: TrackInfo;
}

export interface TrackInfo {
	duration: string;
	playcount: string;
	userplaycount: string;
	name: string;
	artist: Artist | { name: string; url: string };
	album?: {
		artist?: string;
		title?: string;
		url?: string;
		image?: Image[];
	};
}

export interface LastFmUserInfoResponse {
	user: UserInfo;
}

export interface UserInfo {
	name: string;
	realname: string;
	playcount: string;
	artist_count: string;
	album_count: string;
	country: string;
	url: string;
	image: Image[];
}

export interface WeeklyTrackChartResponse {
	weeklytrackchart: WeeklyTrackChart;
}

export interface WeeklyTrackChart {
	track?: WeeklyTrack[];
}

export interface WeeklyTrack {
	name: string;
	playcount: string;
	artist: { "#text": string };
}
