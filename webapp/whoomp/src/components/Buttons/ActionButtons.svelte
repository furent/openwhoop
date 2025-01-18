<script lang="ts">
    export let isConnected: boolean;
    export let isRealtimeActive: boolean;
    export let loading: boolean;
    export let handleConnect: () => void;
    export let handleToggleRealtime: () => void;
  
    // If you also want a "Download" button here, add a prop for it, etc.
  </script>
  
  <div class="flex flex-col sm:flex-row items-center justify-center space-y-4 sm:space-y-0 sm:space-x-4 mb-6">
    <button
      on:click={handleConnect}
      disabled={loading}
      class="
        {isConnected 
          ? 'bg-red-500 hover:bg-red-600' 
          : 'bg-blue-500 hover:bg-blue-600'}
        text-white font-semibold py-2 px-6 rounded shadow-md 
        transition duration-200 ease-in-out 
        disabled:opacity-50
      "
    >
      {#if loading}
        Processing...
      {:else}
        {isConnected ? "Disconnect" : "Connect WHOOP"}
      {/if}
    </button>
  
    <button
      on:click={handleToggleRealtime}
      disabled={!isConnected}
      class="
        {isRealtimeActive 
          ? 'bg-green-500 hover:bg-green-600' 
          : 'bg-blue-500 hover:bg-blue-600'}
        text-white font-semibold py-2 px-6 rounded shadow-md 
        transition duration-200 ease-in-out 
        disabled:opacity-50
      "
    >
      {isRealtimeActive ? "Stop Heart Rate" : "Start Heart Rate"}
    </button>
  
    <button
      disabled={!isConnected}
      class="
        bg-blue-500 hover:bg-blue-600 text-white font-semibold 
        py-2 px-6 rounded shadow-md 
        transition duration-200 ease-in-out 
        disabled:opacity-50
      "
    >
      Download History
    </button>
  </div>
  