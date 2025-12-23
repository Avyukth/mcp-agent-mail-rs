<!--
  @component
  StatusIndicator - Animated status dot with pulse effect.

  @example
  ```svelte
  <StatusIndicator status="online" />
  <StatusIndicator status="busy" label="In a meeting" />
  <StatusIndicator status="offline" size="lg" />
  ```
-->
<script lang="ts">
    import { browser } from "$app/environment";

    /** Status type determining color */
    export let status: "online" | "offline" | "busy" | "away" | "error" | "warning" | "success" = "online";
    /** Size of the indicator */
    export let size: "sm" | "md" | "lg" = "md";
    /** Optional label text */
    export let label: string = "";
    /** Show pulse animation */
    export let pulse: boolean = true;
    /** Additional CSS classes */
    let className: string = "";
    export { className as class };

    // Check for reduced motion preference
    const prefersReducedMotion = browser
        ? window.matchMedia("(prefers-reduced-motion: reduce)").matches
        : false;

    const sizeClasses = {
        sm: "h-2 w-2",
        md: "h-3 w-3",
        lg: "h-4 w-4",
    };

    const statusColors = {
        online: "bg-green-500",
        success: "bg-green-500",
        offline: "bg-gray-400",
        busy: "bg-red-500",
        error: "bg-red-500",
        away: "bg-yellow-500",
        warning: "bg-yellow-500",
    };

    const pulseColors = {
        online: "bg-green-400",
        success: "bg-green-400",
        offline: "bg-gray-300",
        busy: "bg-red-400",
        error: "bg-red-400",
        away: "bg-yellow-400",
        warning: "bg-yellow-400",
    };

    // Screen reader text
    const srText = {
        online: "Online",
        success: "Success",
        offline: "Offline",
        busy: "Busy",
        error: "Error",
        away: "Away",
        warning: "Warning",
    };

    $: showPulse = pulse && !prefersReducedMotion && status !== "offline";
</script>

<span
    class="inline-flex items-center gap-2 {className}"
    data-testid="status-indicator"
    role="status"
>
    <span class="relative inline-flex">
        <!-- Pulse ring -->
        {#if showPulse}
            <span
                class="absolute inline-flex h-full w-full rounded-full opacity-75 {pulseColors[status]} animate-ping"
                aria-hidden="true"
            ></span>
        {/if}
        <!-- Status dot -->
        <span
            class="relative inline-flex rounded-full {sizeClasses[size]} {statusColors[status]}"
            aria-hidden="true"
        ></span>
    </span>

    {#if label}
        <span class="text-sm text-muted-foreground">{label}</span>
    {/if}

    <!-- Screen reader text -->
    <span class="sr-only">{label || srText[status]}</span>
</span>

<style>
    /* Respect reduced motion preference */
    @media (prefers-reduced-motion: reduce) {
        .animate-ping {
            animation: none;
            opacity: 0;
        }
    }
</style>
