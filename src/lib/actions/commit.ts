import type { Commit } from "@/types/bindings";
import { invoke } from "@tauri-apps/api/core";

export async function getHabitCommits(habitID: number | null) {
  try {
    const response = await invoke<Array<Commit>>("get_habit_commits", { habitId: habitID });
    return response;
  } catch (e) {
    console.log(e);
    return [];
  }
}

export async function createCommit(habitID: number) {
  try {
    await invoke<Commit>("create_commit", { habit_id: habitID });
  } catch (e) {
    console.log(e);
  }
}
