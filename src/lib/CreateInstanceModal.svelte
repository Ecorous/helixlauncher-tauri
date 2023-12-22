<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Plus } from "lucide-svelte";
    import { onMount } from "svelte";

    let nameInput: string;
    let mcVersionInput: string;
    let modloaderSelect: string;
    let modloaderVersionInput: string;
    let mVBox: HTMLInputElement;

    onMount(() => {});

    async function createInstance() {
        await invoke("create_instance", {
            name: nameInput,
            mcVersion: mcVersionInput,
            modloader: modloaderSelect,
            modloaderVersion: modloaderVersionInput,
        });
    }
</script>

<input bind:value={nameInput} placeholder="Instance Name" />
<br />
<br />
<input bind:value={mcVersionInput} placeholder="Minecraft Version" />
<br />
<br />
<select
    on:change={() => {
        setTimeout(() => {
            console.log("Current value:", modloaderSelect);
            if (modloaderSelect != "Vanilla") {
                console.log("im here!");
                mVBox.style.display = "inherit"; // shows the modloader version input
            } else {
                mVBox.style.display = "none"; // hides the modloader version input
            }
        }, 0);
    }}
    bind:value={modloaderSelect}
>
    <option>Vanilla</option>
    <option>Quilt</option>
    <option>NeoForge</option>
    <option>Fabric</option>
    <option>Forge</option>
</select>
<br />
<br />
<input
    style="display: none;"
    bind:value={modloaderVersionInput}
    bind:this={mVBox}
    placeholder="Modloader Version"
/>

<button class="launchRow" on:click={createInstance}>
    <Plus />Create Instance
</button>
