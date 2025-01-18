<script lang="ts">
  export let bugs: string[] = [];
  export let features: string[] = [];
  export let todo: string[] = [];
</script>

<div class="fixed right-0 top-20 w-80 h-screen bg-gray-900 p-6 overflow-y-auto z-40">
  <h2 class="text-2xl font-bold text-white mb-6">Development Info</h2>

  <!-- Known Bugs -->
  <div class="mb-6">
    <h3 class="text-lg font-semibold text-red-400 mb-2">Known Bugs</h3>
    <ul class="list-disc list-inside text-red-300">
      {#each bugs as bug}
        <li class="mb-2">{bug}</li>
      {/each}
    </ul>
  </div>

  <!-- Current Features -->
  <div class="mb-6">
    <h3 class="text-lg font-semibold text-green-400 mb-2">Current Features</h3>
    <ul class="list-disc list-inside text-green-300">
      {#each features as feat}
        <li class="mb-2">{feat}</li>
      {/each}
    </ul>
  </div>

  <!-- Todo List -->
  <div class="mb-6">
    <h3 class="text-lg font-semibold text-yellow-400 mb-2">Todo</h3>
    <ul class="list-disc list-inside text-yellow-300">
      {#each todo as item}
        <li class="mb-2">{item}</li>
      {/each}
    </ul>
  </div>
</div>
