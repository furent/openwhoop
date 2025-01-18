<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import Chart from "chart.js/auto";
  import { connectToWhoop, disconnectFromWhoop, sendToggleRealtime, downloadHistory, sendSetClock } from "./whoomp"


  import HeartRateChart from "./components/Charts/HeartRateChart.svelte";
  import TopBar from "./components/TopBar.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import SettingsModal from "./components/SettingsModal.svelte";
  import ProfileManager from "./components/ProfileManager.svelte";
  import DeviceInfoSection from "./components/DeviceInfoSection.svelte";
  import ParsedHistoryData from "./components/ParsedHistoryData.svelte";
  import TerminalLogsSection from "./components/TerminalLogsSection.svelte";
  import DevelopmentInfo from "./components/DevelopmentInfo.svelte";
  

  import type { Profile, ParsedRecord } from "./types"; 



  // ---------------------------
  // Svelte Reactive Variables
  // ---------------------------
  let currentUser: Profile | null = null;
  let chartCanvas: HTMLCanvasElement | null = null;
  let chart: Chart | null = null;
  let isConnected = false;
  let batteryLevel: number | null = null;
  let deviceVersion: { harvard: string; boylston: string } | null = null;
  let isCharging = false;
  let isWorn = false;
  let currentTime: number | null = null;
  let heartRate: number | null = null;
  let isRealtimeActive = false;

  let loading = false;
  let errorMessage: string | null = null;
  let notification: string | null = null;
  let terminalLogs = "";

  // Settings & UI states
  let isSettingsOpen = false;
  let hrvViewMode: "table" | "single" = "table";

  // Data from parsing
  let parsedHistoryData: ParsedRecord[] | null = null;

  // Dev Info overlay
  let showDevInfo = false;

  // Profile overlay
  let isProfileOpen = false;

  // File input
  let fileInput: HTMLInputElement | null = null; 
  let unknownCmdValue = "0xC8";
  let unknownPayloadValue = "01 02";

  // Reconnect attempts for WHOOP
  let reconnectAttempts = 0;

  //Toggle sidebar
  let sidebarOpen = true;

  // ---------------------------
  // SIDEBAR
  // ---------------------------

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }


  // ---------------------------
  // Dev INFO
  // ---------------------------
  const devInfo = {
    bugs: [
      "Clock sync issues with device. Not able to set clock to a chosen date.",
      "Occasional disconnect on data download.",
      "If pairing issues: Unpair from whoop app in device settings",
    ],
    features: [
      "Real-time heart rate monitoring",
      "Battery level tracking",
      "Historical data download",
      "Data parsing and visualization",
      "Device status monitoring",
      "Terminal logging",
      "Visual BLE connection management",
      "Soon RMSSD(HRV)..."
    ],
    todo: [
      "Sleep detection, for most of things like strain, recovery, HRV, etc..., I have been able to reverse engineer calculations, but I need reverse engineer sleep detection and activity detection before they can be automatically calculated",
      "Add data visualization options, doc device workflow, docker support, open-webui/ollama integration...",
      "Sp02 readings",
      "Temperature readings"
    ]
  };

  // ----------------------------------
  // Lifecycle
  // ----------------------------------
  onMount(() => {
    // 1) Balanced braces: we close the function before onDestroy
    if (!chartCanvas) return;
    const ctx = chartCanvas.getContext("2d");
    if (!ctx) {
      console.error("2D context is null, cannot create chart");
      return;
    }
    chart = new Chart(ctx, {
      type: "line",
      data: {
        labels: [],
        datasets: [
          {
            label: "Heart Rate (bpm)",
            data: [],
            borderColor: "#3B82F6",
            borderWidth: 2,
            pointRadius: 0,
            fill: false,
          },
        ],
      },
      options: {},
    });
  });  // <-- close onMount

  onDestroy(() => {
    if (chart) {
      chart.destroy();
      chart = null;
    }
    if (isConnected) {
      disconnectFromWhoop({
        onDisconnect: () => {
          console.log("Disconnected from WHOOP on unmount");
          isConnected = false;
        },
      });
    }
  });



  // ----------------------------------
  // WHOOP Connection / Reconnect
  // ----------------------------------
  async function attemptReconnect(): Promise<boolean> {
    console.log("Attempting reconnect...");
    const success = await connectToWhoop({
      onConnectFailure: (err: Error) => {
        console.error("Reconnect attempt failed:", err);
      },
      onDisconnect: () => {
        console.warn("Disconnected immediately after reconnect attempt!");
      },
    });
    if (success) {
      console.log("Reconnect success!");
      reconnectAttempts = 0;
      return true;
    }
    reconnectAttempts++;
    if (reconnectAttempts < 3) {
      console.log(`Retrying reconnect attempt #${reconnectAttempts + 1}...`);
      await new Promise((r) => setTimeout(r, 3000));
      return attemptReconnect();
    } else {
      console.error("Max reconnect attempts reached. Aborting.");
      return false;
    }
  }

  // ----------------------------------
  // Profile / Login logic (placeholder)
  // ----------------------------------
  function handleLogin(foundProfile: Profile) {
    currentUser = foundProfile;
  }

  // Example fetch + create profile calls
  async function fetchProfile(username: string): Promise<Profile> {
    const res = await fetch(`/api/profile/${username}`, { method: "GET" });
    if (!res.ok) throw new Error("Profile not found");
    return await res.json();
  }
  async function saveProfileToServer(profileData: Profile): Promise<Profile> {
    const res = await fetch("/api/profile", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(profileData),
    });
    if (!res.ok) throw new Error("Failed to save profile");
    return await res.json();
  }

  // ----------------------------------
  // File & History parse logic
  // ----------------------------------
  async function handleParseHistory() {
    console.log("Parse history button clicked!");
    fileInput?.click();
  }

  async function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    try {
      loading = true;
      errorMessage = null;
      if (currentUser) {
        // (1) Upload to user’s profile
        const formData = new FormData();
        formData.append("file", file);
        const uploadResponse = await fetch(`/api/profile/${currentUser.username}/files`, {
          method: "POST",
          body: formData,
        });
        if (!uploadResponse.ok) throw new Error("Failed to upload .bin file");
        const uploadData = await uploadResponse.json();
        alert(`File uploaded: ${uploadData.message}`);

        // (2) Parse on server
        const parseFormData = new FormData();
        parseFormData.append("file", file);
        const parseResponse = await fetch("/api/parse-history", {
          method: "POST",
          body: parseFormData,
        });
        if (!parseResponse.ok) throw new Error("Failed to parse history file");
        parsedHistoryData = await parseResponse.json();
        handleNotification("History file parsed successfully");
      } else {
        // Not logged in => parse anonymously
        console.log("Not logged in => parse anonymously");
        const parseFormData = new FormData();
        parseFormData.append("file", file);
        const parseResponse = await fetch("/api/parse-history", {
          method: "POST",
          body: parseFormData,
        });
        if (!parseResponse.ok) throw new Error("Failed to parse history file (anonymous)");
        parsedHistoryData = await parseResponse.json();
        handleNotification("History file parsed successfully (no login)");
      }
    } catch (err: any) {
      alert(`Upload or parse failed: ${err.message}`);
      errorMessage = err.message;
    } finally {
      loading = false;
    }
  }

  // ----------------------------------
  // Heart Rate Chart logic
  // ----------------------------------

  let heartRateLabels: string[] = [];
  let heartRateValues: number[] = [];

  function updateHeartRateChart(rate: number) {
    const now = new Date();
    const timeLabel = now.toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
    heartRateLabels.push(timeLabel);
    heartRateValues.push(rate);

    if (heartRateLabels.length > 6000) {
      heartRateLabels.shift();
      heartRateValues.shift();
    }
    if (chart) {
    chart.data.labels = heartRateLabels;
    chart.data.datasets[0].data = heartRateValues;
    chart.update(); // re-draws the chart with new data
  }
  }

  // ----------------------------------
  // Misc helpers
  // ----------------------------------
  function handleNotification(message: string) {
    console.log("Notification:", message);
    notification = message;
    setTimeout(() => {
      notification = null;
    }, 2500);
  }

  function handleLog(msg: string) {
    if (msg?.trim()) {
      terminalLogs += msg + "\n";
    } else {
      console.warn("Received empty/invalid log data");
    }
  }

  function computeAverageHrv(data: ParsedRecord[] | null): number | null {
    if (!data) return null;
    const valid = data.filter((r) => r.hrv_rmssd != null).map((r) => r.hrv_rmssd as number);
    if (valid.length === 0) return null;
    return valid.reduce((acc, val) => acc + val, 0) / valid.length;
  }

  function computeAverageHeartRate(data: ParsedRecord[] | null): number | null {
    if (!data) return null;
    const validHr = data.filter((r) => r.heart_rate !== undefined).map((r) => r.heart_rate as number);
    if (validHr.length === 0) return null;
    return validHr.reduce((acc, val) => acc + val, 0) / validHr.length;
  }

  function parseHexPayload(hexStr: string): Uint8Array {
    const cleaned = hexStr.replace(/0x/g, "").replace(/[^0-9A-Fa-f\s]/g, "").trim();
    if (!cleaned) return new Uint8Array([0x00]);
    const bytes = cleaned.split(/\s+/).map((chunk) => parseInt(chunk, 16));
    return new Uint8Array(bytes);
  }

  async function handleSendUnknownCommand() {
    if (!isConnected) {
      alert("Please connect to WHOOP first.");
      return;
    }
    try {
      const cmdByte = parseInt(unknownCmdValue, 16);
      const payload = parseHexPayload(unknownPayloadValue);
      // await sendUnknownCommand(cmdByte, payload);
      handleNotification(`Unknown command 0x${cmdByte.toString(16)} sent successfully!`);
    } catch (err: any) {
      const msg = err?.message || String(err);
      console.error("Failed to send unknown command:", msg);
      handleNotification(`Failed to send unknown command: ${msg}`);
    }
  }

  // ----------------------------------
  // WHOOP connect/disconnect
  // ----------------------------------
  async function handleConnect() {
    console.log("Connect button clicked");
    if (!isConnected) {
      loading = true;
      errorMessage = null;
      console.log("Attempting to connect to WHOOP...");
      const success = await connectToWhoop({
        onConnectSuccess: () => {
          console.log("Connection successful");
          isConnected = true;
          loading = false;
        },
        onConnectFailure: (error: Error) => {
          console.error("Connection failed:", error);
          loading = false;
          errorMessage = `Failed to connect: ${error.message}`;
        },
        onDisconnect: () => {
          console.log("Disconnected from WHOOP");
          isConnected = false;
          loading = false;
        },
        onBatteryUpdate: (level: number) => {
          batteryLevel = level;
        },
        onVersionUpdate: (harvard: string, boylston: string) => {
          deviceVersion = { harvard, boylston };
        },
        onChargingStatusUpdate: (charging: boolean) => {
          isCharging = charging;
        },
        onWristStatusUpdate: (worn: boolean) => {
          isWorn = worn;
        },
        onClockUpdate: (unix: number) => {
          currentTime = unix;
        },
        onHeartRateUpdate: (rate: number) => {
          heartRate = rate; 
          updateHeartRateChart(rate);
        },
        onNotification: handleNotification,
        onLog: handleLog,
      });

      if (success) {
        console.log("connectToWhoop returned true");
      } else {
        console.log("connectToWhoop returned false");
      }
    } else {
      // Disconnect
      console.log("Attempting to disconnect from WHOOP...");
      loading = true;
      const success = await disconnectFromWhoop({
        onDisconnect: () => {
          console.log("Disconnection successful");
          isConnected = false;
          loading = false;
        },
      });
      if (!success) {
        console.error("disconnectFromWhoop returned false");
        errorMessage = "Failed to disconnect properly.";
        loading = false;
      }
    }
  }

  async function handleToggleRealtime() {
    console.log("Toggle real-time heart rate clicked");
    if (isConnected) {
      await sendToggleRealtime();
      isRealtimeActive = !isRealtimeActive;
      console.log(`Real-time ${isRealtimeActive ? "started" : "stopped"}`);
    } else {
      alert("Please connect to WHOOP first.");
    }
  }

  async function handleDownloadHistory() {
    loading = true;
    errorMessage = null;
    try {
      await downloadHistory({
        onNotification: handleNotification,
      });
    } catch (err: any) {
      errorMessage = "History download failed.";
      console.error("Download History Error:", err);
    } finally {
      loading = false;
    }
  }

  async function handleSetClockPlusOneDay() {
    if (!isConnected) {
      alert("Please connect to WHOOP first.");
      return;
    }
    try {
      const now = Math.floor(Date.now() / 1000);
      const futureTime = now + 86400;
      await sendSetClock(futureTime, {
        onConnectFailure: (err: Error) => (errorMessage = err.message),
        onNotification: (msg: string) => handleNotification(msg),
      });
      handleNotification("Clock set to one day in the future!");
    } catch (err: any) {
      console.error("Error setting clock +1 day:", err);
      errorMessage = err.message;
    }
  }

  // ----------------------------------
  // Svelte “computed” style helpers
  // ----------------------------------

  function averageHrv() {
    return hrvViewMode === "single"
      ? computeAverageHrv(parsedHistoryData)
      : undefined;
  }

  function averageHr() {
    return hrvViewMode === "single"
      ? computeAverageHeartRate(parsedHistoryData)
      : undefined;
  }
