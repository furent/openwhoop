<script lang="ts">
    export let logs: string = "";
  </script>
  
  <!-- We simply show the logs if non-empty -->
  {#if logs}
    <div class="bg-gray-700 rounded-lg shadow p-6 mt-6">
      <h3 class="text-lg font-semibold text-white mb-4">Terminal Logs</h3>
      <div class="max-h-60 overflow-auto bg-gray-900 text-green-400 p-4 rounded">
        <pre>{logs}</pre>
      </div>
    </div>
  {/if}
  