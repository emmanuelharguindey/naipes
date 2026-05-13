export const AVAILABLE_GAMES = ["brisca"] as const;
export type AvailableGame = (typeof AVAILABLE_GAMES)[number];
