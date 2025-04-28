<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Button } from "@/components/ui/button";
  import { queryClient } from "@/context/query";
  import type {
    Habit,
    HabitCategory,
    HabitReminder,
    HabitStreak,
    HabitTheme,
  } from "@/types/bindings";
  import { Box, CalendarFold, Clock, PaintRoller } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type } from "arktype";
  import clsx from "clsx";
  import SuperDebug, { defaults, superForm, type Infer } from "sveltekit-superforms";
  import { arktype } from "sveltekit-superforms/adapters";
  import { tv } from "tailwind-variants";

  let defaultForm = {
    title: "",
    description: "",
    streak: "daily",
    category: "other",
    color: "green",
    reminder: "no reminder",
  };

  const schema = type({
    title: "2 <= string <= 32",
    description: "2 <= string <= 64",
    streak: "string",
    category: "string",
    color: "string",
    reminder: "string",
  });

  const data = defaults(arktype(schema, { defaults: defaultForm }));

  let myOpen = $state(false);

  const { form, errors, enhance, reset } = superForm<
    Infer<typeof schema>,
    { status: number; text: string }
  >(data, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors-and-message",
    validators: arktype(schema, { defaults: defaultForm }),
    onUpdate: async ({ form }) => {
      //TODO: validate form inputs
      const data: Habit = {
        id: 0,
        title: form.data.title,
        description: form.data.description,
        category: form.data.category as HabitCategory,
        completions: 0,
        created: new Date().toISOString(),
        reminder:
          form.data.reminder === "no reminder" ? null : (form.data.reminder as HabitReminder),
        status: "onGoing",
        streak: form.data.streak as HabitStreak,
        theme: form.data.color as HabitTheme,
        updated: null,
      };
      try {
        const response = await invoke("create_habit", { habit: data });
        if (response === "ok") {
          myOpen = false;
          queryClient.invalidateQueries({ queryKey: ["all_habits"] });
          return;
        }
        console.log("not done");
      } catch (e) {
        console.error(e);
      }
    },
  });

  const styles = tv({
    slots: {
      errorMessage: "text-xs text-rose-400 capitalize",
      input:
        "text-muted-foreground placeholder:text-muted-foreground/60 border-transparent p-0 shadow-transparent focus-visible:ring-0",
      selectTrigger:
        "text-muted-foreground flex-center bg-muted-foreground/5 hover:bg-muted-foreground/10 color-fade h-8 w-auto justify-start gap-1.5 p-1.5 text-[11px] capitalize shadow-none focus:ring-0 focus-visible:ring-0",
    },
  });

  const { errorMessage, input, selectTrigger } = styles();

  const streaks = [
    { value: "daily", label: "Daily" },
    { value: "weekly", label: "Weekly" },
    { value: "monthly", label: "Monthly" },
  ];

  const categories = [
    { value: "health", label: "Health" },
    { value: "fitness", label: "Fitness" },
    { value: "nutrition", label: "Nutrition" },
    { value: "leisure", label: "Leisure" },
    { value: "productivity", label: "Productivity" },
    { value: "finace", label: "Finace" },
    { value: "personalGrowth", label: "Personal Growth" },
    { value: "social", label: "Social" },
    { value: "creative", label: "Creative" },
    { value: "study", label: "Study" },
    { value: "home", label: "Home" },
    { value: "work", label: "Work" },
    { value: "school", label: "School" },
    { value: "morning", label: "Morning" },
    { value: "afternoon", label: "Afternoon" },
    { value: "evening", label: "Evening" },
    { value: "night", label: "Night" },
    { value: "other", label: "Other" },
  ];

  const habitColors = [
    { value: "red", label: "Red", class: "bg-red-400" },
    { value: "green", label: "Green", class: "bg-emerald-400" },
    { value: "blue", label: "Blue", class: "bg-blue-400" },
    { value: "rose", label: "Rose", class: "bg-rose-400" },
    { value: "mint", label: "Mint", class: "bg-lime-400" },
    { value: "sky", label: "Sky", class: "bg-sky-400" },
    { value: "amber", label: "Amber", class: "bg-amber-400" },
    { value: "indigo", label: "Indigo", class: "bg-indigo-400" },
    { value: "neutral", label: "Neutral", class: "bg-neutral-400" },
  ];

  const reminderDays = [
    { value: "mon", label: "Mon" },
    { value: "tue", label: "Tue" },
    { value: "wed", label: "Wed" },
    { value: "thu", label: "Thu" },
    { value: "fri", label: "Fri" },
    { value: "sat", label: "Sat" },
    { value: "sun", label: "Sun" },
    { value: "no reminder", label: "No Reminder" },
  ];
