<script lang="ts">
  import Timeline from "@/components/app/timeline/timeline.svelte";
  import type { Habit } from "@/types/bindings";
  import { invoke } from "@tauri-apps/api/core";
  import { tv } from "tailwind-variants";
  import AddHabit from "@/components/app/statusbar/addHabit.svelte";
  import { createQuery } from "@tanstack/svelte-query";

  const query = createQuery<Array<Habit>>({
    queryKey: ["all_habits"],
    queryFn: () => invoke("get_all_habits"),
  });

  const styles = tv({
    slots: {
      taskbar: "flex-center mb-4 justify-end",
      taskbarStats: "text-muted-foreground text-xs",
      timelines: "space-y-8",
    },
  });
  const { taskbar, taskbarStats, timelines } = styles();
</script>

<div class={taskbar()}>
  <div>
    <AddHabit />
  </div>
</div>
<div class={timelines()}>
  {#if $query.isSuccess}
    {#each $query.data as habit}
      <Timeline {habit} />
    {/each}
  {/if}
</div>
