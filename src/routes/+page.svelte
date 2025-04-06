<script lang="ts">
  import Timeline from "@/components/app/timeline/timeline.svelte";
  import type { Habit } from "@/types/bindings";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let habits: Array<Habit> | [] = $state([]);

  onMount(async () => {
    try {
      const response = await invoke<Array<Habit>>("get_all_habits");
      habits = response;
    } catch (e) {
      console.log(e);
    }
  });
</script>

<div class="space-y-8">
  {#each habits as habit}
    <Timeline {habit} />
  {/each}
</div>
