<script lang="ts">
  import { getHabitCommits } from "@/actions/commit";
  import { createYearTimeline } from "@/actions/timeline";
  import Button from "@/components/ui/button/button.svelte";
  import type { Commit, Habit } from "@/types/bindings";
  import { convertDateNoTime } from "@/utils";
  import { CheckIcon, TriangleAlertIcon } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import clsx from "clsx";
  import dayjs from "dayjs";
  import { onMount } from "svelte";
  import { tv } from "tailwind-variants";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import Edit from "./edit.svelte";

  const html = document.querySelector("html");

  let { habit }: { habit: Habit } = $props();

  let yearTimeline = $state<Array<Array<string>>>([]);
  let commits = $state<Array<Commit>>([]);
  let committedToday = $state(false);
  let today = dayjs().format("YYYY-MM-DD");

  onMount(async () => {
    commits = await getHabitCommits(habit.id);
    yearTimeline = await createYearTimeline();

    committedToday = Boolean(
      commits?.find((commit) => convertDateNoTime(commit.completionDate) === today),
    );
  });

  async function toggleTodaysCommit() {
    switch (committedToday) {
      case false:
        committedToday = true;
        await invoke("create_commit", { habitId: habit.id, completed: true });
        break;
      case true:
        committedToday = false;
        commits = await getHabitCommits(habit.id);
        const todaysCommit = commits?.find(
          (commit) => convertDateNoTime(commit.completionDate) === today,
        );
        await invoke("delete_commit", { commitId: todaysCommit?.id });
        break;
    }
  }

  function habitTheme() {
    switch (habit.theme) {
      case "red":
        return "bg-red-400";
      case "green":
        return "bg-emerald-400";
      case "blue":
        return "bg-blue-400";
      case "rose":
        return "bg-rose-400";
      case "mint":
        return "bg-lime-400";
      case "sky":
        return "bg-sky-400";
      case "amber":
        return "bg-amber-400";
      case "indigo":
        return "bg-indigo-400";
      case "neutral":
        return "bg-neutral-400";
    }
  }

  const styles = tv({
    slots: {
      container: "",
      header: "flex-center mb-1 w-full justify-between rounded-lg py-2",
      headerDesc: "text-muted-foreground text-xs",
      week: "flex-center mb-[2px] gap-[2px]",
      node: "size-[13px] rounded",
    },
    variants: {
      status: {
        completed: {},
        incomplete: {},
      },
    },
  });

  const { header, headerDesc, week } = styles();
</script>

<!-- NODE -->
{#snippet tlnode(day: string, commit: Commit | undefined)}
  {@const completed = commit?.completed}
  {@const isNodeToday = convertDateNoTime(day) === convertDateNoTime(today)}
  {@const highlightCurrentDay = html?.getAttribute("data-highlight-current-day") === "true"}
  <div
    data-date={convertDateNoTime(day)}
    class={clsx("size-[13px] rounded", {
      [habitTheme()]: committedToday && isNodeToday,
      "bg-foreground/40": !committedToday && isNodeToday && highlightCurrentDay,
      [`${habitTheme()} opacity-75`]: completed && !isNodeToday,
      [`${habitTheme()} opacity-25`]:
        (!completed && !isNodeToday) || (!committedToday && isNodeToday && !highlightCurrentDay),
    })}
  ></div>
{/snippet}

<!-- WEEK -->
{#snippet tlweek(days: Array<string>)}
  <div class={week()}>
    {#each days as day}
      {@const commit = commits?.find(
        (commit) => convertDateNoTime(commit.completionDate) === convertDateNoTime(day),
      )}
      {@render tlnode(day, commit)}
    {/each}
  </div>
{/snippet}

<!-- TIMELINE -->
<div class="mx-auto max-w-[772px]">
  <div class={header()}>
    <div>
      <Edit {habit} />
      <p class={headerDesc()}>{habit.description}</p>
    </div>
    <div>
      <Button
        onclick={() => toggleTodaysCommit()}
        class={clsx(`text-foreground transition-opacity duration-300 hover:${habitTheme()}`, {
          [`${habitTheme()} hover:opacity-80`]: committedToday === true,
          [`${habitTheme()} opacity-70 hover:opacity-80`]: committedToday === false,
        })}
        size="icon"
      >
        {#if committedToday}
          <CheckIcon size={16} />
        {:else}
          <TriangleAlertIcon size={16} />
        {/if}
      </Button>
    </div>
  </div>

  {#each yearTimeline as weekday}
    {@render tlweek(weekday)}
  {/each}
</div>
