<script lang="ts">
    export let batteryLevel: number | null = null;
    export let deviceVersion: { harvard: string; boylston: string } | null = null;
    export let isCharging: boolean = false;
    export let isWorn: boolean = false;
    export let currentTime: number | null = null;
    export let heartRate: number | null = null;
  
    // optional props
    export let avgHeartRate: number | null = null;
    export let avgHrv: number | null = null;

  
    function formatUnixTime(unix: number): string {
      if (!unix) return "Loading...";
      return new Date(unix * 1000).toLocaleString();
    }
  </script>
  
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
    <!-- Current Time -->
    <div class="bg-gray-700 rounded-lg shadow p-6">
      <h3 class="text-lg font-semibold text-white">Current Time</h3>
      <p class="mt-4 text-gray-300">
        {#if currentTime}
          {formatUnixTime(currentTime)}
        {:else}
          Loading...
        {/if}
      </p>
    </div>
  
    <!-- Heart Rate -->
    <div class="bg-gray-700 rounded-lg shadow p-6">
      <h3 class="text-lg font-semibold text-white">Heart Rate</h3>
      <p class="mt-4 text-gray-300">
        {heartRate !== null ? `${heartRate} BPM` : "Loading..."}
      </p>
    </div>
  
    <!-- Show avg heart rate if defined (i.e. "single" mode) -->
    {#if avgHeartRate !== undefined}
      <div class="bg-gray-700 rounded-lg shadow p-6">
        <h3 class="text-lg font-semibold text-white">Avg Heart Rate</h3>
        <p class="mt-4 text-gray-300">
          {avgHeartRate == null ? "N/A" : `${avgHeartRate.toFixed(2)} BPM`}
        </p>
      </div>
    {/if}
  
    {#if avgHrv !== undefined}
      <div class="bg-gray-700 rounded-lg shadow p-6">
        <h3 class="text-lg font-semibold text-white">Avg HRV (RMSSD)</h3>
        <p class="mt-4 text-gray-300">
          {avgHrv == null ? "N/A" : `${avgHrv.toFixed(2)} ms`}
        </p>
      </div>
    {/if}
  </div>
  