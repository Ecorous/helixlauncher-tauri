<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";
    import { Cross, Pencil, Play } from "lucide-svelte";
    import { onMount } from "svelte";

    let instances: [{ name: string }] = [{ name: "Test" }];
    let mapKillButtons = new Map<string, HTMLButtonElement>();
    let map = new Map<string, number>();

    onMount(async () => {
        instances = await invoke("get_instances");
        // @ts-ignore
        appWindow.listen("started", (e) => {
            console.log(e.payload);
            // @ts-ignore
            map.set(
                // @ts-ignore
                e.payload.name,
                // @ts-ignore
                e.payload.id,
            )
            // @ts-ignore
            map.get(
                // @ts-ignore
                mapKillButtons.get(e.payload.name),
            ).style.display = "inherit";
        });
    });

    async function killInstance(name: string) {
        appWindow.emit("kill-game", map.get(name) as number);
    }

    async function launchInstance(name: string) {
        await invoke("launch", { name });
    }
</script>

<div>
    <p>instances!</p>

    {#each instances as i}
        <div class="instance">
            <!---->
            <div class="launchNameRow">
                <img
                    alt="Instance Icon"
                    src="https://cdn.ecorous.org/blackhole.png"
                />

                <h3>{i.name}</h3>
                <br />
            </div>
            <br />
            <button
                on:click={async () => {
                    await launchInstance(i.name);
                }}
                class="launchRow"><Play class="launch-icon" /> Play</button
            >
            <button
                on:load={async (e) => {
                    // @ts-ignore
                    mapKillButtons.set(i.name, e.target);
                }}
                style="display:none"
                on:click={async () => {
                    await killInstance(i.name);
                }}><Cross class="launch-icon" /> Stop</button
            >
            <button on:click={async () => {}} class="editButton launchRow"
                ><Pencil class="launch-icon" />Edit</button
            >
        </div>
        <br />
    {/each}
</div>
