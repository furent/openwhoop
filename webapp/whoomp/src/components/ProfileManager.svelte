<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { Profile } from "../types";
  
    // Prop: parent passes a function that receives the newly logged-in profile
    export let onLogin: (profile: Profile) => void;
  
    // Local states
    let mode: "login" | "signup" = "login";
  
    let loginUsername = "";
    let loginPassword = "";
  
    let signupUsername = "";
    let signupPassword = "";
    let signupName = "";
    let signupAge: number = 0;
  
    // 1) GET /api/profile/{username}
    async function fetchProfileFromServer(username: string): Promise<Profile> {
      const res = await fetch(`/api/profile/${username}`, {
        method: "GET",
      });
      if (!res.ok) {
        throw new Error("Profile not found or server error");
      }
      return await res.json();
    }
  
    // 2) POST /api/profile
    async function createProfileOnServer(profile: Profile): Promise<Profile> {
      const res = await fetch("/api/profile", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(profile),
      });
      if (!res.ok) {
        throw new Error("Failed to save profile");
      }
      return await res.json();
    }
  
    async function handleLogin() {
      try {
        const foundProfile = await fetchProfileFromServer(loginUsername);
        if (foundProfile.password === loginPassword) {
          onLogin(foundProfile);
        } else {
          alert("Login failed: wrong password");
        }
      } catch (err: any) {
        alert("Login failed: " + err.message);
      }
    }
  
    async function handleSignup() {
      try {
        const newProfile: Profile = {
          username: signupUsername,
          password: signupPassword,
          name: signupName,
          age: signupAge,
        };
        const saved = await createProfileOnServer(newProfile);
        alert("Profile created! You can now login.");
        // Optionally auto-login or just switch to login mode
        mode = "login";
      } catch (err: any) {
        alert("Signup failed: " + err.message);
      }
    }
  </script>
  
  <div class="bg-gray-700 p-4 rounded text-white w-full max-w-md mx-auto">
    <h2 class="text-2xl font-bold mb-4">Profile Manager</h2>
  
    <div class="flex space-x-4 mb-4">
      <button
        class="px-4 py-2 rounded {mode === 'login' ? 'bg-blue-600' : 'bg-gray-600'}"
        on:click={() => (mode = 'login')}
      >
        Login
      </button>
      <button
        class="px-4 py-2 rounded {mode === 'signup' ? 'bg-blue-600' : 'bg-gray-600'}"
        on:click={() => (mode = 'signup')}
      >
        Sign Up
      </button>
    </div>
  
    {#if mode === "login"}
      <div>
        <div class="mb-2">
          <label>Username:</label>
          <input
            class="block w-full bg-gray-800 border border-gray-600 rounded p-2 mt-1"
            bind:value={loginUsername}
          />
        </div>
        <div class="mb-2">
          <label>Password:</label>
          <input
            class="block w-full bg-gray-800 border border-gray-600 rounded p-2 mt-1"
            type="password"
            bind:value={loginPassword}
          />
        </div>
        <button
          class="bg-green-600 px-4 py-2 rounded mt-2"
          on:click={handleLogin}
        >
          Login
        </button>
      </div>
    {:else}
      <div>
        <div class="mb-2">
          <label>Username:</label>
          <input
            class="block w-full bg-gray-800 border border-gray-600 rounded p-2 mt-1"
            bind:value={signupUsername}
          />
        </div>
        <div class="mb-2">
          <label>Password:</label>
          <input
            class="block w-full bg-gray-800 border border-gray-600 rounded p-2 mt-1"
            type="password"
            bind:value={signupPassword}
          />
        </div>
        <div class="mb-2">
          <label>Name:</label>
          <input
            class="block w-full bg-gray-800 border border-gray-600 rounded p-2 mt-1"
            bind:value={signupName}
          />
        </div>
        <div class="mb-2">
          <label>Age:</label>
          <input
            class="block w-full bg-gray-800 border border-gray-600 rounded p-2 mt-1"
            type="number"
            bind:value={signupAge}
          />
        </div>
        <button
          class="bg-green-600 px-4 py-2 rounded mt-2"
          on:click={handleSignup}
        >
          Create Profile
        </button>
      </div>
    {/if}
  </div>
  