<!--
  @component
  AnimatedList - Staggered entrance animation for list items.
  Each item fades in with a slight delay after the previous one.

  @example
  ```svelte
  <AnimatedList items={messages} let:item let:index>
    <MessageCard {item} />
  </AnimatedList>
  ```
-->
<script lang="ts" generics="T">
    import { browser } from "$app/environment";
    import { onMount } from "svelte";
    import { fly, fade } from "svelte/transition";

    /** Items to render in the list */
    export let items: T[];
    /** Delay between each item animation in ms */
    export let stagger: number = 50;
    /** Initial animation delay in ms */
    export let delay: number = 0;
    /** Animation duration per item in ms */
    export let duration: number = 300;
    /** Whether to animate on initial render */
    export let animateOnMount: boolean = true;
    /** Direction items come from */
    export let direction: "up" | "down" | "left" | "right" = "up";
    /** Additional CSS classes for the container */
    let className: string = "";
    export { className as class };

    // Check for reduced motion preference
    const prefersReducedMotion = browser
        ? window.matchMedia("(prefers-reduced-motion: reduce)").matches
        : false;

    let mounted = false;

    onMount(() => {
        if (animateOnMount) {
            mounted = true;
        }
    });

    // Compute fly parameters based on direction
    const flyParams = {
        up: { y: 20, x: 0 },
        down: { y: -20, x: 0 },
        left: { x: 20, y: 0 },
        right: { x: -20, y: 0 },
    };

    function getTransition(index: number) {
        if (prefersReducedMotion) {
            return { duration: 0 };
        }
        return {
            delay: delay + index * stagger,
            duration,
            ...flyParams[direction],
        };
    }
</script>

<ul class="animated-list {className}" data-testid="animated-list">
    {#each items as item, index (item)}
        {#if mounted || !animateOnMount}
            <li
                in:fly={getTransition(index)}
                class="animated-list-item"
            >
                <slot {item} {index} />
            </li>
        {/if}
    {/each}
</ul>

<style>
    .animated-list {
        list-style: none;
        padding: 0;
        margin: 0;
    }

    .animated-list-item {
        /* Ensure items don't overflow during animation */
        overflow: visible;
    }
</style>
