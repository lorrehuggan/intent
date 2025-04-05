export type WeekDay = "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat" | "Sun";

export interface TimelineYear {
  nodes: Record<WeekDay, Array<string>>;
}
