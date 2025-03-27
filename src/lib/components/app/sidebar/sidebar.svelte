<script>
  import { tv } from "tailwind-variants";

  const styles = tv({
    slots: {
      container: "h-screen w-16 border-r-[1px] border-neutral-800 bg-neutral-900/80",
    },
  });
  const { container } = styles();
</script>

<div class={container()}></div>
