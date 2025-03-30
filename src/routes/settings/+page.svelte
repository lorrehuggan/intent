<script>
  import Switch from "@/components/ui/switch/switch.svelte";

  const html = document.querySelector("html");
  let switchToggle = $state(false);

  $effect(() => {
    if (switchToggle) {
      html?.classList.add("dark");
    } else {
      html?.classList.remove("dark");
    }
  });

  // function toggleTheme() {
  //   if (!html) return;
  //   if (html.classList.contains("dark")) {
  //     html.classList.remove("dark");
  //     switchToggle = false;
  //   } else {
  //     html.classList.add("dark");
  //     switchToggle = true;
  //   }
  // }
</script>

<p>Settings</p>
<Switch bind:checked={switchToggle} />
