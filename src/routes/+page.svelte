<script lang="ts">
  import Timeline from "@/components/app/timeline/timeline.svelte";
  import type { Habit } from "@/types/bindings";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { tv } from "tailwind-variants";
  import AddHabit from "@/components/app/statusbar/addHabit.svelte";
  import * as Select from "$lib/components/ui/select/index.js";

  let habits: Array<Habit> | [] = $state([]);
  let streakOpen = $state(false);

  onMount(async () => {
    try {
      const response = await invoke<Array<Habit>>("get_all_habits");
      habits = response;
    } catch (e) {
      console.log(e);
    }
  });

  const styles = tv({
    slots: {
      taskbar: "flex-center mb-4 justify-between",
      taskbarStats: "text-muted-foreground text-xs",
      timelines: "space-y-8",
    },
  });
  const { taskbar, taskbarStats, timelines } = styles();
</script>

<div class={taskbar()}>
  <div class="flex-center gap-1">
    <p class={taskbarStats()}>{`Active habits: ${habits.length}`}</p>
    <p class={taskbarStats()}>Stacked habits: 0</p>
  </div>
  <div>
    <AddHabit />
  </div>
</div>
<div class={timelines()}>
  {#each habits as habit}
    <Timeline {habit} />
  {/each}
</div>
