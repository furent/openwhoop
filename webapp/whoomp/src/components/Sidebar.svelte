<script lang="ts">
    import { createEventDispatcher } from "svelte";
  
    /**
     * Whether the sidebar should be open or closed.
     */
    export let isOpen: boolean = false;
  
    /**
     * An array of navigation items to display in the sidebar.
     */
    export let navItems: { label: string; route?: string }[] = [
      { label: "Dashboard", route: "/dashboard" },
      { label: "Parsed Data", route: "/parsed" },
      { label: "Profile", route: "/profile" },
      { label: "Settings", route: "/settings" },
    ];
  
    // Dispatch event so parent can handle route changes or toggles
    const dispatch = createEventDispatcher();
  
    function handleNavClick(item: any) {
      dispatch("navigate", item);
    }
  </script>
  
  <style>
    /* Slide-in from left (optional). 
       Adjust transitions if you want a pinned layout or a different style. */
    .sidebar-enter {
      transform: translateX(-100%);
      transition: transform 0.3s ease;
    }
    .sidebar-enter.sidebar-enter-active {
      transform: translateX(0);
    }
  
    .sidebar-leave {
      transform: translateX(0);
      transition: transform 0.3s ease;
    }
    .sidebar-leave.sidebar-leave-active {
      transform: translateX(-100%);
    }
  </style>
  
  <div
  class="fixed top-16 left-0 h-[calc(100vh-4rem)] w-64 bg-gray-900 text-white border-r border-gray-700 
         z-40 shadow-md transition-transform duration-300"
  style="
    transform: translateX({isOpen ? '0' : '-100%'});
  "
>
  <div class="px-4 py-2 border-b border-gray-700 font-bold text-lg">Menu</div>

  <nav class="flex-1 overflow-auto mt-2">
    {#each navItems as item (item.label)}
      <button
        on:click={() => handleNavClick(item)}
        class="w-full text-left px-4 py-2 hover:bg-gray-800 transition flex items-center"
      >
        <span class="ml-1">{item.label}</span>
      </button>
    {/each}
  </nav>
</div>
  