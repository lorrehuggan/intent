<script lang="ts">
  import { getHabitCommits } from "@/actions/commit";
  import { createYearTimeline } from "@/actions/timeline";
  import Button from "@/components/ui/button/button.svelte";
  import type { Commit, Habit } from "@/types/bindings";
  import { convertDateNoTime } from "@/utils";
  import { CheckIcon } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import clsx from "clsx";
  import dayjs from "dayjs";
  import { onMount } from "svelte";
  import { tv } from "tailwind-variants";

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

  const styles = tv({
    slots: {
      container: "",
      header: "flex-center mb-2 w-full justify-between rounded-lg py-2",
      headerTitle: "text-sm font-bold",
      headerDesc: "text-muted-foreground text-xs",
      week: "flex-center mb-[2px] gap-[2px]",
      node: "size-[13px] rounded",
      commitButton: "text-background",
    },
    variants: {
      status: {
        completed: {
          commitButton: "bg-primary",
        },
        incomplete: {
          commitButton: "bg-muted-foreground",
        },
      },
    },
  });

  const { header, headerTitle, headerDesc, week, commitButton } = styles();
</script>

<!-- NODE -->
{#snippet tlnode(day: string, commit: Commit | undefined)}
  {@const completed = commit?.completed}
  {@const isNodeToday = convertDateNoTime(day) === convertDateNoTime(today)}
  <div
    data-date={convertDateNoTime(day)}
    class={clsx("size-[13px] rounded", {
      "bg-primary cursor-pointer": committedToday && isNodeToday,
      "bg-primary/50": !committedToday && isNodeToday,
      "bg-primary/75 cursor-pointer": completed && !isNodeToday,
      "bg-primary/25": !completed && !isNodeToday,
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
<div class="">
  <div class={header()}>
    <div>
      <p class={headerTitle()}>{habit.title}</p>
      <p class={headerDesc()}>{habit.description}</p>
    </div>
    <div>
      <Button
        onclick={() => toggleTodaysCommit()}
        class={commitButton({
          status: committedToday ? "completed" : "incomplete",
        })}
        size="icon"
      >
        <CheckIcon size={16} />
      </Button>
    </div>
  </div>

  {#each yearTimeline as weekday}
    {@render tlweek(weekday)}
  {/each}
</div>
