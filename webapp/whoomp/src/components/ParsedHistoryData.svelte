<script lang="ts">
    import type { ParsedRecord } from "../types";
  
    export let data: ParsedRecord[] = [];
    export let viewMode: "table" | "single" = "table";
  </script>
  
  {#if !data || data.length === 0}
    <p class="text-gray-300">No history data to display.</p>
  {:else}
    <div class="bg-gray-700 rounded-lg shadow p-6 mt-6">
      <h3 class="text-lg font-semibold text-white mb-4">
        Parsed History Data
      </h3>
  
      {#if viewMode === "table"}
        <div class="overflow-x-auto">
          <table class="table-auto w-full text-left text-gray-300">
            <thead class="bg-gray-800">
              <tr>
                <th class="px-4 py-2">Timestamp</th>
                <th class="px-4 py-2">Heart Rate</th>
                <th class="px-4 py-2">RR Intervals</th>
                <th class="px-4 py-2">HRV (RMSSD)</th>
              </tr>
            </thead>
            <tbody>
              {#each data as record, index}
                <tr class="border-t border-gray-600">
                  <td class="px-4 py-2">
                    {record.timestamp ?? "N/A"}
                  </td>
                  <td class="px-4 py-2">
                    {record.heart_rate != null ? record.heart_rate : "N/A"}
                  </td>
                  <td class="px-4 py-2">
                    {#if record.rr_intervals && record.rr_intervals.length}
                      {record.rr_intervals.join(", ")}
                    {:else}
                      N/A
                    {/if}
                  </td>
                  <td class="px-4 py-2">
                    {#if record.hrv_rmssd != null}
                      {record.hrv_rmssd.toFixed(2)}
                    {:else}
                      N/A
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {:else}
        <div class="text-white text-sm italic">
          <p>HRV/Heart Rate displayed in single-mode or summarized elsewhere.</p>
        </div>
      {/if}
    </div>
  {/if}
  