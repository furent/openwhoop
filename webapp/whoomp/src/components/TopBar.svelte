<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import Icon from '@iconify/svelte';

  // ---------------------------
  // PROPS
  // ---------------------------
  export let onPowerClick: () => void;
  export let onProfileClick: () => void;
  export let onSettingsClick: () => void;
  export let onDownloadClick: () => void;
  export let batteryLevel: number | null = null;
  export let isCharging = false;
  export let isConnected = false;
  export let onToggleSidebar: () => void;

  // ---------------------------
  // LOCAL STATE COMPONENTS
  // ---------------------------
  let animationPhase: "idle" | "slot" | "fade" | "done" = "idle";
  let showStatusPill = !isConnected;
  let showBattery = isConnected;

  // ---------------------------
  // FUNCTIONS
  // ---------------------------

  function handleStatusClick() {
    // If not connected & we are idle => do the fancy pill animation
    if (!isConnected && animationPhase === "idle") {
      animationPhase = "slot";
      // After 0.75s -> fade
      setTimeout(() => {
        animationPhase = "fade";
        setTimeout(() => {
          // fade complete
          animationPhase = "done";
          showStatusPill = false;
          showBattery = true;
          // Now call the real connect function
          onPowerClick();
        }, 500);
      }, 750);
    } else {
      // Else just do normal connect/disconnect
      onPowerClick();
    }
  }

  const statusText = isConnected ? "Connected" : "Disconnected";

  // Icon color logic
  function powerIconClass() {
    if (animationPhase === "fade" || animationPhase === "slot") {
      return "transition-colors duration-500 text-green-500";
    }
    return isConnected ? "text-green-500" : "text-red-500";
  }

  // ---------------------------
  // STYLING
  // ---------------------------
</script>

<nav
  class="fixed top-0 left-0 w-full h-16 z-50 bg-gray-900 text-white px-4 py-2 flex items-center justify-between"
>
  <div class="flex items-center space-x-4">
    <div class="flex items-center space-x-4">
      <button
        class="focus:outline-none hover:opacity-80"
        on:click={onToggleSidebar}
        title="Open/Close Sidebar"
      >
      <Icon icon="solar:hamburger-menu-linear" class="w-6 h-6 text-white" />

      </button>
      <span class="font-bold text-xl tracking-wide">WHOOMP</span>

      {#if showStatusPill}
        <!-- Fancy pill -->
        <div
          class="flex items-center space-x-2 px-3 py-1 rounded-full bg-black"
          style="animation: {animationPhase === 'fade'
            ? 'pillFadeLeft 0.5s forwards'
            : 'none'}"
        >
          <!-- Power icon button -->
          <button
            on:click={handleStatusClick}
            title={isConnected ? "Disconnect" : "Connect"}
            class="hover:opacity-80 transition flex items-center justify-center"
          >
            <Icon icon="solar:power-bold" class="w-6 h-6 text-white" />

          </button>

          <!-- Slot machine text -->
          <div
            class="relative overflow-hidden h-5 w-24 flex items-center justify-center"
            style="animation: {animationPhase === 'slot'
              ? 'slotMachineDown 0.75s ease-in-out forwards'
              : 'none'}"
          >
            {statusText}
          </div>
        </div>
      {/if}

      {#if showBattery && batteryLevel !== null}
        <!-- Battery bar -->
        <div
          class="flex items-center space-x-2"
          style="animation: batterySlideIn 0.5s ease-out forwards"
        >
          <span>Battery:</span>
          <div
            class="w-24 bg-gray-600 rounded-full h-4 relative overflow-hidden"
          >
            {#if isCharging}
              <div
                class="h-4 rounded-full relative animate-battery-charging"
                style="
                  width: {batteryLevel}%;
                  background: linear-gradient(to right, #22c55e 25%, #16a34a 50%, #22c55e 75%);
                  background-size: 200% auto;
                "
              >
                <span
                  class="absolute inset-0 flex items-center justify-center text-white text-xs"
                >
                  ⚡
                </span>
              </div>
            {:else}
              <!-- svelte-ignore element_invalid_self_closing_tag -->
              <div
                class="h-4 rounded-full"
                style="
                  width: {batteryLevel}%;
                  background-color: {batteryLevel < 20
                  ? '#ef4444' /* red-500 */
                  : batteryLevel < 50
                    ? '#f59e0b' /* yellow-500 */
                    : '#22c55e'};       /* green-500 */
                "
              />
            {/if}
          </div>
        </div>
      {/if}
    </div>

    <div class="flex items-center space-x-4">
      <!-- Download button -->
      <button
        on:click={onDownloadClick}
        title="Download History"
        class="hover:opacity-80 transition flex items-center justify-center"
      >
        <!-- The prefix "i-solar-" still applies -->
        <Icon icon="solar:download-square-bold" class="w-6 h-6 text-white" />


      </button>

      <button
        on:click={onProfileClick}
        class="bg-gray-700 px-3 py-1 rounded hover:bg-gray-600 transition"
      >
        Profile
      </button>
      <button
        on:click={onSettingsClick}
        class="bg-gray-700 px-3 py-1 rounded hover:bg-gray-600 transition"
      >
        Settings
      </button>
    </div>
  </div>
</nav>

<style>
  @keyframes slotMachineDown {
    0% {
      transform: translateY(0%);
      opacity: 1;
    }
    50% {
      transform: translateY(100%);
      opacity: 0;
    }
    51% {
      transform: translateY(-100%);
    }
    100% {
      transform: translateY(0%);
      opacity: 1;
    }
  }

  @keyframes pillFadeLeft {
    0% {
      opacity: 1;
      transform: translateX(0);
    }
    100% {
      opacity: 0;
      transform: translateX(-50px);
    }
  }

  @keyframes batterySlideIn {
    0% {
      opacity: 0;
      transform: translateX(50px);
    }
    100% {
      opacity: 1;
      transform: translateX(0);
    }
  }

</style>
