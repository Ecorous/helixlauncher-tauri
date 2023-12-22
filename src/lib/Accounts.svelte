<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Check, Play } from "lucide-svelte";
    import { onMount } from "svelte";

    let accounts: [{ username: string; uuid: string }] = [
        { username: "TestAccount", uuid: "not a valid uuid :3" },
    ];

    onMount(async () => {
        accounts = await invoke("get_accounts");
    });

    async function selectUUID(uuid: string) {
        invoke("select_account", { uuid });
    }

    async function isAccountSelected(uuid: string): Promise<boolean> {
        return await invoke("is_account_selected", { uuid });
    }
</script>

<div>
    <p>accounts!</p>

    {#each accounts as a}
        <div class="account">
            {#await isAccountSelected(a.uuid) then bool}
                {#if bool}
                    <h3>
                        {a.username}<button
                            disabled
                            on:click={async () => {
                                await selectUUID(a.uuid);
                            }}
                            class="launchRow"><Check  />Selected</button
                        >
                    </h3>
                {:else}
                    <h3>
                        {a.username}<button
                            on:click={async () => {
                                await selectUUID(a.uuid);
                            }}
                            class="launchRow">Select</button
                        >
                    </h3>
                {/if}
            {/await}

            <p>UUID: {a.uuid}</p>
        </div>

        <br />
    {/each}
</div>
