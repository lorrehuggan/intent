import type { YearTimeline } from "@/types/bindings";
import { invoke } from "@tauri-apps/api/core";

export async function createYearTimeline() {
  const yearTimeline = [];
  const year = await invoke<YearTimeline>("create_year_timeline");
  const nodes = year.nodes;
  if (
    !nodes.Sun ||
    !nodes.Mon ||
    !nodes.Tue ||
    !nodes.Wed ||
    !nodes.Thu ||
    !nodes.Fri ||
    !nodes.Sat
  )
    return [];

  yearTimeline.push(nodes.Sun);
  yearTimeline.push(nodes.Mon);
  yearTimeline.push(nodes.Tue);
  yearTimeline.push(nodes.Wed);
  yearTimeline.push(nodes.Thu);
  yearTimeline.push(nodes.Fri);
  yearTimeline.push(nodes.Sat);

  return yearTimeline;
}
