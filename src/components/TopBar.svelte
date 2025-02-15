<script>
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/stores"; // Reactive store for the current route

  let showBackButton = false;

  // Determine if the back button should be shown
  onMount(() => {
    $: {
      const currentPath = $page.url.pathname;
      showBackButton = currentPath.startsWith("/pdf") || currentPath.startsWith("/epub");
    }
  });

  // Navigate back to the bookshelf
  function goBack() {
    goto("/");
  }
</script>

{#if showBackButton}
  <div class="top-bar">
    <button class="back-button" on:click={goBack}>← Back</button>
  </div>
{/if}

<style>
  .top-bar {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    background-color: #f4f4f4;
    padding: 10px;
    box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.1);
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: flex-start;
  }

  .back-button {
    font-size: 16px;
    color: #333;
    background: none;
    border: none;
    cursor: pointer;
    padding: 5px 10px;
    border-radius: 5px;
    transition: background-color 0.2s;
  }

  .back-button:hover {
    background-color: #ddd;
  }
</style>
