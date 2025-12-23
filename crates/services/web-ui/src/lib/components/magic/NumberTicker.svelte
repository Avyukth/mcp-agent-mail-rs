<!--
  @component
  NumberTicker - Animate numbers counting up or down to a target value.
  Enhanced version with direction support and more formatting options.

  @example
  ```svelte
  <NumberTicker value={1234} />
  <NumberTicker value={99.9} decimals={1} suffix="%" />
  <NumberTicker value={50000} prefix="$" direction="up" />
  ```
-->
<script lang="ts">
    import { tweened } from "svelte/motion";
    import { cubicOut } from "svelte/easing";
    import { browser } from "$app/environment";
    import { onMount } from "svelte";

    /** Target value to animate to */
    export let value: number;
    /** Animation duration in milliseconds */
    export let duration: number = 1000;
    /** Animation delay in milliseconds */
    export let delay: number = 0;
    /** Number of decimal places */
    export let decimals: number = 0;
    /** Prefix string (e.g., "$") */
    export let prefix: string = "";
    /** Suffix string (e.g., "%", "+") */
    export let suffix: string = "";
    /** Count direction - up from 0 or down from value*2 */
    export let direction: "up" | "down" = "up";
    /** Use locale formatting (thousands separators) */
    export let useLocale: boolean = true;
    /** Additional CSS classes */
    let className: string = "";
    export { className as class };

    // Check for reduced motion preference
    const prefersReducedMotion = browser
        ? window.matchMedia("(prefers-reduced-motion: reduce)").matches
        : false;

    // Start value based on direction
    const startValue = direction === "up" ? 0 : value * 2;

    const displayed = tweened(prefersReducedMotion ? value : startValue, {
        duration: prefersReducedMotion ? 0 : duration,
        easing: cubicOut,
    });

    let hasStarted = false;

    onMount(() => {
        if (prefersReducedMotion) {
            displayed.set(value);
            return;
        }

        const timeoutId = setTimeout(() => {
            hasStarted = true;
            displayed.set(value);
        }, delay);

        return () => clearTimeout(timeoutId);
    });

    // React to value changes after initial mount
    $: if (hasStarted) {
        displayed.set(value);
    }

    // Format number
    function formatNumber(n: number): string {
        if (useLocale) {
            return n.toLocaleString(undefined, {
                minimumFractionDigits: decimals,
                maximumFractionDigits: decimals,
            });
        }
        return decimals > 0 ? n.toFixed(decimals) : Math.round(n).toString();
    }
</script>

<span
    class="tabular-nums tracking-tight {className}"
    data-testid="number-ticker"
    aria-live="polite"
>
    {prefix}{formatNumber($displayed)}{suffix}
</span>