</script>

<Dialog.Root bind:open={() => myOpen, (newOpen) => (myOpen = newOpen)}>
  <Dialog.Trigger>
    <Button onclick={() => reset()} class="text-foreground text-xs" variant="default" size="sm"
      >Create Habit</Button
    >
  </Dialog.Trigger>
  <Dialog.Content>
    <form method="POST" use:enhance class="">
      <div>
        <Input
          bind:value={$form.title}
          type="text"
          class={input({
            class: "text-xl placeholder:text-xl md:text-2xl md:placeholder:text-2xl",
          })}
          placeholder="Habit Title"
        />
        {#if $errors.title}
          <span class={errorMessage()}>{$errors.title}</span>
        {/if}
      </div>
      <div>
        <Input
          bind:value={$form.description}
          type="text"
          class={input({
            class: "text-sm placeholder:text-sm",
          })}
          placeholder="Add a short description"
        />
        {#if $errors.description}
          <span class={errorMessage()}>{$errors.description}</span>
        {/if}
      </div>
      <div class="flex-center gap-2">
        <Select.Root type="single" name="streak" bind:value={$form.streak}>
          <Select.Trigger class={selectTrigger()}
            ><CalendarFold size="14" class="text-blue-300" />
            <span>{$form.streak}</span></Select.Trigger
          >
          <Select.Content>
            <Select.Group>
              <Select.GroupHeading class="text-muted-foreground text-xs"
                >Streak Goal</Select.GroupHeading
              >
              {#each streaks as streak}
                <Select.Item
                  class="text-muted-foreground text-xs"
                  value={streak.value}
                  label={streak.label}
                />
              {/each}
            </Select.Group>
          </Select.Content>
        </Select.Root>
        <Select.Root type="single" name="category" bind:value={$form.category}>
          <Select.Trigger class={selectTrigger()}
            ><Box size="14" class="text-orange-500" />
            <span
              >{#if $form.category.length}
                {$form.category}
              {:else}
                Category
              {/if}</span
            ></Select.Trigger
          >
          <Select.Content>
            <Select.Group>
              <Select.GroupHeading class="text-muted-foreground text-xs"
                >Category</Select.GroupHeading
              >
              {#each categories as category}
                <Select.Item
                  class="text-muted-foreground text-xs"
                  value={category.value}
                  label={category.label}
                />
              {/each}
            </Select.Group>
          </Select.Content>
        </Select.Root>
        <Select.Root type="single" name="color" bind:value={$form.color}>
          <Select.Trigger class={selectTrigger()}
            ><PaintRoller size="14" class="text-neutral-700" />
            <span>
              {#if $form.color.length}
                {$form.color}
              {:else}
                Theme
              {/if}
            </span></Select.Trigger
          >
          <Select.Content>
            <Select.Group>
              <Select.GroupHeading class="text-muted-foreground text-xs">Color</Select.GroupHeading>
              {#each habitColors as color}
                <div class="flex-center gap-1">
                  <div class={clsx(`h-2.5 w-2.5 rounded-full ${color.class}`)}></div>
                  <Select.Item
                    class="text-muted-foreground flex-1/2 text-xs"
                    value={color.value}
                    label={color.label}
                  />
                </div>
              {/each}
            </Select.Group>
          </Select.Content>
        </Select.Root>
        <Select.Root type="single" name="color" bind:value={$form.reminder}>
          <Select.Trigger class={selectTrigger()}
            ><Clock size="14" class="text-pink-400" />
            <span>
              {$form.reminder.length ? $form.reminder : "Reminder"}
            </span></Select.Trigger
          >
          <Select.Content>
            <Select.Group>
              <Select.GroupHeading class="text-muted-foreground text-xs">Days</Select.GroupHeading>
              {#each reminderDays as day}
                <Select.Item
                  class="text-muted-foreground flex-1/2 text-xs"
                  value={day.value}
                  label={day.label}
                />
              {/each}
            </Select.Group>
          </Select.Content>
        </Select.Root>
      </div>
      <div class="flex-center mt-8 justify-end gap-2">
        <Dialog.Close>
          <Button size="sm" variant="secondary">Cancel</Button>
        </Dialog.Close>
        <Button type="submit" size="sm">Create</Button>
      </div>
    </form>
  </Dialog.Content>
</Dialog.Root>
