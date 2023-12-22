<script lang="ts">
  import { Home, Library, Users, RotateCw, Plus } from "lucide-svelte";
  import HomePage from "./lib/HomePage.svelte";
  import Accounts from "./lib/Accounts.svelte";
  import Instances from "./lib/Instances.svelte";
  import CreateInstanceModal from "./lib/CreateInstanceModal.svelte";
  import LoginModal from "./lib/LoginModal.svelte";
  import { onMount } from "svelte";
  // @ts-ignore
  import Modal from 'svelte-simple-modal';
  import { writable } from "svelte/store";

  let isProduction = import.meta.env.MODE === "production";

  let pageDiv: HTMLDivElement;
  let debugButton: HTMLButtonElement;

  let currentPage = "home";

  function switchPage() {
    for (let element of pageDiv.children) {
      pageDiv.removeChild(element);
    }
    if (currentPage == "home") {
      new HomePage({ target: pageDiv });
    } else if (currentPage == "instances") {
      new Instances({ target: pageDiv });
    } else if (currentPage == "accounts") {
      new Accounts({ target: pageDiv });
    }
  }

  function switchToHome() {
    // if (currentPage != "home") {
    //   switchPage();
    // }
    currentPage = "home";
    switchPage();
  }

  function switchToInstances() {
    // if (currentPage != "instances") {
    //   switchPage();
    // }
    currentPage = "instances";
    switchPage();
  }

  function switchToAccounts() {
    // if (currentPage != "accounts") {
    //   switchPage();
    // }
    currentPage = "accounts";
    switchPage();
  }

  function cleanupButton() {
    window.location.href = "/";
  }

  function showAddInstance() {
    console.log("w");
    showAddInstanceModal();
  }

  function activeLogin() {
    showLoginModal()
  }

  onMount(async () => {
    switchPage();
    if (!isProduction) {
      debugButton.style.display = "flex";
    }
  });

  const loginModal = writable(null as unknown);
  const showLoginModal = () => loginModal.set(LoginModal)
  const addInstanceModal = writable(null as unknown);
  const showAddInstanceModal = () => addInstanceModal.set(CreateInstanceModal);

</script>

<main class="container">
  <Modal show={$addInstanceModal}></Modal>
  <Modal show={$loginModal}></Modal>
  <div class="sidebar">
    <img
      alt="Helix Launcher Banner"
      src="/banner-launcher_96h.png"
      width="241"
      height="64"
    />
    <div class="sidebar-links">
      <button type="button" on:click={switchToHome} class="row">
        <Home class="navbar-icon" /> Home
      </button>
      <br />
      <div style="display: flex;">
        <button type="button" on:click={switchToInstances} class="override row">
          <Library class="navbar-icon" /> Instances
        </button>
        <button on:click={showAddInstance} class="addInstanceButton"><Plus class="addInstanceIcon"/></button>
      </div>
      <br />
      <div style="display: flex;">
        <button type="button" on:click={switchToAccounts} class="override-a row">
          <Users class="navbar-icon" /> Accounts
        </button>
        <button on:click={activeLogin} class="addInstanceButton"><Plus class="addInstanceIcon"/></button>
      </div>
      <br />
      <br />
      <br />
      <button
        bind:this={debugButton}
        style="display: none;"
        type="button"
        on:click={cleanupButton}
        class="row red"
      >
        <RotateCw class="navbar-icon" /> Cleanup (DEV)
      </button>
    </div>
  </div>

  <div class="mainpage">
    <div bind:this={pageDiv}></div>
  </div>
</main>
