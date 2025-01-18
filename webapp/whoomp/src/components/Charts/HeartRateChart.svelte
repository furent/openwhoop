<script lang="ts">
  </script>
  
  <div class="mt-4">
    <p class="text-lg">
      Heart Rate: <span id="heartRate">--</span> bpm
    </p>
  
    <div class="flex items-center gap-1 mb-4">
      <label for="timeLimit" class="text-sm font-semibold">
        Time Limit (minutes)
      </label>
      <input
        type="number"
        id="timeLimit"
        value="10"
        min="1"
        max="60"
        class="border border-gray-300 rounded-md p-1 w-12 text-sm focus:ring-2"
      />
    </div>
  
    <canvas id="heartRateChart" width="400" height="200"></canvas>
  </div>
  