</script>

<style>
  .hidden {
    display: none;
  }
  .fade-in {
    transition: opacity 0.3s ease;
  }
</style>

<div class="bg-gradient-to-b from-gray-900 to-gray-800 min-h-screen text-white">

  <TopBar
    batteryLevel={batteryLevel}
    isCharging={isCharging}
    isConnected={isConnected}
    onPowerClick={handleConnect} 
    onProfileClick={() => (isProfileOpen = true)}
    onSettingsClick={() => (isSettingsOpen = true)}
    onDownloadClick={handleDownloadHistory}
    onToggleSidebar={toggleSidebar}
  />
  <Sidebar
    isOpen={sidebarOpen}
    navItems={[
      { label: "Dashboard", route: "/dashboard" },
      { label: "Parsed Data", route: "/parsed" },
      { label: "Profile", route: "/profile" },
      { label: "Settings", route: "/settings" },
    ]}
    on:navigate={(e) => {
      console.log("Sidebar navigation item clicked:", e.detail);
    }}
  />

  <div
  class="
    pt-20 px-4 sm:px-8 transition-all duration-300 
    max-w-7xl mx-auto
  "
  style="margin-left: {sidebarOpen ? '16rem' : '0'}"
>
    <button
      class="bg-gray-700 text-white px-4 py-2 rounded-lg hover:bg-gray-600"
      on:click={() => (showDevInfo = !showDevInfo)}
    >
      {showDevInfo ? "Hide Dev Info" : "Show Dev Info"}
    </button>

    <button
      on:click={handleParseHistory}
      class="bg-purple-500 hover:bg-purple-600 text-white font-semibold py-2 px-6 rounded shadow-md
             transition duration-200 ease-in-out disabled:opacity-50"
      disabled={loading}
    >
      Parse History
    </button>

    <input
      type="file"
      bind:this={fileInput}
      style="display: none"
      accept=".bin"
      on:change={handleFileSelect}
    />

    {#if isConnected}
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
      <DeviceInfoSection
        {batteryLevel}
        {deviceVersion}
        {isCharging}
        {isWorn}
        {currentTime}
        {heartRate}
        avgHeartRate={averageHr()}
        avgHrv={averageHrv()}
      />
    </div>
  {/if}

    <button
      on:click={handleToggleRealtime}
      class="{isRealtimeActive ? 'bg-green-500 hover:bg-green-600' : 'bg-blue-500 hover:bg-blue-600'}
             text-white font-semibold py-2 px-6 rounded shadow-md transition duration-200 ease-in-out
             disabled:opacity-50"
      disabled={loading}
    >
      {isRealtimeActive ? "Stop Heart Rate" : "Start Heart Rate"}
    </button>

    {#if isConnected}
      <button
        on:click={handleSetClockPlusOneDay}
        class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-6 rounded shadow-md
               transition duration-200 ease-in-out"
      >
        Set Clock +1 Day
      </button>
    {/if}

    <canvas bind:this={chartCanvas} width="400" height="200"></canvas>


    <div class="mt-4 mb-2">
      <TerminalLogsSection logs={terminalLogs} />
    </div>

    <div class="mt-4 p-4 border border-gray-700 rounded">
      <h3 class="font-bold mb-2">Send Unknown Command</h3>
      <label class="block">
        Cmd (hex):
        <input
          type="text"
          placeholder="0xC8"
          bind:value={unknownCmdValue}
          class="ml-2 text-black"
        />
      </label>
      <label class="block mt-2">
        Payload (hex bytes):
        <input
          type="text"
          placeholder="01 02"
          bind:value={unknownPayloadValue}
          class="ml-2 text-black"
        />
      </label>
    </div>
    <button
      on:click={handleSendUnknownCommand}
      class="bg-red-500 text-white px-4 py-2 mt-4 rounded"
    >
      Send Unknown Command
    </button>

    {#if parsedHistoryData}
      <ParsedHistoryData data={parsedHistoryData} viewMode={hrvViewMode} />
    {/if}

    {#if showDevInfo}
      <DevelopmentInfo
        bugs={devInfo.bugs}
        features={devInfo.features}
        todo={devInfo.todo}
      />
    {/if}

    <SettingsModal
      isOpen={isSettingsOpen}
      currentMode={hrvViewMode}
      on:close={() => (isSettingsOpen = false)}
      on:modeChange={(e) => {
        hrvViewMode = e.detail;
      }}
    />

    {#if isProfileOpen}
      <div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-70 z-[9999]">
        <div class="bg-gray-800 p-6 rounded shadow-lg relative w-80">
          <button
            class="absolute top-2 right-2 text-gray-300 hover:text-white"
            on:click={() => (isProfileOpen = false)}
          >
            &times;
          </button>
          <h2 class="text-xl font-bold mb-4">Profile Info</h2>
          {#if currentUser}
            <p><strong>Username:</strong> {currentUser.username}</p>
            <p><strong>Name:</strong> {currentUser.name}</p>
            <p><strong>Age:</strong> {currentUser.age}</p>
            <p><strong>Password:</strong> {currentUser.password}</p>
          {:else}
            <p>No user logged in.</p>
          {/if}
        </div>
      </div>
    {/if}
  </div>

  {#if notification}
    <div
      class="fixed bottom-4 left-1/2 transform -translate-x-1/2 bg-black bg-opacity-70 text-white 
             px-4 py-2 rounded fade-in"
    >
      {notification}
    </div>
  {/if}
</div>