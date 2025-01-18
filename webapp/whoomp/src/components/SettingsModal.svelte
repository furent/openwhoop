<script lang="ts">
    import { createEventDispatcher } from "svelte";
  
    export let isOpen: boolean = false;
    export let currentMode: "table" | "single" = "table";
  
    // We'll dispatch events to signal closing or mode changes
    const dispatch = createEventDispatcher();
  
    function handleClose() {
      dispatch("close");
    }
  
    function setMode(newMode: "table" | "single") {
      dispatch("modeChange", newMode);
    }
  
    function handleModalClick(event: MouseEvent) {
      // Prevent clicks inside the modal from closing it
      event.stopPropagation();
    }
  </script>
  
  {#if isOpen}
    <div
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center"
      on:click={handleClose}
    >
      <div
        class="relative bg-gray-800 rounded-lg w-11/12 sm:w-3/4 lg:w-1/2 xl:w-2/5 p-6 shadow-lg"
        on:click={handleModalClick}
      >
        <!-- Close button -->
        <button
          class="absolute top-2 right-2 text-gray-300 hover:text-white text-xl"
          on:click={handleClose}
        >
          &times;
        </button>
  
        <h2 class="text-2xl text-white font-bold mb-4">Settings</h2>
  
        <div class="mb-6">
          <label class="block text-gray-300 font-semibold mb-2">
            HRV Display Mode:
          </label>
          <div class="space-y-2">
            <label class="inline-flex items-center space-x-2">
              <input
                type="radio"
                name="hrvMode"
                value="table"
                checked={currentMode === "table"}
                on:change={() => setMode("table")}
                class="form-radio text-blue-500"
              />
              <span class="text-gray-200">Table</span>
            </label>
  
            <label class="inline-flex items-center space-x-2">
              <input
                type="radio"
                name="hrvMode"
                value="single"
                checked={currentMode === "single"}
                on:change={() => setMode("single")}
                class="form-radio text-blue-500"
              />
              <span class="text-gray-200">Single</span>
            </label>
          </div>
        </div>
      </div>
    </div>
  {/if}
  